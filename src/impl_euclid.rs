use euclid::{Point2D, UnknownUnit};

use crate::{Point};

impl Point for Point2D<f32, UnknownUnit> {
    type Value = f32;
    fn new(x: f32, y: f32) -> Self {
        Point2D::new(x, y)
    }
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
}
