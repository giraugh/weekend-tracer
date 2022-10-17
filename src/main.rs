use std::io::{stderr, stdout, Write};

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
                let r = (x as f64) / (self.width - 1) as f64;
                let g = (y as f64) / (self.width - 1) as f64;
                let b = 0.25;

                // Write this pixels colour
                writeln!(
                    writer,
                    "{} {} {}",
                    (255.99 * r) as usize,
                    (255.99 * g) as usize,
                    (255.99 * b) as usize
                )?;
            }
        }

        Ok(())
    }
}
