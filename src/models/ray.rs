use super::point::Point;
use super::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        Point(*self.origin + self.direction * t)
    }
}
