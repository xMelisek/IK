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

    let mut layer = Layers::VisualLayer;
    let body_tex = rl.load_texture(&thread, "assets/c_body.png").unwrap();
    let leg_tex = rl.load_texture(&thread, "assets/c_leg.png").unwrap();
    let mut creature = Creature::new(window_size / 2.0, 35.0, Vector2::new(30.0, 45.0), Vector2::new(0.0, -25.0), body_tex, leg_tex);

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
            //Force update the lerps to reset the legs ?
        }

        creature.add_force(Vector2::new(0.0, vel.y));
        creature.rot += vel.x;

        if rl.is_key_pressed(KEY_SPACE) {
            layer = if matches!(layer, Layers::VisualLayer) { Layers::PhysicalLayer } else { Layers::VisualLayer };
            // creature.update_targets(Vector2::new(50.0, 50.0));
        }

        creature.calc_legs();

        //--------
        //Drawing
        //--------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        match layer {
            Layers::VisualLayer => {
                // Draw legs
                for i in 0..4 {
                    creature.draw_leg(&mut d, i);
                }
                // Draw body
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
            },
            Layers::PhysicalLayer => {
                // Draw leg targets
                for i in 0..4 {
                    let points = creature.get_points(i);

                    d.draw_line(points[0].x as i32, points[0].y as i32, points[1].x as i32, points[1].y as i32, Color::YELLOWGREEN);
                    d.draw_line(points[1].x as i32, points[1].y as i32, points[2].x as i32, points[2].y as i32, Color::YELLOWGREEN);
                    d.draw_spline_bezier_quadratic(&points, 5.0, Color::BLUE);
                    d.draw_circle(points[1].x as i32, points[1].y as i32, 4.0, Color::WHITE);
                    d.draw_circle(points[2].x as i32, points[2].y as i32, 5.0, Color::WHITE);
                }
                for i in 0..4 {
                    let pos = creature.get_leg_target(i) + creature.pos;
                    d.draw_circle(pos.x as i32, pos.y as i32, 4.0, Color::RED);
                }
                for i in 0..4 {
                    let pos = creature.get_leg_lerp(i);
                    d.draw_circle(pos.x as i32, pos.y as i32, 3.0, Color::YELLOW);
                }
                // Draw body
                d.draw_rectangle_pro(Rectangle {
                    x: creature.pos.x,
                    y: creature.pos.y,
                    width: creature.body_tex.width as f32,
                    height: creature.body_tex.height as f32,
                }, Vector2::new(creature.body_tex.width as f32/ 2.0, creature.body_tex.height as f32 / 2.0), creature.rot, Color::WHITE);
            },
        }

        d.draw_text(&format!("Layer: {}", if matches!(layer, Layers::VisualLayer) { "Visual Layer" } else { "Physical Layer" }), 0, 50, 20, Color::GRAY);
        d.draw_fps(0, 0);
    }
}