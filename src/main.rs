use std::fs::File;

use cairo::{Context, ImageSurface, IoError};

#[derive(Debug)]
enum MapsError {
    IoError(std::io::Error),
    CairoError(cairo::Error),
    CairoIoError(IoError),
}

impl From<std::io::Error> for MapsError {
    fn from(error: std::io::Error) -> Self { MapsError::IoError(error) }
}
impl From<cairo::Error> for MapsError {
    fn from(error: cairo::Error) -> Self { MapsError::CairoError(error) }
}
impl From<cairo::IoError> for MapsError {
    fn from(error: cairo::IoError) -> Self { MapsError::CairoIoError(error) }
}

fn main() -> Result<(), MapsError> {
    let surface = ImageSurface::create(cairo::Format::Rgb24, 640, 480)?;
    let ctx = Context::new(&surface)?;
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint()?;
    let mut file = File::create("output.png")?;
    surface.write_to_png(&mut file)?;
    Ok(())
}
