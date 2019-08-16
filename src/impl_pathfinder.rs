use pathfinder_content::{
    outline::{Contour as PaContour, Outline as PaOutline},
    stroke::{StrokeStyle, LineCap, LineJoin, OutlineStrokeToFill},
    color::ColorU
};
use pathfinder_renderer::{
    scene::{Scene, PathObject},
    paint::Paint
};
pub use pathfinder_geometry::rect::RectF;
use crate::{Contour, Vector, Surface, Outline, Transform};


impl Contour for PaContour {
    fn new(start: Vector) -> Self {
        let mut contour = PaContour::new();
        contour.push_endpoint(start);
        contour
    }
    fn line_to(&mut self, p: Vector) {
        self.push_endpoint(p);
    }
    fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        self.push_quadratic(c, p);
    }
    fn cubic_curve_to(&mut self, c0: Vector, c1: Vector, p: Vector) {
        self.push_cubic(c0, c1, p);
    }
    fn close(&mut self) {
        PaContour::close(self)
    }
}

impl Outline for PaOutline {
    type Contour = PaContour;
    
    fn empty() -> Self {
        PaOutline::new()
    }
    fn add_contour(&mut self, contour: Self::Contour) {
        self.push_contour(contour);
    }
    fn add_outline(&mut self, outline: Self) {
        for contour in outline.contours() {
            self.push_contour(contour.clone());
        }
    }
    fn transform(mut self, transform: Transform) -> Self {
        PaOutline::transform(&mut self, &transform);
        self
    }
}

impl Surface for Scene {
    type Outline = PaOutline;
    type Color = Paint;
    type StrokeStyle = StrokeStyle;
    
    fn new(size: Vector) -> Self {
        let mut scene = Scene::new();
        scene.set_view_box(RectF::new(Vector::default(), size));
        scene
    }
    fn color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> Self::Color {
        Paint { color: ColorU { r, g, b, a } }
    }
    fn stroke(&mut self, width: f32) -> Self::StrokeStyle {
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
