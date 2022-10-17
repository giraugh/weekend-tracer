use crate::ray::Ray;
use crate::vec3::{vec3, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vec3::zero();
        let horizontal = vec3!(viewport_width, 0, 0);
        let vertical = vec3!(0, viewport_height, 0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - vec3!(0, 0, focal_length);

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn image_size(&self, image_width: usize) -> (usize, usize) {
        (
            image_width,
            ((image_width as f64) / self.aspect_ratio) as usize,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(16.0 / 9.0, 2.0, 1.0)
    }
}
