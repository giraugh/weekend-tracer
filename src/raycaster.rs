use std::io::{stderr, Write};

use crate::ray::Ray;
use crate::vec3::{vec3, Point3, Vec3};

pub struct Raycaster {
    // Image
    image_width: usize,
    aspect_ratio: f64,

    // Camera
    viewport_height: f64,
    focal_length: f64,
    origin: Point3,
}

impl Default for Raycaster {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,

            viewport_height: 2.0,
            focal_length: 1.0,
            origin: Point3::zero(),
        }
    }
}

impl Raycaster {
    pub fn render<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: Write,
    {
        // Calculate image and viewport sizes
        let (image_width, image_height) = (
            self.image_width,
            ((self.image_width as f64) / self.aspect_ratio) as usize,
        );
        let (viewport_width, viewport_height) = (
            self.viewport_height as f64,
            (self.viewport_height as f64) * self.aspect_ratio,
        );

        // Calculate util vectors
        let horizontal = vec3!(viewport_width, 0, 0);
        let vertical = vec3!(0, viewport_height, 0);
        let lower_left_corner =
            self.origin - horizontal / 2.0 - vertical / 2.0 - vec3!(0, 0, self.focal_length);

        // Write meta info
        writeln!(writer, "P3\n{}\n{}\n255", image_width, image_height)?;

        // Write pixel info
        for y in (0..image_height).rev() {
            // Show progress
            stderr().flush()?;
            eprint!("\rScanlines remaining: {}", y);

            // Render scanline
            for x in 0..self.image_width {
                // For now we make up some pixel data
                let u = (x as f64) / (image_width - 1) as f64;
                let v = (y as f64) / (image_height - 1) as f64;
                let ray = Ray::new(
                    self.origin,
                    lower_left_corner + u * horizontal + v * vertical - self.origin,
                );

                // Write rays debug color
                ray.debug_color().write_color(writer)?;
            }
        }

        Ok(())
    }
}

// fn from_aspect(width: f64, aspect_ratio: f64, transpose: bool) -> (usize, usize) {
//     (
//         width as usize,
//         if transpose {
//             (width / aspect_ratio) as usize
//         } else {
//             (aspect_ratio * width) as usize
//         },
//     )
// }
