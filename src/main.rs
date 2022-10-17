use std::io::stdout;

mod ray;
mod raycaster;
mod vec3;

use raycaster::Raycaster;

fn main() -> Result<(), std::io::Error> {
    let raycaster = Raycaster::default();
    raycaster.render(&mut stdout())?;
    Ok(())
}
