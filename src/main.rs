use std::io::stdout;

mod hittable;
mod ray;
mod raycaster;
mod sphere;
mod vec3;

use raycaster::Raycaster;

fn main() -> Result<(), std::io::Error> {
    let raycaster = Raycaster::default();
    raycaster.render(&mut stdout())?;
    Ok(())
}
