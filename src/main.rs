#![allow(dead_code, unused)]
use std::{error::Error, fs::File, io::Write};

mod models;
use models::{NormalizedColor, RGBColor, PPMFile};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u16 = 400;
const IMG_HEIGHT: u16 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u16;
const MAX_COLOR: f64 = 255.0;

fn main() -> Result<(), Box<dyn Error>> {
    // Create PPM file
    let mut file = PPMFile::new(IMG_WIDTH, IMG_HEIGHT, MAX_COLOR, "img.ppm")?;

    // Generate Pixel data
    for h in 0..IMG_HEIGHT {
        println!("Rendering Scanline {} out of {}", h, IMG_HEIGHT);
        for w in 0..IMG_WIDTH {
            let red = w as f64 / (IMG_WIDTH - 1) as f64;
            let green = h as f64 / (IMG_HEIGHT - 1) as f64;

            let color = RGBColor::new((red * MAX_COLOR) as u8, (green * MAX_COLOR) as u8, 0);
            file.write_color(color);
        }
    }

    Ok(())
}
