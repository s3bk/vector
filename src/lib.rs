#[macro_use] extern crate log;

use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

pub trait Value: Into<f32> + From<f32> + From<i16> + Copy + Sized + Add + Sub + Mul + Div {}
impl Value for f32 {}

pub use pathfinder_geometry::{
    vector::Vector2F as Vector,
    transform2d::Transform2F as Transform,
    rect::RectF as Rect,
};

pub trait Contour: Clone + Sized {
    fn new() -> Self;
    fn move_to(&mut self, p: Vector);
    fn line_to(&mut self, p: Vector);
    fn quadratic_curve_to(&mut self, c: Vector, p: Vector);
    fn cubic_curve_to(&mut self, c0: Vector, c1: Vector, p: Vector);
    fn arc(&mut self, transform: Transform, start_angle: f32, end_angle: f32, clockwise: bool);
    fn close(&mut self);
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

pub trait Outline: Clone + Sized {
    type Contour: Contour;
    
    fn empty() -> Self;
    fn add_contour(&mut self, contour: Self::Contour);
    fn add_outline(&mut self, outline: Self);
    fn bounding_box(&self) -> Option<Rect>;
    fn transform(self, transform: Transform) -> Self;
    fn clear(&mut self);
}

#[derive(Copy, Clone)]
enum PathState {
    // nothing has ben drawn yet. only move_to is valid
    Empty,

    // we have a starting point, but it is not connected to a previous path
    Start(Vector),

    // out starting point is the end of the last path
    End(Vector)
}

#[derive(Copy, Clone)]
pub struct PathBuilder<O: Outline> {
    outline: O,
    contour: O::Contour,
    state: PathState,
}
impl<O: Outline> PathBuilder<O> {
    #[inline]
    pub fn new() -> Self {
        PathBuilder {
            outline: O::empty(),
            contour: O::Contour::new(),
            state: PathState::Empty
        }
    }

    #[inline]
    fn start(&mut self) {
        match self.state {
            PathState::Empty => panic!("no starting point set. call move_to first"),
            PathState::Start(p) => {
                // copy the contour instead of allocating a new buffer with unknown size each time
                // that way we reuse one buffer for each contour (of unknown length) and only need one allocation per contour
                // (instead of growing and reallocating every contour a bunch of times)
                if !self.contour.is_empty() {
                    self.outline.add_contour(self.contour.clone());
                    self.contour.clear();
                }
                self.contour.move_to(p);
            }
            PathState::End(_) => {}
        }
    }

    #[inline]
    pub fn move_to(&mut self, p: Vector) {
        self.state = PathState::Start(p);
    }
    #[inline]
    pub fn line_to(&mut self, p: Vector) {
        self.start();
        self.contour.line_to(p);
        self.state = PathState::End(p);
    }
    #[inline]
    pub fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        self.start();
        self.contour.quadratic_curve_to(c, p);
        self.state = PathState::End(p);
    }
    #[inline]
    pub fn cubic_curve_to(&mut self, c1: Vector, c2: Vector, p: Vector) {
        self.start();
        self.contour.cubic_curve_to(c1, c2, p);
        self.state = PathState::End(p);
    }
    #[inline]
    pub fn rect(&mut self, rect: Rect) {
        self.move_to(rect.origin());
        self.line_to(rect.upper_right());
        self.line_to(rect.lower_right());
        self.line_to(rect.lower_left());
        self.close();
        self.state = PathState::End(rect.lower_left());
    }
    #[inline]
    pub fn circle(&mut self, center: Vector, radius: f32) {
        self.ellipse(center, Vector::splat(radius), 0.0);
    }
    #[inline]
    pub fn ellipse(&mut self, center: Vector, radius: Vector, phi: f32) {
        let transform = Transform::from_translation(center)
            * Transform::from_rotation(phi)
            * Transform::from_scale(radius);
        self.contour.arc(transform, 0.0, 2.0 * core::f32::consts::PI, false);
        self.contour.close();
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

        self.state = match self.state {
            PathState::End(p) => PathState::Start(p),
            s => s
        };

        outline
    }
    #[inline]
    pub fn clear(&mut self) {
        self.contour.clear();
        self.outline.clear();
        self.state = PathState::Empty;
    }

