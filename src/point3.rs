use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt::{Display};
use crate::vector3;

// A 3-dimensional point
#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    // The x coordinate
    pub x: f32,
    // The y coordinate
    pub y: f32,
    // The z coordinate
    pub z: f32,
}

//Functions specific to Point3
impl Point3 {

    // Add vector to point and return result
    pub fn Add(&self, _rhs: vector3::Vector3) -> Point3 {
        Point3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }

    // Add vector to point and assign result to point
    pub fn AddAssign(&mut self, _rhs: vector3::Vector3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }

    // Substract vector from point and return result
    pub fn Sub(&self, _rhs: vector3::Vector3) -> Point3 {
        Point3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }

    // Substract vector from point and assign result to point
    pub fn SubAssign(&mut self, _rhs: vector3::Vector3) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }

}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}