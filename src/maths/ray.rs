use crate::maths::Point3;
use crate::maths::vec3::Vec3;

pub struct Ray {
    origin: Point3,
     direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn at(&self, distance: f64) -> Point3 {
        self.origin +  distance * self.direction
    }
}