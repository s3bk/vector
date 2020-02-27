use crate::{Contour, Outline, Surface, Vector, Transform, Rgba8, PathStyle};
use raqote::{Point, Path, Winding, PathOp, DrawTarget, Source, SolidSource, StrokeStyle, DrawOptions};

fn point(v: Vector) -> Point {
    Point::new(v.x(), v.y())
}

impl Contour for Path {
    fn new() -> Self {
        Path {
            ops: vec![],
            winding: Winding::EvenOdd
        }
    }
    fn move_to(&mut self, p: Vector) {
        self.ops.push(PathOp::MoveTo(point(p)));
    }
    fn line_to(&mut self, p: Vector) {
        self.ops.push(PathOp::LineTo(point(p)));
    }
    fn quadratic_curve_to(&mut self, c: Vector, p: Vector) {
        self.ops.push(PathOp::QuadTo(point(c), point(p)));
    }
    fn cubic_curve_to(&mut self, c1: Vector, c2: Vector, p: Vector) {
        self.ops.push(PathOp::CubicTo(point(c1), point(c2), point(p)));
    }
    fn close(&mut self) {
        match self.ops.last() {
            Some(&PathOp::Close) | None => {},
            _ => self.ops.push(PathOp::Close)
        }
    }
    fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
    fn clear(&mut self) {
        self.ops.clear();
    }
}

impl Outline for Path {
    type Contour = Path;
    
    fn empty() -> Self {
        Path {
            ops: vec![],
            winding: Winding::EvenOdd
        }
    }
    fn bounding_box(&self) -> Option<Rect> {
        None // FIXME
    }
    fn add_contour(&mut self, contour: Self::Contour) {
        self.ops.extend_from_slice(&contour.ops);
    }
    fn add_outline(&mut self, outline: Self) {
        self.ops.extend_from_slice(&outline.ops);
    }
    fn transform(mut self, transform: Transform) -> Self {
        let tr = |p: Point| point(transform * Vector::new(p.x, p.y));
        
        for op in &mut self.ops {
            *op = match *op {
                PathOp::MoveTo(p) => PathOp::MoveTo(tr(p)),
                PathOp::LineTo(p) => PathOp::LineTo(tr(p)),
                PathOp::QuadTo(c, p) => PathOp::QuadTo(tr(c), tr(p)),
                PathOp::CubicTo(c1, c2, p) => PathOp::CubicTo(tr(c1), tr(c2), tr(p)),
                PathOp::Close => PathOp::Close
            }
        }
        self
    }
    fn clear(&mut self) {
        self.ops.clear();
    }
}

#[derive(Clone)]
pub struct Style {
    fill: Option<Source<'static>>,
    stroke: Option<(Source<'static>, StrokeStyle)>
}

fn solid((r, g, b, a): Rgba8) -> Source<'static> {
    Source::Solid(SolidSource { r, g, b, a })
}
impl Surface for DrawTarget {
    type Outline = Path;
    type Style = Style;
    type ClipPath = Path;
    
    fn new(size: Vector) -> Self {
        DrawTarget::new(size.x().ceil() as i32, size.y().ceil() as i32)
    }
    fn build_style(&mut self, style: PathStyle) -> Self::Style {
        Style {
            fill: style.fill.map(|color| solid(color)),
            stroke: style.stroke.map(|(color, width)| (
                solid(color),
                StrokeStyle {
                    width: width,
                    .. StrokeStyle::default()
                }
            ))
        }
    }
    
    fn draw_path(&mut self, path: Path, style: &Style, clip: Option<&Self::ClipPath>) {
        if let Some(path) = clip {
            self.push_clip(path);
        }
        if let Some(ref fill) = style.fill {
            self.fill(&path, fill, &DrawOptions::new());
        }
        if let Some((ref stroke, ref style)) = style.stroke {
            self.stroke(&path, stroke, style, &DrawOptions::new());
        }
        if clip.is_some() {
            self.pop_clip();
        }
    }

    fn clip_path(&mut self, path: Self::Outline, fill_rule: FillRule) -> Self::ClipPath {
        path
    }
}

