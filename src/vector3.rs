use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display};
use crate::point3::Point3;

//A 3-dimensional vector
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    // The x coordinate
    pub x: f32,
    // The y coordinate
    pub y: f32,
    // The z coordinate
    pub z: f32,
}

// Vector addition
impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// Vector addition with Point3
impl Add<Point3> for Vector3 {
    type Output = Point3;
    fn add(self, _rhs: Point3) -> Point3 {
        Point3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// Vector self addition
impl AddAssign for Vector3 {
    fn add_assign(&mut self, _rhs: Vector3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

// Vector substraction
impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

// Vector substraction with Point3
impl Sub<Point3> for Vector3 {
    type Output = Vector3;
    fn sub(self, _rhs: Point3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

// Vector self substraction
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, _rhs: Vector3) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

// Vector multiplication
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

// Vector multiplication with float
impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

// Vector self multiplication
impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// Vector division
impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

// Vector division with float
impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

// Vector self division
impl DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// Functions specific to Vector3
impl Vector3 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn unit(&self) -> Vector3 {
        let length = self.length();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn unit_assign(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}