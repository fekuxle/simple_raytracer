use std::ops;

use super::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub Vector3<f64>);

impl Point {
    #[allow(unused)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3::new(x, y, z))
    }
}

impl ops::Deref for Point {
    type Target = Vector3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
