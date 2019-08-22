extern crate raqote;

use std::ops::{Add, Sub, Mul, Div};

pub trait Value: Into<f32> + From<f32> + From<i16> + Copy + Sized + Add + Sub + Mul + Div {}
impl Value for f32 {}

pub use pathfinder_geometry::{
    vector::Vector2F as Vector,
    transform2d::Transform2F as Transform,
    rect::RectF as Rect
};

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
    #[inline]
    pub fn new() -> Self {
        PathBuilder {
            outline: O::empty(),
            contour: None
        }
    }
    #[inline]
    pub fn move_to(&mut self, p: Vector) {
        if let Some(contour) = self.contour.replace(O::Contour::new(p)) {
            self.outline.add_contour(contour);
        }
    }
    #[inline]
    pub fn line_to(&mut self, p: Vector) {
        self.contour.as_mut().expect("no current contour").line_to(p);
    }
    #[inline]
    pub fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        self.contour.as_mut().expect("no current contour").quadratic_curve_to(c, p);
    }
    #[inline]
    pub fn cubic_curve_to(&mut self, c1: Vector, c2: Vector, p: Vector) {
        self.contour.as_mut().expect("no current contour").cubic_curve_to(c1, c2, p);
    }
    #[inline]
    pub fn close(&mut self) {
        if let Some(mut contour) = self.contour.take() {
            contour.close();
            self.outline.add_contour(contour);
        }
    }
    #[inline]
    pub fn into_outline(mut self) -> O {
        if let Some(contour) = self.contour.take() {
            self.outline.add_contour(contour);
        }
        self.outline
    }   
}

pub type Rgba8 = (u8, u8, u8, u8);
pub struct PathStyle {
    pub fill: Option<Rgba8>,
    pub stroke: Option<(Rgba8, f32)>
}

pub trait Surface {
    type Outline: Outline;
    type Style;
    
    fn new(size: Vector) -> Self;
    fn build_style(&mut self, style: PathStyle) -> Self::Style;
    fn draw_path(&mut self, path: Self::Outline, style: &Self::Style);
}

mod impl_raqote;
mod impl_svg;
mod impl_pathfinder;

pub use impl_svg::Svg;
