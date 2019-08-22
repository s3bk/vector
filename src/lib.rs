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
    fn new() -> Self;
    fn move_to(&mut self, p: Vector);
    fn line_to(&mut self, p: Vector);
    fn quadratic_curve_to(&mut self, c: Vector, p: Vector);
    fn cubic_curve_to(&mut self, c0: Vector, c1: Vector, p: Vector);
    fn close(&mut self);
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

pub trait Outline: Clone + Sized {
    type Contour: Contour;
    
    fn empty() -> Self;
    fn add_contour(&mut self, contour: Self::Contour);
    fn add_outline(&mut self, outline: Self);
    fn transform(self, transform: Transform) -> Self;
    fn clear(&mut self);
}

pub struct PathBuilder<O: Outline> {
    outline: O,
    contour: O::Contour
}
impl<O: Outline> PathBuilder<O> {
    #[inline]
    pub fn new() -> Self {
        PathBuilder {
            outline: O::empty(),
            contour: O::Contour::new()
        }
    }
    #[inline]
    pub fn move_to(&mut self, p: Vector) {
        // copy the contour instead of allocating a new buffer with unknown size each time
        // that way we reuse one buffer for each contour (of unknown length) and only need one allocation per contour
        // (instead of growing and reallocating every contour a bunch of times)
        if !self.contour.is_empty() {
            self.outline.add_contour(self.contour.clone());
            self.contour.clear();
        }
        self.contour.move_to(p);
    }
    #[inline]
    pub fn line_to(&mut self, p: Vector) {
        self.contour.line_to(p);
    }
    #[inline]
    pub fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        self.contour.quadratic_curve_to(c, p);
    }
    #[inline]
    pub fn cubic_curve_to(&mut self, c1: Vector, c2: Vector, p: Vector) {
        self.contour.cubic_curve_to(c1, c2, p);
    }
    #[inline]
    pub fn close(&mut self) {
        self.contour.close();
    }
    #[inline]
    pub fn into_outline(mut self) -> O {
        if !self.contour.is_empty() {
            self.outline.add_contour(self.contour);
        }
        self.outline
    }
    #[inline]
    pub fn take(&mut self) -> O {
        if !self.contour.is_empty() {
            self.outline.add_contour(self.contour.clone());
            self.contour.clear();
        }
        
        let outline = self.outline.clone();
        self.outline.clear();
        outline
    }
    #[inline]
    pub fn clear(&mut self) {
        self.contour.clear();
        self.outline.clear();
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
