use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

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

    let mut creature = creature::Creature::new(window_size / 2.0, 75.0, Vector2::new(5.0, 5.0));

    //The maximum leg distance
    let leg_max_length = 175.0;
    let mut foot_pos = creature.pos;

    let speed = 2.0;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        //--------
        //Logic
        //--------
        //Move the body
        if rl.is_key_down(KEY_W) {
            creature.pos.y -= speed;
        }
        if rl.is_key_down(KEY_S) {
            creature.pos.y += speed;
        }
        if rl.is_key_down(KEY_A) {
            creature.pos.x -= speed;
        }
        if rl.is_key_down(KEY_D) {
            creature.pos.x += speed;
        }

        //Calculate the leg
        let target_pos = rl.get_mouse_position();
        let dist_ratio;
        if creature.pos.distance_to(target_pos) > leg_max_length {
            dist_ratio = leg_max_length / creature.pos.distance_to(target_pos);
        }
        else {
            dist_ratio = 1.0;
        }
        let lerp_pos = (target_pos - creature.pos) * dist_ratio + creature.pos;
        foot_pos = foot_pos.lerp(lerp_pos, 0.1);
        let ankle_pos = creature.pos + IK::calc_ik(leg_max_length / 2.0, foot_pos - creature.pos, -1.0);


        //--------
        //Drawing
        //--------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        //Draw leg connections
        d.draw_line(creature.pos.x as i32, creature.pos.y as i32, ankle_pos.x as i32, ankle_pos.y as i32, Color::YELLOWGREEN);
        d.draw_line(ankle_pos.x as i32, ankle_pos.y as i32, foot_pos.x as i32, foot_pos.y as i32, Color::YELLOWGREEN);
        //The body and the leg
        d.draw_circle(creature.pos.x as i32, creature.pos.y as i32, 20.0, Color::WHITE);
        //Ankle pos.
        d.draw_circle(ankle_pos.x as i32, ankle_pos.y as i32, 5.0, Color::WHITE);
        //Leg circle
        d.draw_circle(foot_pos.x as i32, foot_pos.y as i32, 10.0, Color::WHITE);

        //Draw the target
        d.draw_circle(target_pos.x as i32, target_pos.y as i32, 5.0, Color::RED);
        //Draw the lerp
        d.draw_circle(lerp_pos.x as i32, foot_pos.y as i32, 5.0, Color::YELLOW);
        
        //Info
        d.draw_text("Red - target position - move it with your mouse", 0, 0, 20, Color::RED);
        d.draw_text("Yellow - current leg lerp position to the target", 0, 25, 20, Color::YELLOW);
        d.draw_text("White big circle - the body \nWhite small circles - leg parts", 0, 50, 20, Color::WHITE);

        d.draw_fps(0, 100);
    }
}