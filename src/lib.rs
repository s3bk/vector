extern crate raqote;

use std::ops::{Add, Sub, Mul, Div};

pub trait Value: Into<f32> + From<f32> + From<i16> + Copy + Sized + Add + Sub + Mul + Div {}
impl Value for f32 {}

pub use pathfinder_geometry::vector::Vector2F as Vector;
pub use pathfinder_geometry::transform2d::Transform2F as Transform;

pub trait Contour: Clone + Sized {
    fn new(start: Vector) -> Self;
    fn line_to(&mut self, p: Vector);
    fn quadratic_curve_to(&mut self, c: Vector, p: Vector);
    fn cubic_curve_to(&mut self, c0: Vector, c1: Vector, p: Vector);
    fn close(&mut self);
}

pub trait Outline: Clone + Sized {
    type Contour: Contour;
    
    fn empty() -> Self;
    fn add_contour(&mut self, contour: Self::Contour);
    fn add_outline(&mut self, outline: Self);
    fn transform(self, transform: Transform) -> Self;
}

pub struct PathBuilder<O: Outline> {
    outline: O,
    contour: Option<O::Contour>
}
impl<O: Outline> PathBuilder<O> {
    pub fn new() -> Self {
        PathBuilder {
            outline: O::empty(),
            contour: None
        }
    }
    pub fn move_to(&mut self, p: Vector) {
        if let Some(contour) = self.contour.replace(O::Contour::new(p)) {
            self.outline.add_contour(contour);
        }
    }
    pub fn line_to(&mut self, p: Vector) {
        self.contour.as_mut().expect("no current contour").line_to(p);
    }
    pub fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        self.contour.as_mut().expect("no current contour").quadratic_curve_to(c, p);
    }
    pub fn cubic_curve_to(&mut self, c1: Vector, c2: Vector, p: Vector) {
        self.contour.as_mut().expect("no current contour").cubic_curve_to(c1, c2, p);
    }
    pub fn close(&mut self) {
        if let Some(mut contour) = self.contour.take() {
            contour.close();
            self.outline.add_contour(contour);
        }
    }
    pub fn into_outline(mut self) -> O {
        if let Some(contour) = self.contour.take() {
            self.outline.add_contour(contour);
        }
        self.outline
    }   
}

pub trait Surface {
    type Outline: Outline;
    type Color;
    type StrokeStyle;
    
    fn new(size: Vector) -> Self;
    
    fn color_rgb(&mut self, r: u8, g: u8, b: u8) -> Self::Color {
        self.color_rgba(r, g, b, 255)
    }
    fn color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> Self::Color;
    fn stroke(&mut self, width: f32) -> Self::StrokeStyle;
    
    fn draw_path(&mut self, path: Self::Outline, fill: Option<Self::Color>, stroke: Option<(Self::Color, Self::StrokeStyle)>);
}

mod impl_raqote;
mod impl_svg;
mod impl_pathfinder;

pub use impl_svg::Svg;
