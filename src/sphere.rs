use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // Setup quadratic
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let hb = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = hb * hb - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let root = (-hb - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-hb + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        // Create hit
        let point = ray.at(root);
        Some(Hit::new(
            point,
            root,
            ray,
            (point - self.center) / self.radius,
        ))
    }
}
