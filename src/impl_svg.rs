use std::fmt::Write;
use crate::{Contour, Vector, Point};

impl Contour for String {
    type Point = (f32, f32);
    fn new(start: Self::Point) -> Self {
        format!("M {} {}", start.x(), start.y())
    }
    fn line_to(&mut self, p: Self::Point) {
        write!(self, " L {} {}", p.x(), p.y());
    }
    fn quadratic_curve_to(&mut self, c: Self::Point, p: Self::Point) {
        write!(self, "Q {} {} {} {}", c.x(), c.y(), p.x(), p.y());
    }
    fn cubic_curve_to(&mut self, c0: Self::Point, c1: Self::Point, p: Self::Point) {
        write!(self, "C {} {} {} {} {} {}", c0.x(), c0.y(), c1.x(), c1.y(), p.x(), p.y());
    }
    fn close(&mut self) {
        write!(self, " Z");
    }
}

pub struct Svg<W: Write>(W);
impl<W: Write> Svg<W> {
    pub fn new(mut w: W) -> Svg<W> {
        writeln!(w, "<svg>");
        Svg(w)
    }
    pub fn finish(mut self) -> W {
        writeln!(self.0, "</svg>");
        self.0
    }
}
impl<W: Write> Vector for Svg<W> {
    type Value = f32;
    type Point = (f32, f32);
    type Outline = Vec<String>;
    type Color = (u8, u8, u8, u8);
    type StrokeStyle = f32;
    
    fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8) {
        (r, g, b, a)
    }
    fn stoke(width: Self::Value) -> Self::StrokeStyle {
        width
    }
    
    fn draw_path(&mut self, path: Self::Outline, fill: Option<Self::Color>, stroke: Option<(Self::Color, Self::StrokeStyle)>) {
        write!(self.0, "<path");
        
        fn f(u: u8) -> f32 { u as f32 / 255. }
        if let Some((r, g, b, a)) = fill {
            write!(self.0, " fill=\"rgba({} {} {} {})\"", f(r), f(g), f(b), f(a));
        }
        if let Some(((r, g, b, a), width)) = stroke {
            write!(self.0, " stroke=\"rgba({} {} {} {})\" stroke-width=\"{}\"", f(r), f(g), f(b), f(a), width);
        }
        write!(self.0, " d=\"");
        let mut contours = path.iter();
        if let Some(first) = contours.next() {
            write!(self.0, "{}", first);
        }
        for other in contours {
            write!(self.0, " {}", other);
        }
        writeln!(self.0, "\" />");
    }
}
