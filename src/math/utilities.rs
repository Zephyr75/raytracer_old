pub const PI: f32 = std::f32::consts::PI;
pub const INF: u32 = std::u32::MAX;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}