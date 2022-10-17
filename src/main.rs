use std::io::{stderr, stdout, Write};

mod vec3;
use crate::vec3::{Color, Vec3};

fn main() -> Result<(), std::io::Error> {
    let img = Image {
        width: 256,
        height: 256,
    };

    let mut out = stdout();
    img.write(&mut out)?;

    Ok(())
}

struct Image {
    width: usize,
    height: usize,
}

impl Image {
    fn write<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: Write,
    {
        // Write meta info
        writeln!(writer, "P3\n{}\n{}\n255", self.width, self.height)?;

        // Write pixel info
        for y in (0..self.height).rev() {
            // Show progress
            stderr().flush()?;
            eprint!("\rScanlines remaining: {}", y);

            // Render scanline
            for x in 0..self.width {
                // For now we make up some pixel data
                let color: Color = Color::new(
                    (x as f64) / (self.width - 1) as f64,
                    (y as f64) / (self.width - 1) as f64,
                    0.25,
                );

                // Write this pixels colour
                color.write_color(writer)?;
            }
        }

        Ok(())
    }
}
