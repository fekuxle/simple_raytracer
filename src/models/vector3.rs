#![allow(unused)]

use std::{fmt, ops};

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

impl<T> ops::Deref for Vec3<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Vec3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts a normalized color vector to RGB
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
/// Converts an RGB color vector to normalized values
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

impl<T: ops::Add<Output=T> + Copy> ops::Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self[0] = self[0] + rhs[0];
        self[1] = self[1] + rhs[1];
        self[2] = self[2] + rhs[2];
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign for Vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] = self[0] * rhs[0];
        self[1] = self[1] * rhs[1];
        self[2] = self[2] * rhs[2];
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::DivAssign for Vec3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self[0] = self[0] / rhs[0];
        self[1] = self[1] / rhs[1];
        self[2] = self[2] / rhs[2];
    }
}