use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt::{Display};
use crate::vector3::Vector3;

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

// Vector addition
impl Add<Point3> for Point3 {
    type Output = Vector3;
    fn add(self, _rhs: Point3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// Vector addition
impl Add<Vector3> for Point3 {
    type Output = Point3;
    fn add(self, _rhs: Vector3) -> Point3 {
        Point3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// Vector self addition
impl AddAssign<Vector3> for Point3 {
    fn add_assign(&mut self, _rhs: Vector3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

// Vector substraction
impl Sub<Point3> for Point3 {
    type Output = Vector3;
    fn sub(self, _rhs: Point3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

// Vector substraction
impl Sub<Vector3> for Point3 {
    type Output = Point3;
    fn sub(self, _rhs: Vector3) -> Point3 {
        Point3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

// Vector self substraction
impl SubAssign<Vector3> for Point3 {
    fn sub_assign(&mut self, _rhs: Vector3) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

// Functions specific to Point3
impl Point3 {

}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}