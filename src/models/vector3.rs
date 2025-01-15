#![allow(unused)]

use std::{fmt, ops};

/// Generic struct which can represent a Color, Position and a Vector among other things
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3<T>(pub [T; 3]);

impl<T> Vector3<T> {
    /// Generates a Vec3 from 3 separate values of the same type
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
    /// Generates a Vec3 From an array of length 3
    pub fn from_array(array: [T; 3]) -> Self {
        Self(array)
    }
}

impl<T> fmt::Display for Vector3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl<T> ops::Deref for Vector3<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Vector3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Vector3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self[0] = self[0] + rhs[0];
        self[1] = self[1] + rhs[1];
        self[2] = self[2] + rhs[2];
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul for Vector3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Mul<f64> for Vector3<f64> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] = self[0] * rhs[0];
        self[1] = self[1] * rhs[1];
        self[2] = self[2] * rhs[2];
    }
}

impl ops::MulAssign<f64> for Vector3<f64> {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] = self[0] * rhs;
        self[1] = self[1] * rhs;
        self[2] = self[2] * rhs;
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div for Vector3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vector3::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl ops::Div<f64> for Vector3<f64> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::DivAssign for Vector3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self[0] = self[0] / rhs[0];
        self[1] = self[1] / rhs[1];
        self[2] = self[2] / rhs[2];
    }
}

impl ops::DivAssign<f64> for Vector3<f64> {
    fn div_assign(&mut self, rhs: f64) {
        self[0] = self[0] / rhs;
        self[1] = self[1] / rhs;
        self[2] = self[2] / rhs;
    }
}
