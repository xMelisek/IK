// It moves faster when the legs are pracitcally in their correct places so kind of standing
// More natural leg placement
// Velocity-based target placement
/*
0-4 - leg indexes
B - Body
0   1
  B
3   2
*/
use raylib::prelude::*;
use mathf::*;

#[path="mathf.rs"]
mod mathf;

const LEG_SEGMENTS : u32 = 8;

//Notes:
/*
pos is global
leg_offset and add_offset are local
targets and joints are also local
lerps and legs are global */

pub struct Creature {
    pub pos: Vector2,
    pub vel: Vector2,
    pub rot: f32,
    pub leg_length: f32,
    leg_offset: Vector2,
    add_offset: Vector2,
    targets: [Vector2; 4],
    lerps: [Vector2; 4],
    joints: [ Vector2; 4 ],
    legs: [Vector2; 4],
    pub body_tex: Texture2D,
    pub leg_tex: Texture2D,
}

impl Creature {
    pub fn new(pos: Vector2, leg_length: f32, leg_offset: Vector2, add_offset: Vector2, body_tex: Texture2D, leg_tex: Texture2D) -> Creature {
        let targets = Creature::calc_leg_targets(leg_offset, add_offset, 0.0);

        let mut lerps = [ Vector2::zero(); 4 ];
        for i in 0..targets.len() {
            lerps[i] = pos + targets[i];
        }

        let joints = [
            -Vector2::new(body_tex.width as f32 /2.0, body_tex.height as f32/2.0),
            Vector2::new(body_tex.width as f32 /2.0, -body_tex.height as f32/2.0),
            Vector2::new(body_tex.width as f32 /2.0, body_tex.height as f32/2.0),
            Vector2::new(-body_tex.width as f32 /2.0, body_tex.height as f32/2.0),
        ];
        
        Creature {
            pos: pos,
            vel: Vector2::zero(),
            rot: 0.0,
            leg_length: leg_length,
            leg_offset: leg_offset,
            add_offset: add_offset,
            targets: targets,
            lerps: lerps,
            joints: joints,
            legs: targets,
            body_tex: body_tex,
            leg_tex: leg_tex,
        }
    }

    pub fn add_force(&mut self, vel: Vector2) {
        self.vel += vel;
    }

    pub fn process(&mut self) {
        self.pos += self.vel.rotated(self.rot.to_radians());

        if self.vel == Vector2::zero() {
            // Set the leg targets to a default position
        }

        // Direction
        self.calc_legs();
        self.vel = Vector2::zero();
    }
    
