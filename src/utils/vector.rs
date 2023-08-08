use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::Sub;
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub magnitude: f64,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        let magnitude: f64 = f64::sqrt(x * x + y * y + z * z);
        Vector { x, magnitude, y, z }
    }

    pub fn dot_product(&self, other: Vector) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross_product(&self, other: Vector) -> Vector {
        return Vector::new(
            self.y * other.z - self.z * other.y,
            -self.x * other.z + self.z * other.x,
            self.x * other.y - self.y * other.x,
        );
    }

    pub fn mul_with_scalar(&self, scale: f64) -> Vector {
        return Vector::new(self.x * scale, self.y * scale, self.z * scale);
    }

    pub fn get_normal_vec(&self) -> Vector {
        return Vector {
            x: self.x / self.magnitude,
            y: self.y / self.magnitude,
            z: self.z / self.magnitude,
            magnitude: 1.0,
        };
    }
}
