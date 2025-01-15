#![allow(dead_code)]
use std::ops::{Deref, DerefMut};

use super::Vector3;

pub struct RGBColor(pub Vector3<u8>);

impl Deref for RGBColor {
    type Target = Vector3<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RGBColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RGBColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(Vector3::new(r, g, b))
    }
    /// Converts an RGB color vector to normalized values
    pub fn to_normalized(self) -> NormalizedColor {
        let [r, g, b] = *self.0;

        NormalizedColor(Vector3::new(
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
        ))
    }
}

pub struct NormalizedColor(pub Vector3<f64>);

impl Deref for NormalizedColor {
    type Target = Vector3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NormalizedColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NormalizedColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3::new(r, g, b))
    }
    /// Converts a normalized color vector to RGB
    pub fn to_rgb(self) -> RGBColor {
        let [r, g, b] = *self.0;

        RGBColor(Vector3::new(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        ))
    }
}