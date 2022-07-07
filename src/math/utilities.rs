const PI: f64 = std::f64::consts::PI;
const INF: u64 = std::u64::MAX;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}