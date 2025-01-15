#![allow(unused)]
use std::{error::Error, fs::File, io::Write};

mod models;
use models::{NormalizedColor, RGBColor};

const IMG_HEIGHT: u16 = 256;
const IMG_WIDTH: u16 = 256;
const MAX_COLOR: f64 = 255.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("img.ppm")?;

    // Generate PPM Header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMG_WIDTH, IMG_HEIGHT)?;
    writeln!(file, "{}", MAX_COLOR as u8)?;

    // Generate Pixel data
    for h in 0..IMG_HEIGHT {
        println!("Rendering Scanline {} out of {}", h, IMG_HEIGHT);
        for w in 0..IMG_WIDTH {
            let red = w as f64 / (IMG_WIDTH - 1) as f64;
            let green = h as f64 / (IMG_HEIGHT - 1) as f64;

            let color = RGBColor::new((red * MAX_COLOR) as u8, (green * MAX_COLOR) as u8, 0);
            writeln!(file, "{}", *color)?;
        }
    }

    Ok(())
}
