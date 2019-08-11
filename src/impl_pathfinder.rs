use pathfinder_geometry::vector::Vector2F;
use pathfinder_content::{
    outline::{Contour as PaContour, Outline as PaOutline},
    stroke::{StrokeStyle, LineCap, LineJoin, OutlineStrokeToFill},
    color::ColorU
};
use pathfinder_renderer::{
    scene::{Scene, PathObject},
    paint::Paint
};
use crate::{Contour, Vector, Point, Outline};

impl Point for Vector2F {
    type Value = f32;
    
    fn new(x: f32, y: f32) -> Self {
        Vector2F::new(x, y)
    }
    fn x(&self) -> Self::Value {
        Vector2F::x(*self)
    }
    fn y(&self) -> Self::Value {
        Vector2F::y(*self)
    }
}

impl Contour for PaContour {
    type Point = Vector2F;
    fn new(start: Self::Point) -> Self {
        let mut contour = PaContour::new();
        contour.push_endpoint(start);
        contour
    }
    fn line_to(&mut self, p: Self::Point) {
        self.push_endpoint(p);
    }
    fn quadratic_curve_to(&mut self, c: Self::Point, p: Self::Point) {
        self.push_quadratic(c, p);
    }
    fn cubic_curve_to(&mut self, c0: Self::Point, c1: Self::Point, p: Self::Point) {
        self.push_cubic(c0, c1, p);
    }
    fn close(&mut self) {
        PaContour::close(self)
    }
}

impl Outline for PaOutline {
    type Point = Vector2F;
    type Contour = PaContour;
    
    fn empty() -> Self {
        PaOutline::new()
    }
    fn add_contour(&mut self, contour: Self::Contour) {
        self.push_contour(contour);
    }
}

impl Vector for Scene {
    type Value = f32;
    type Point = Vector2F;
    type Outline = PaOutline;
    type Color = Paint;
    type StrokeStyle = StrokeStyle;
    
    fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> Self::Color {
        Paint { color: ColorU { r, g, b, a } }
    }
    fn stoke(width: f32) -> Self::StrokeStyle {
        StrokeStyle {
            line_width: width,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter(width),
        }
    }
    fn draw_path(&mut self, path: Self::Outline, fill: Option<Self::Color>, stroke: Option<(Self::Color, Self::StrokeStyle)>) {
        if let Some((stroke, style)) = stroke {
            let paint = self.push_paint(&stroke);
            let outline = OutlineStrokeToFill::new(&path, style).into_outline();
            self.push_path(PathObject::new(outline, paint, String::new()));
        }
        if let Some(fill) = fill {
            let paint = self.push_paint(&fill);
            self.push_path(PathObject::new(path, paint, String::new()));
        }
    }
}
