use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

impl Hit {
    pub fn new(point: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let is_front_face = ray.direction.dot(outward_normal) < 0.0;
        Self {
            point,
            normal: if is_front_face {
                outward_normal
            } else {
                -1.0 * outward_normal
            },
            t,
            is_front_face,
        }
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut current_hit = None;

        for hittable in self.iter() {
            if let Some(hit_record) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                current_hit = Some(hit_record);
            }
        }

        current_hit
    }
}

pub type World = Vec<Box<dyn Hittable>>;
