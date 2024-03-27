use std::fs::File;
use cairo::{Context, ImageSurface, IoError};
use libnoise::{Generator, Source};

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
    // clear to white
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint()?;

    let scale = 3.5;
    let generator = Source::simplex(42).scale([scale, scale]);
    //let buf = NoiseBuffer::<3>::new([30, 20, 25], &generator);
    let grid_width = 64;
    let grid_height = 48;
    for gx in 0..grid_width {
        for gy in 0..grid_height {
            let x = gx as f64 * surface.width() as f64 / grid_width as f64;
            let y = gy as f64 * surface.height() as f64 / grid_height as f64;
            let value = generator.sample([x / surface.width() as f64, y / surface.height() as f64]) * 0.5 + 0.5;
            ctx.set_source_rgb(value, value, value);
            ctx.rectangle(x, y, surface.width() as f64 / grid_width as f64, surface.height() as f64 / grid_height as f64);
            ctx.fill()?;
        }
    }
    let mut file = File::create("output.png")?;
    surface.write_to_png(&mut file)?;
    Ok(())
}
