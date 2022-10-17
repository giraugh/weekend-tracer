#![allow(dead_code, unused_macros)]

use crate::vec3::{color, vec3, Color, Point3, Vec3};

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
        // hits sphere?
        if self.intersects_sphere(vec3!(0, 0, -1), 0.5) {
            return color!(1, 0, 0);
        }

        let dir = self.direction.normalised();
        let t = 0.5 * (dir.y + 1.0);
        (1.0 - t) * Color::one() + t * color!(0.5, 0.7, 1.0)
    }

    fn intersects_sphere(&self, sphere_center: Point3, sphere_radius: f64) -> bool {
        let oc = self.origin - sphere_center;
        let a = self.direction.square();
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.square() - sphere_radius * sphere_radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
