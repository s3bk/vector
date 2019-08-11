extern crate euclid;
extern crate raqote;

trait Value: Into<f32> {}
impl Value for f32 {}

trait Point {
    type Value: Value;
    fn new(x: Self::Value, y: Self::Value) -> Self;
    fn x(&self) -> Self::Value;
    fn y(&self) -> Self::Value;
}
impl Point for (f32, f32) {
    type Value = f32;
    fn new(x: f32, y: f32) -> Self {
        (x, y)
    }
    fn x(&self) -> f32 {
        self.0
    }
    fn y(&self) -> f32 {
        self.1
    }
}

trait Contour {
    type Point: Point;
    fn new(start: Self::Point) -> Self;
    fn line_to(&mut self, p: Self::Point);
    fn quadratic_curve_to(&mut self, c: Self::Point, p: Self::Point);
    fn cubic_curve_to(&mut self, c0: Self::Point, c1: Self::Point, p: Self::Point);
    fn close(&mut self);
}

trait Outline {
    type Point: Point;
    type Contour: Contour<Point = Self::Point>;
    
    fn empty() -> Self;
    fn add_contour(&mut self, contour: Self::Contour);
}
impl<C: Contour> Outline for Vec<C> {
    type Point = C::Point;
    type Contour = C;
    
    fn empty() -> Self {
        vec![]
    }
    fn add_contour(&mut self, contour: C) {
        self.push(contour);
    }
}
trait Vector {
    type Value: Value;
    type Point: Point<Value=Self::Value>;
    type Outline: Outline<Point=Self::Point>;
    type Color;
    type StrokeStyle;
    
    fn color_rgb(r: u8, g: u8, b: u8) -> Self::Color {
        Self::color_rgba(r, g, b, 255)
    }
    fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> Self::Color;
    fn stoke(width: Self::Value) -> Self::StrokeStyle;
    
    fn draw_path(&mut self, path: Self::Outline, fill: Option<Self::Color>, stroke: Option<(Self::Color, Self::StrokeStyle)>);
}

mod impl_euclid;
mod impl_raqote;
mod impl_svg;
mod impl_pathfinder;

pub use impl_svg::Svg;
