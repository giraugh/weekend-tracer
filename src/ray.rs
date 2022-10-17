#![allow(dead_code, unused_macros)]
use crate::vec3::{color, Color, Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn debug_color(&self) -> Color {
        let dir = self.direction.normalised();
        let t = 0.5 * (dir.y + 1.0);
        (1.0 - t) * Color::one() + t * color!(0.5, 0.7, 1.0)
    }
}
