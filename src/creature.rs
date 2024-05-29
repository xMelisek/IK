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
}

impl Creature {
    pub fn new(pos: Vector2, leg_length: f32, leg_offset: Vector2, add_offset: Vector2, body_tex: Texture2D) -> Creature {
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