use indicatif::ProgressIterator;
use rand::random;
use std::io::Write;

use crate::camera::Camera;
use crate::hittable::World;
use crate::sphere::Sphere;
use crate::vec3::{vec3, Color, Vec3};

pub struct Raycaster {
    image_width: usize,
    samples_per_pixel: usize,
    max_bounces: usize,
    camera: Camera,
    world: World,
}

impl Default for Raycaster {
    fn default() -> Self {
        Self {
            image_width: 400,
            samples_per_pixel: 100,
            max_bounces: 50,
            camera: Camera::default(),
            world: vec![
                Box::new(Sphere::new(vec3!(0, 0, -1), 0.5)),
                Box::new(Sphere::new(vec3!(0, -100.5, -1), 100.0)),
            ],
        }
    }
}

impl Raycaster {
    pub fn render<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: Write,
    {
        // Calculate image sizes
        let (image_width, image_height) = self.camera.image_size(self.image_width);

        // Write meta info
        writeln!(writer, "P3\n{}\n{}\n255", image_width, image_height)?;

        // Write pixel info
        for y in (0..image_height).rev().progress() {
            // Render scanline
            for x in 0..self.image_width {
                // Sample several times
                let mut pixel_color = Color::zero();
                for _s in 0..self.samples_per_pixel {
                    let u = (x as f64 + random::<f64>()) / (image_width - 1) as f64;
                    let v = (y as f64 + random::<f64>()) / (image_height - 1) as f64;
                    let ray = self.camera.get_ray(u, v);
                    pixel_color += ray.color(&self.world, self.max_bounces);
                }

                // Write average color
                pixel_color.write_sampled_color(writer, self.samples_per_pixel)?;
            }
        }

        Ok(())
    }
}
