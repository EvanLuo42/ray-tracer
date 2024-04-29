use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }
    
    pub fn zero() -> Vec3 {
        Vec3::new(0f64, 0f64, 0f64)
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.x.powf(2f64) + self.y.powf(2f64) + self.z.powf(2f64))
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::maths::vec3::Vec3;

    #[test]
    fn eq() {
        let lhs = Vec3::new(1f64, 2f64, 3f64);
        let rhs = Vec3::new(1f64, 2f64, 3f64);
        assert!(lhs == lhs);
        assert!(rhs == rhs);
        assert!(lhs == rhs);
    }

    #[test]
    fn add() {
        let lhs = Vec3::new(1f64, 2f64, 3f64);
        let rhs = Vec3::new(1f64, 2f64, 3f64);
        let result = Vec3::new(2f64, 4f64, 6f64);
        assert!(lhs + rhs == result)
    }

    #[test]
    fn len() {
        let vec = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(vec.len(), f64::sqrt(14f64))
    }
}
