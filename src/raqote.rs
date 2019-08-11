use raqote::{Path as RaPath, Point as RaPoint, Winding, PathOp, DrawTarget, Source, SolidSource, LineCap, LineJoin, StrokeStyle};


impl Outline for Path {
    type Point: RaPoint;
    fn new(start: Self::Point) -> Self {
        RaPath {
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

impl Path for RaPath {
    type Point: RaPoint;
    type Outline: Outline;
    
    fn empty() -> Self {
        RaPath {
            ops: vec![],
            winding: Winding::EvenOdd
        }
    }
    fn add_outline(&mut self, outline: Self::Outline) {
        self.ops.extend_from_slice(outline.ops);
    }
}

impl Vector for DrawTarget {
    type Value = f32;
    type Point = RaPoint;
    type Path = RaPath;
    type Color = Source;
    type StrokeStyle = StrokeStyle;
    
    fn color_rgb(r: u8, g: u8, b: u8) -> Self::Color {
        Source::Solid(SolidSource { r, g, b, 255 })
    }
    fn color_rgba(r: u8, g: u8, b: u8: a: u8) -> Self::Color {
        Source::Solid(SolidSource { r, g, b, a })
    }
    fn stoke(width: Self::V, color: Self::Color) -> Self::StrokeStyle {
        StrokeStyle {
            width,
            .. StrokeStyle::default()
        }
    }
    
    fn draw_path(&mut self, path: Self::Path, fill: Option<Self::Color>, stroke: Option<(Self::Color, Self::StrokeStyle)>) {
        if let Some(fill) = fill {
            self.fill(&path, &fill, &DrawOptions::new());
        }
        if let Some((stroke, style)) = stroke {
            self.stroke(&path, &stroke, &style, &DrawOptions::new());
        }
    }
}

