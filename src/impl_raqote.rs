use crate::{Vector, Contour, Outline};

use raqote::{Path, Point, Winding, PathOp, DrawTarget, Source, SolidSource, StrokeStyle, DrawOptions};


impl Contour for Path {
    type Point = Point;
    fn new(start: Self::Point) -> Self {
        Path {
            ops: vec![PathOp::MoveTo(start)],
            winding: Winding::EvenOdd
        }
    }
    fn line_to(&mut self, p: Self::Point) {
        self.ops.push(PathOp::LineTo(p));
    }
    fn quadratic_curve_to(&mut self, c: Self::Point, p: Self::Point) {
        self.ops.push(PathOp::QuadTo(c, p));
    }
    fn cubic_curve_to(&mut self, c0: Self::Point, c1: Self::Point, p: Self::Point) {
        self.ops.push(PathOp::CubicTo(c0, c1, p));
    }
    fn close(&mut self) {
        match self.ops.last() {
            Some(&PathOp::Close) | None => {},
            _ => self.ops.push(PathOp::Close)
        }
    }
}

impl Outline for Path {
    type Point = Point;
    type Contour = Path;
    
    fn empty() -> Self {
        Path {
            ops: vec![],
            winding: Winding::EvenOdd
        }
    }
    fn add_contour(&mut self, contour: Self::Contour) {
        self.ops.extend_from_slice(&contour.ops);
    }
}

impl Vector for DrawTarget {
    type Value = f32;
    type Point = Point;
    type Outline = Path;
    type Color = Source<'static>;
    type StrokeStyle = StrokeStyle;
    
    fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> Source<'static> {
        Source::Solid(SolidSource { r, g, b, a })
    }
    fn stoke(width: f32) -> StrokeStyle {
        StrokeStyle {
            width,
            .. StrokeStyle::default()
        }
    }
    
    fn draw_path(&mut self, path: Path, fill: Option<Source>, stroke: Option<(Source, StrokeStyle)>) {
        if let Some(fill) = fill {
            self.fill(&path, &fill, &DrawOptions::new());
        }
        if let Some((stroke, style)) = stroke {
            self.stroke(&path, &stroke, &style, &DrawOptions::new());
        }
    }
}

