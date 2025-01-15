use std::{fs::File, io::Write};

use super::color::RGBColor;

pub struct PPMFile {
    width: u16,
    height: u16,
    max_color: f64,
    handle: File,
}

impl PPMFile {
    pub fn new(
        width: u16,
        height: u16,
        max_color: f64,
        filename: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut handle = File::create(filename).unwrap();

        writeln!(handle, "P3").unwrap();
        writeln!(handle, "{} {}", width, height).unwrap();
        writeln!(handle, "{}", max_color as u8).unwrap();

        Ok(Self {
            width,
            height,
            max_color,
            handle,
        })
    }

    pub fn write_color(&mut self, color: RGBColor) {
        writeln!(self.handle, "{}", *color).unwrap();
    }
}
