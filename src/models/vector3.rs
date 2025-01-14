#![allow(unused)]

use std::{fmt, ops::Deref};

//use crate::MAX_COLOR;
pub const MAX_COLOR: f64 = 255.0;
/// Generic struct which can represent a Color, Position and a Vector among other things
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Vec3<T>(pub [T; 3]);

impl<T> Vec3<T> {
    /// Generates a Vec3 from 3 separate values of the same type
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
    /// Generates a Vec3 From an array of length 3
    pub fn from_array(array: [T; 3]) -> Self {
        Self(array)
    }
}

impl<T> fmt::Display for Vec3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl<T> Deref for Vec3<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Converts Normalized color to RGB
impl Vec3<f64> {
    pub fn to_rgb(self) -> Vec3<u8> {
        let [r, g, b] = self.0;

        return Vec3::new(
            (r * MAX_COLOR) as u8,
            (g * MAX_COLOR) as u8,
            (b * MAX_COLOR) as u8,
        );
    }
}
/// Converts RGB to normalized colors
impl Vec3<u8> {
    pub fn to_normalized(self) -> Vec3<f64> {
        let [r, g, b] = self.0;

        return Vec3::new(
            r as f64 / MAX_COLOR,
            g as f64 / MAX_COLOR,
            b as f64 / MAX_COLOR,
        );
    }
}
