use crate::maths::vec3::Vec3;

pub mod vec3;
pub mod ray;

pub type Point3 = Vec3;

pub trait Range {
    type Value;

    fn clamp(self, max: Self::Value, min: Self::Value) -> Self::Value;
    fn max(self, max: Self::Value) -> Self::Value;
    fn min(self, min: Self::Value) -> Self::Value;
    fn in_range(self, min: Self::Value, max: Self::Value) -> bool;
}

impl Range for f64 {
    type Value = f64;

    fn clamp(self, max: Self::Value, min: Self::Value) -> Self::Value {
        if self > max {
            return max
        } else if self < min {
            return min
        }
        self
    }

    fn max(self, max: Self::Value) -> Self::Value {
        self.clamp(self, max)
    }

    fn min(self, min: Self::Value) -> Self::Value {
        self.clamp(min, self)
    }

    fn in_range(self, min: Self::Value, max: Self::Value) -> bool {
        self > min && self < max
    }
}
