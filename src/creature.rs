// 4 limbs
// Body
// It moves faster when the legs are pracitcally in their correct places so kind of standing
/*
leg target
0   1
  C
3   2
*/
use raylib::prelude::*;

pub struct Creature {
    pub pos: Vector2,
    pub rot: f32,
    pub leg_length: f32,
    targets: [Vector2; 4],
    lerps: [Vector2; 4],
    legs: [Vector2; 4],
}

impl Creature {
    pub fn new(pos: Vector2, leg_length: f32, leg_offset: Vector2) -> Creature {
        Creature {
            pos: pos,
            rot: 0.0,
            leg_length: leg_length,
            targets: [ Vector2::zero(); 4],
            lerps: [ Vector2::zero(); 4],
            legs: [ Vector2::new(-leg_offset.x, leg_offset.y),
            leg_offset,
            Vector2::new(leg_offset.x, -leg_offset.y),
            -leg_offset ],
        }
    }

    //Implement movement that is rotated based on the creature direction
    pub fn add_force(&mut self, vel: Vector2) {
        self.pos += vel;
    }

    pub fn get_leg_target(&self, index: usize) -> Vector2 {
        self.targets[index]
    }

    pub fn get_leg_lerp(&self, index: usize) -> Vector2 {
        self.targets[index]
    }

    pub fn get_leg(&self, index: usize) -> Vector2 {
        self.targets[index]
    }
}