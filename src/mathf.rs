use raylib::prelude::*;

pub fn _v_q_bezier(v0: Vector2, v1: Vector2, v2: Vector2, t: f32) -> Vector2 {
    v1 + (v0 - v1).scale_by((1.0-t).powf(2.0)) + (v2 - v1).scale_by(t * t)
}

pub fn _dir_q_bezier(v0: Vector2, v1: Vector2, v2: Vector2, t: f32) -> f32 {
    let dir = (v1 - v0).scale_by(2.0 * (1.0 - t)) + (v2 - v1).scale_by(2.0 * t);
    dir.y.atan2(dir.x)
}

pub fn q_bezier(v0: Vector2, v1: Vector2, v2: Vector2, t: f32) -> (Vector2, f32) {
    let dir = (v1 - v0).scale_by(2.0 * (1.0 - t)) + (v2 - v1).scale_by(2.0 * t);
    (v1 + (v0 - v1).scale_by((1.0-t).powf(2.0)) + (v2 - v1).scale_by(t * t), dir.y.atan2(dir.x))
}

pub fn calc_ik(l: f32, affector: Vector2, mut sign: f32) -> Vector2 {
    sign = sign.signum();

    let num = affector.x * affector.x + (affector.y * affector.y);
    let denom = 2.0 * l * (affector.x * affector.x + (affector.y * affector.y)).sqrt();
    let mut angle = (num/denom).acos();
    if angle.is_nan() {
        angle = 0.0;
    }

    let theta = sign * angle + affector.y.atan2(affector.x);
    let point = Vector2::new(theta.cos() * l, theta.sin() * l);
    
    return point;
}