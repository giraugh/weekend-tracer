#![allow(dead_code, unused_macros)]

use crate::hittable::Hittable;
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

    pub fn color(&self, hittable: &dyn Hittable, depth: usize) -> Color {
        // End of depth?
        if depth == 0 {
            return Color::zero();
        }

        // hits world?
        if let Some(hit) = hittable.hit(self, 0.001, f64::INFINITY) {
            let target = hit.point + hit.normal + Vec3::random_unit();
            return 0.5 * Ray::new(hit.point, target - hit.point).color(hittable, depth - 1);
        }

        // Color with background
        let dir = self.direction.normalised();
        let t = 0.5 * (dir.y + 1.0);
        (1.0 - t) * Color::one() + t * color!(0.5, 0.7, 1.0)
    }
}
