use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use IK::calc_ik;

mod creature;
mod IK;

fn main() {
    let window_size = Vector2::new(1280.0, 720.0);

    let (mut rl, thread) = raylib::init()
        .size(window_size.x as i32, window_size.y as i32)
        .title("Inverse Kinematics")
        .resizable()
        .vsync()
        .msaa_4x()
        .build();

    let body_tex = rl.load_texture(&thread, "assets/c_body.png").unwrap();
    let mut creature = creature::Creature::new(window_size / 2.0, 35.0, Vector2::new(45.0, 25.0), Vector2::new(0.0, -50.0), body_tex);

    let speed = 2.0;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        //--------
        //Logic
        //--------
        //Move the body
        let mut vel = Vector2::zero();
        if rl.is_key_down(KEY_W) {
            vel.y -= speed;
        }
        if rl.is_key_down(KEY_S) {
            vel.y += speed;
        }
        if rl.is_key_down(KEY_A) {
            vel.x -= speed;
        }
        if rl.is_key_down(KEY_D) {
            vel.x += speed;
        }

        if vel == Vector2::zero() {
            //Force update the lerps to reset the legs
        }

        creature.add_force(Vector2::new(0.0, vel.y));
        creature.rot += vel.x;

        if rl.is_key_pressed(KEY_SPACE) {
            creature.update_targets(Vector2::new(50.0, 50.0));
        }

        creature.calc_legs();

        //--------
        //Drawing
        //--------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        //Draw spider body
        d.draw_texture_pro(&creature.body_tex, Rectangle {
            x: 0.0,
            y: 0.0,
            width: creature.body_tex.width as f32,
            height: creature.body_tex.height as f32,
        }, Rectangle {
            x: creature.pos.x,
            y: creature.pos.y,
            width: creature.body_tex.width as f32,
            height: creature.body_tex.height as f32,
        }, Vector2::new(creature.body_tex.width as f32, creature.body_tex.height as f32) / 2.0, creature.rot, Color::WHITE);
        //Draw leg targets
        for i in 0..4 {
            let pos = creature.get_leg(i);
            let joint = creature.pos + creature.get_joint(i).rotated(creature.rot.to_radians());
            let ankle = creature.pos + calc_ik(creature.leg_length, pos - creature.pos, if(i == 0 || i == 3) {-1.0} else {1.0});
            
            d.draw_line(joint.x as i32, joint.y as i32, ankle.x as i32, ankle.y as i32, Color::YELLOWGREEN);
            d.draw_line(ankle.x as i32, ankle.y as i32, pos.x as i32, pos.y as i32, Color::YELLOWGREEN);
            d.draw_circle(ankle.x as i32, ankle.y as i32, 4.0, Color::WHITE);
            d.draw_circle(pos.x as i32, pos.y as i32, 5.0, Color::WHITE);
        }
        // for i in 0..4 {
        //     let pos = creature.get_leg_target(i) + creature.pos;
        //     d.draw_circle(pos.x as i32, pos.y as i32, 4.0, Color::RED);
        // }
        // for i in 0..4 {
        //     let pos = creature.get_leg_lerp(i);
        //     d.draw_circle(pos.x as i32, pos.y as i32, 3.0, Color::YELLOW);
        // }
        d.draw_fps(0, 0);
    }
}