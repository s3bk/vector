use pathfinder_content::{
    outline::{Contour as PaContour, Outline as PaOutline},
    stroke::{StrokeStyle, LineCap, LineJoin, OutlineStrokeToFill},
    color::ColorU
};
use pathfinder_renderer::{
    scene::{Scene, PathObject},
    paint::{Paint, PaintId}
};
pub use pathfinder_geometry::rect::RectF;
use crate::{Contour, Vector, Surface, Outline, Transform, Rgba8, PathStyle};


impl Contour for PaContour {
    #[inline]
    fn new() -> Self {
        trace!("Contour::new()");
        PaContour::new()
    }
    #[inline]
    fn move_to(&mut self, p: Vector) {
        trace!("Contour::move_to({:?})", p);
        self.push_endpoint(p);
    }
    #[inline]
    fn line_to(&mut self, p: Vector) {
        trace!("Contour::line_to({:?})", p);
        self.push_endpoint(p);
    }
    #[inline]
    fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        trace!("Contour::quadratic_curve_to({:?}, {:?})", c, p);
        self.push_quadratic(c, p);
    }
    #[inline]
    fn cubic_curve_to(&mut self, c1: Vector, c2: Vector, p: Vector) {
        trace!("Contour::cubic_curve_to({:?}, {:?}, {:?})", c1, c2, p);
        self.push_cubic(c1, c2, p);
    }
    #[inline]
    fn close(&mut self) {
        trace!("Contour::close()");
        PaContour::close(self)
    }
    #[inline]
    fn is_empty(&self) -> bool {
        PaContour::is_empty(self)
    }
    #[inline]
    fn clear(&mut self) {
        PaContour::clear(self)
    }
}

impl Outline for PaOutline {
    type Contour = PaContour;
    
    #[inline]
    fn empty() -> Self {
        PaOutline::new()
    }
    #[inline]
    fn add_contour(&mut self, contour: Self::Contour) {
        self.push_contour(contour);
    }
    #[inline]
    fn add_outline(&mut self, outline: Self) {
        for contour in outline.contours() {
            self.push_contour(contour.clone());
        }
    }
    #[inline]
    fn transform(mut self, transform: Transform) -> Self {
        PaOutline::transform(&mut self, &transform);
        self
    }
    #[inline]
    fn clear(&mut self) {
        PaOutline::clear(self)
    }
}

pub struct Style {
    fill: Option<PaintId>,
    stroke: Option<(PaintId, StrokeStyle)>
}
fn paint((r, g, b, a): Rgba8) -> Paint {
    Paint { color: ColorU { r, g, b, a } }
}

impl Surface for Scene {
    type Outline = PaOutline;
    type Style = Style;
    
    #[inline]
    fn new(size: Vector) -> Self {
        let mut scene = Scene::new();
        scene.set_view_box(RectF::new(Vector::default(), size));
        scene
    }
    fn build_style(&mut self, style: PathStyle) -> Self::Style {
        Style {
            fill: style.fill.map(|color| self.push_paint(&paint(color))),
            stroke: style.stroke.map(|(color, width)| (
                self.push_paint(&paint(color)),
                StrokeStyle {
                    line_width: width,
                    line_cap: LineCap::Butt,
                    line_join: LineJoin::Miter(width),
                }
            ))
        }
    }
    fn draw_path(&mut self, path: Self::Outline, style: &Self::Style) {
        if let Some((paint, style)) = style.stroke {
            let outline = OutlineStrokeToFill::new(&path, style.clone()).into_outline();
            self.push_path(PathObject::new(outline, paint, String::new()));
        }
        if let Some(paint) = style.fill {
            self.push_path(PathObject::new(path, paint, String::new()));
        }
    }
}
