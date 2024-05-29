// 4 limbs - Done
// Body - Done
// It moves faster when the legs are pracitcally in their correct places so kind of standing
// More natural leg placement
/*
leg target - Done
0   1
  C
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
        let mut targets = Creature::calc_leg_targets(leg_offset, 0.0);
        for i in 0..targets.len() {
            targets[i] += add_offset;
        }
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
        self.pos += vel.rotated(self.rot.to_radians());
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
        let ankle_pos = self.pos + calc_ik(self.leg_length, self.legs[leg_index] - self.pos, if leg_index == 0 || leg_index == 3 {-1.0} else {1.0});
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
            }, size / 2.0, dir.to_degrees(), Color::WHITE); 
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
        self.targets = Creature::calc_leg_targets(leg_offset, self.rot);
        self.leg_offset = leg_offset;
    }

    fn update_lerps(&mut self) {
        for i in 0..4 {
            if (self.pos + self.joints[i].rotated(self.rot.to_radians())).distance_to(self.lerps[i]) > self.leg_length * 2.1 {
                self.lerps[i] = self.pos + self.targets[i] + self.add_offset.rotated(self.rot.to_radians());
            }
        }
    }

    fn calc_leg_targets(leg_offset: Vector2, rot: f32) -> [ Vector2; 4 ]{
        [ -leg_offset.rotated(rot.to_radians()),
        Vector2::new(leg_offset.x, -leg_offset.y).rotated(rot.to_radians()),
        leg_offset.rotated(rot.to_radians()),
        Vector2::new(-leg_offset.x, leg_offset.y).rotated(rot.to_radians()) ]
    }
}