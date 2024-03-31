use std::fs::File;
use cairo::{Context, ImageSurface, IoError};
use libnoise::{Generator, NoiseBuffer, Source};
use maps::grid::{Grid, Resolution};
use maps::marching_squares::find_contours;

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
    let resolution = Resolution {width: 640, height: 480};
    let surface = ImageSurface::create(cairo::Format::Rgb24, resolution.width as i32, resolution.height as i32)?;
    let ctx = Context::new(&surface)?;
    // clear to white
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint()?;

    let scale = 0.025;
    let generator = Source::simplex(42).scale([scale, scale]);
    
    let grid_resolution = Resolution { width: 64, height: 64};
    let buffer = NoiseBuffer::<2>::new(grid_resolution.slice(), &generator);
    let grid = Grid::from_buffer(grid_resolution, &buffer.buffer);
    // draw grid
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let value = grid[(x, y)] * 0.5 + 0.5;
            ctx.set_source_rgb(value, 0.0, value);
            ctx.rectangle(
                x as f64 * resolution.width as f64 / grid.width() as f64,
                y as f64 * resolution.height as f64 / grid.height() as f64,
                resolution.width as f64 / grid.width() as f64,
                resolution.height as f64 / grid.height() as f64,
            );
            ctx.fill()?;
        }
    }

    let contours = find_contours(&grid, 0.0);
    println!("{}", contours.len());
    ctx.set_line_width(1.0);
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    for contour in contours {
        //println!("  length {}", contour.len());
        for p in contour {
            ctx.line_to(
                p[0] * resolution.width as f64 / (grid.width() - 1) as f64,
                p[1] * resolution.height as f64 / (grid.height() - 1) as f64,
            );
        }
        ctx.stroke()?;
    }

    let mut file = File::create("output.png")?;
    surface.write_to_png(&mut file)?;
    Ok(())
}
