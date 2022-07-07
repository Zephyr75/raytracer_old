use crate::math::point3::Point3;
use crate::math::vector3::Vector3;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

//A 3D ray
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    // The 3D point representing the origin
    pub origin: Point3,
    // The 3D vector representing the direction
    pub direction: Vector3,
}

// Functions specific to Ray
impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Origin: ({}, {}, {}) / Direction: ({}, {}, {})",
            self.origin.x,
            self.origin.y,
            self.origin.z,
            self.direction.x,
            self.direction.y,
            self.direction.z
        )
    }
}