    pub fn draw(&self, mut d: &mut RaylibDrawHandle) {
        // Draw legs
        for i in 0..4 {
            self.draw_leg(&mut d, i);
        }
        // Draw body
        d.draw_texture_pro(&self.body_tex, Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.body_tex.width as f32,
            height: self.body_tex.height as f32,
        }, Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.body_tex.width as f32,
            height: self.body_tex.height as f32,
        }, Vector2::new(self.body_tex.width as f32, self.body_tex.height as f32) / 2.0, self.rot, Color::WHITE);
    }

    #[allow(unused_mut)]
    #[cfg(debug_assertions)]
    pub fn draw_physics(&self, mut d: &mut RaylibDrawHandle) {
        // Draw leg targets
        for i in 0..4 {
            let points = self.get_points(i);

            d.draw_line(points[0].x as i32, points[0].y as i32, points[1].x as i32, points[1].y as i32, Color::YELLOWGREEN);
            d.draw_line(points[1].x as i32, points[1].y as i32, points[2].x as i32, points[2].y as i32, Color::YELLOWGREEN);
            d.draw_spline_bezier_quadratic(&points, 5.0, Color::BLUE);
            d.draw_circle(points[1].x as i32, points[1].y as i32, 4.0, Color::WHITE);
            d.draw_circle(points[2].x as i32, points[2].y as i32, 5.0, Color::WHITE);
        }
        for i in 0..4 {
            let pos = self.get_leg_target(i) + self.pos;
            let joint = self.pos + self.targets[i];
            d.draw_circle_lines(joint.x as i32, joint.y as i32, self.leg_length, Color::LIMEGREEN);
            d.draw_circle(pos.x as i32, pos.y as i32, 4.0, Color::RED);
        }
        for i in 0..4 {
            let pos = self.get_leg_lerp(i);
            d.draw_circle(pos.x as i32, pos.y as i32, 3.0, Color::YELLOW);
        }
        // Draw body
        d.draw_rectangle_pro(Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.body_tex.width as f32,
            height: self.body_tex.height as f32,
        }, Vector2::new(self.body_tex.width as f32/ 2.0, self.body_tex.height as f32 / 2.0), self.rot, Color::WHITE);
    }

    pub fn calc_legs(&mut self) {
        self.update_targets(self.leg_offset);
        self.update_lerps();
        for i in 0..4 {
            self.legs[i] = self.legs[i].lerp(self.lerps[i], 0.1);
        }
    }

    pub fn get_points(&self, leg_index: usize) -> [Vector2;3] {
        let joint_pos = self.joints[leg_index].rotated(self.rot.to_radians()) + self.pos;
        let ankle_pos = self.pos + calc_ik(self.leg_length / 2.0, self.legs[leg_index] - (self.pos + self.joints[leg_index]), if leg_index == 0 || leg_index == 3 {-1.0} else {1.0});
        let leg_pos = self.legs[leg_index];
        return [joint_pos, ankle_pos, leg_pos];
    }

    #[allow(unused_mut)]
    pub fn draw_leg(&self, mut d: &mut RaylibDrawHandle, index: usize) {
        let size = Vector2::new(self.leg_tex.width as f32, self.leg_tex.height as f32 / LEG_SEGMENTS as f32);
        for i in 0..LEG_SEGMENTS {
            let points = self.get_points(index);
            let offset = (self.leg_tex.height / LEG_SEGMENTS as i32) * i as i32;
            let (pos, dir) = q_bezier(points[0], points[1], points[2], LEG_SEGMENTS as f32 / self.leg_tex.height as f32 * i as f32);
            d.draw_texture_pro(&self.leg_tex, Rectangle {
                x: 0.0,
                y: offset as f32,
                width: size.x,
                height: size.y,
            }, Rectangle {
                x: pos.x,
                y: pos.y,
                width: size.x,
                height: size.y,
            }, size / 2.0, dir.to_degrees() - 90.0, Color::WHITE); 
        }
    }

    pub fn get_leg_target(&self, index: usize) -> Vector2 {
        self.targets[index]
    }

    pub fn get_leg_lerp(&self, index: usize) -> Vector2 {
        self.lerps[index]
    }

    pub fn get_leg(&self, index: usize) -> Vector2 {
        self.legs[index]
    }

    pub fn get_joint(&self, index: usize) -> Vector2 {
        self.joints[index]
    }

    pub fn update_targets(&mut self, leg_offset: Vector2) {
        self.targets = Creature::calc_leg_targets(leg_offset, self.add_offset, self.rot);
        self.leg_offset = leg_offset;
    }

    fn update_lerps(&mut self) {
        for i in 0..4 {
            if (self.pos + self.joints[i].rotated(self.rot.to_radians())).distance_to(self.lerps[i]) > self.leg_length {
                self.lerps[i] = self.pos + self.targets[i];
            }
        }
    }

    fn calc_leg_targets(leg_offset: Vector2, add_offset: Vector2, rot: f32) -> [ Vector2; 4 ]{
        [ (-leg_offset + add_offset).rotated(rot.to_radians()),
        (Vector2::new(leg_offset.x, -leg_offset.y) + add_offset).rotated(rot.to_radians()),
        (leg_offset + add_offset).rotated(rot.to_radians()),
        (Vector2::new(-leg_offset.x, leg_offset.y) + add_offset).rotated(rot.to_radians()) ]
    }
}