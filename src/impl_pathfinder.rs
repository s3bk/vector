use pathfinder_content::{
    outline::{Contour as PaContour, Outline as PaOutline, ArcDirection},
    stroke::{StrokeStyle, OutlineStrokeToFill},
    fill::FillRule as PaFillRule,
    effects::BlendMode
};
use pathfinder_renderer::{
    scene::{Scene, DrawPath, ClipPath as PaClipPath, ClipPathId},
    paint::{Paint as PaPaint, PaintId as PaPaintId},
};
use pathfinder_color::ColorU;
use pathfinder_content::{
    gradient::{Gradient},
    pattern::{Pattern, Image, PatternSource, PatternFlags},
    stroke::{LineJoin as PaLineJoin, LineCap as PaLineCap}
};
use pathfinder_geometry::{
    rect::RectF,
    vector::Vector2I
};
use crate::{Contour, Vector, Surface, Outline, Transform, Paint, PathStyle, FillRule, PixelFormat, LineCap, LineJoin};
use std::sync::Arc;

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
    fn arc(&mut self, transform: Transform, start_angle: f32, end_angle: f32, clockwise: bool) {
        let direction = match clockwise {
            false => ArcDirection::CCW,
            true => ArcDirection::CW
        };
        self.push_arc(&transform, start_angle, end_angle, direction);
    }
    #[inline]
    fn close(&mut self) {
        trace!("Contour::close()");
        PaContour::close(self)
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
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
    fn bounding_box(&self) -> Option<RectF> {
        if self.contours().len() > 0 {
            Some(self.bounds())
        } else {
            None
        }
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

#[derive(Clone)]
pub struct Style {
    fill: Option<PaPaintId>,
    stroke: Option<(PaPaintId, StrokeStyle)>,
    fill_rule: PaFillRule,
}
#[inline]
fn paint(paint: Paint<Scene>) -> PaPaint {
    match paint {
        Paint::Solid((r, g, b, a)) => PaPaint::from_color(ColorU { r, g, b, a }),
        Paint::Image(image, tr) => {
            let mut pattern = Pattern::from_image(image);
            pattern.apply_transform(tr);
            PaPaint::from_pattern(pattern)
        }
    }
}
impl Into<PaFillRule> for FillRule {
    #[inline]
    fn into(self) -> PaFillRule {
        match self {
            FillRule::EvenOdd => PaFillRule::EvenOdd,
            FillRule::NonZero => PaFillRule::Winding,
        }
    }
}
impl Into<PaLineCap> for LineCap {
    #[inline]
    fn into(self) -> PaLineCap {
        match self {
            LineCap::Butt => PaLineCap::Butt,
            LineCap::Square => PaLineCap::Square,
            LineCap::Round => PaLineCap::Round,
        }
    }
}
impl Into<PaLineJoin> for LineJoin {
    #[inline]
    fn into(self) -> PaLineJoin {
        match self {
            LineJoin::Miter(w) => PaLineJoin::Miter(w),
            LineJoin::Bevel => PaLineJoin::Bevel,
            LineJoin::Round => PaLineJoin::Round,
        }
    }
}
impl Surface for Scene {
    type Outline = PaOutline;
    type Style = Style;
    type ClipPath = ClipPathId;
    type Image = Image;
    
    #[inline]
    fn new(size: Vector) -> Self {
        let mut scene = Scene::new();
        scene.set_view_box(RectF::new(Vector::default(), size));
        scene
    }
    fn build_style(&mut self, style: PathStyle<Self>) -> Self::Style {
        Style {
            fill: style.fill.map(|color| self.push_paint(&paint(color))),
            stroke: style.stroke.map(|(color, line)| (
                self.push_paint(&paint(color)),
                StrokeStyle {
                    line_width: line.width,
                    line_cap: line.cap.into(),
                    line_join: line.join.into(),
                }
            )),
            fill_rule: style.fill_rule.into()
        }
    }
    fn draw_path(&mut self, path: Self::Outline, style: &Self::Style, clip: Option<&Self::ClipPath>) {
        let stroke = style.stroke.map(|(paint, stroke_style)| {
            let mut stroke_to_fill = OutlineStrokeToFill::new(&path, stroke_style);
            stroke_to_fill.offset();
            let outline = stroke_to_fill.into_outline();
            let mut draw_path = DrawPath::new(outline, paint);
            draw_path.set_fill_rule(style.fill_rule);
            draw_path
        });
        if let Some(paint) = style.fill {
            let mut draw_path = DrawPath::new(path, paint);
            draw_path.set_fill_rule(style.fill_rule);
            self.push_path(draw_path);
        }
        if let Some(draw_path) = stroke {
            self.push_path(draw_path);
        }
    }
    #[inline]
    fn clip_path(&mut self, path: Self::Outline, fill_rule: FillRule) -> Self::ClipPath {
        let mut clip_path = PaClipPath::new(path);
        clip_path.set_fill_rule(fill_rule.into());
        self.push_clip_path(clip_path)
    }
    fn texture(&mut self, width: u32, height: u32, data: &[u8], format: PixelFormat) -> Self::Image {
        let data: Vec<ColorU> = match format {
            PixelFormat::L8 => data.iter().map(|&l| ColorU { r: l, g: l, b: l, a: 255 }).collect(),
            PixelFormat::Rgb24 => data.chunks(3).map(|c| ColorU { r: c[0], g: c[1], b: c[2], a: 255 }).collect(),
            PixelFormat::Rgba32 => data.chunks(4).map(|c| ColorU { r: c[0], g: c[1], b: c[2], a: c[3] }).collect(),
        };
        assert_eq!(data.len(), width as usize * height as usize);
        Image::new(Vector2I::new(width as i32, height as i32), Arc::new(data))
    }
}