    #[inline]
    pub fn pos(&self) -> Option<Vector> {
        match self.state {
            PathState::Empty => None,
            PathState::Start(p) => Some(p),
            PathState::End(p) => Some(p)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FillRule {
    EvenOdd,
    NonZero
}

pub enum Paint<S: Surface> {
    Solid(Rgba8),
    Image(S::Image, Transform)
}
impl<S: Surface> Clone for Paint<S> {
    #[inline]
    fn clone(&self) -> Self {
        match *self {
            Paint::Solid(color) => Paint::Solid(color),
            Paint::Image(ref image, tr) => Paint::Image(image.clone(), tr)
        }
    }
}
impl<S: Surface> fmt::Debug for Paint<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Paint::Solid((r, g, b, a)) => write!(f, "Solid(rgba({}, {}, {}, {}))", r, g, b, a),
            Paint::Image(_, _) => write!(f, "Image")
        }
    }
}
impl<S: Surface> Paint<S> {
    #[inline]
    pub fn white() -> Self {
        Paint::Solid((255, 255, 255, 255))
    }
    #[inline]
    pub fn black() -> Self {
        Paint::Solid((0, 0, 0, 255))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineStyle {
    pub width: f32,
    pub cap: LineCap,
    pub join: LineJoin,
}
impl LineStyle {
    pub fn default(width: f32) -> Self {
        LineStyle {
            width,
            cap: LineCap::Butt,
            join: LineJoin::Miter(width)
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineCap {
    Butt,
    Square,
    Round,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineJoin {
    Miter(f32),
    Bevel,
    Round,
}

pub type Rgba8 = (u8, u8, u8, u8);
pub struct PathStyle<S: Surface> {
    pub fill: Option<Paint<S>>,
    pub stroke: Option<(Paint<S>, LineStyle)>,
    pub fill_rule: FillRule
}
impl<S: Surface> Clone for PathStyle<S> {
    fn clone(&self) -> Self {
        PathStyle {
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            fill_rule: self.fill_rule
        }
    }
}
impl<S: Surface> fmt::Debug for PathStyle<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PathStyle")
            .field("fill", &self.fill)
            .field("stroke", &self.stroke)
            .field("fill_rule", &self.fill_rule)
            .finish()
    }
}
impl<S: Surface> PathStyle<S> {
    pub fn stroke(paint: Paint<S>, line: LineStyle) -> Self {
        PathStyle {
            fill: None,
            stroke: Some((paint, line)),
            fill_rule: FillRule::NonZero
        }
    }
    pub fn fill(paint: Paint<S>) -> Self {
        PathStyle {
            fill: Some(paint),
            stroke: None,
            fill_rule: FillRule::NonZero
        }
    }
}

pub enum PixelFormat {
    L8,
    Rgb24,
    Rgba32
}
pub trait Surface: Sized {
    type Outline: Outline;
    type Style: Clone;
    type ClipPath: Clone;
    type Image: Clone;
    
    fn new(size: Vector) -> Self;
    fn build_style(&mut self, style: PathStyle<Self>) -> Self::Style;
    fn draw_path(&mut self, path: Self::Outline, style: &Self::Style, clip: Option<&Self::ClipPath>);
    fn clip_path(&mut self, path: Self::Outline, fill_rule: FillRule) -> Self::ClipPath;
    fn texture(&mut self, width: u32, height: u32, data: &[u8], format: PixelFormat) -> Self::Image;
}

#[cfg(feature = "impl_raquote")]
mod impl_raqote;

#[cfg(feature = "impl_svg")]
mod impl_svg;

#[cfg(feature = "impl_pathfinder")]
mod impl_pathfinder;

#[cfg(feature = "impl_svg")]
pub use impl_svg::Svg;
