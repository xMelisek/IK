use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use creature::*;

mod creature;

enum Layers {
    PhysicalLayer,
    VisualLayer,
}

fn main() {
    let window_size = Vector2::new(1280.0, 720.0);

    let (mut rl, thread) = raylib::init()
        .size(window_size.x as i32, window_size.y as i32)
        .title("Inverse Kinematics")
        .resizable()
        .vsync()
        .msaa_4x()
        .build();

    // rl.set_window_icon(image);

    let mut layer = Layers::VisualLayer;
    let body_tex = rl.load_texture(&thread, "assets/c_body.png").unwrap();
    let leg_tex = rl.load_texture(&thread, "assets/c_leg.png").unwrap();
    let mut creature = Creature::new(window_size / 2.0, 25.0, Vector2::new(15.0, 25.0), Vector2::new(0.0, -5.0), body_tex, leg_tex);

    let speed = 2.0;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        //--------
        //Logic
        //--------
        
        // Creature logic processing
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

        creature.add_force(Vector2::new(0.0, vel.y));
        creature.rot += vel.x;

        #[cfg(debug_assertions)]
        if rl.is_key_pressed(KEY_SPACE) {
            layer = if matches!(layer, Layers::VisualLayer) { Layers::PhysicalLayer } else { Layers::VisualLayer };
        }

        creature.process();

        //--------
        //Drawing
        //--------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        match layer {
            Layers::VisualLayer => {
                creature.draw(&mut d);
            },
            Layers::PhysicalLayer => {
                creature.draw_physics(&mut d);
            },
        }

        d.draw_text(&format!("Layer: {}", if matches!(layer, Layers::VisualLayer) { "Visual Layer" } else { "Physical Layer" }), 0, 50, 20, Color::GRAY);
        d.draw_fps(0, 0);
    }
}