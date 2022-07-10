use rand::Rng;


pub const PI: f32 = std::f32::consts::PI;
pub const INF: u32 = std::u32::MAX;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

pub fn random_double() -> f32 {
    rand::thread_rng().gen_range(0.0 .. 1.0)
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min .. max)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}