use raylib::prelude::*;

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