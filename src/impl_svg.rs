use std::fmt::{Write};
use crate::{Surface, Vector};
use itertools::Itertools;
use pathfinder_content::outline::{Outline as PaOutline};

pub struct Svg(String);
impl Svg {
    pub fn finish(mut self) -> String {
        writeln!(self.0, "</svg>").unwrap();
        self.0
    }
}

impl Surface for Svg {
    type Outline = PaOutline;
    type Color = (u8, u8, u8, u8);
    type StrokeStyle = f32;
    
    fn new(size: Vector) -> Self {
        let mut w = String::with_capacity(1024);
        writeln!(w, "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\">", size.x(), size.y()).unwrap();
        Svg(w)
    }
    fn color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8) {
        (r, g, b, a)
    }
    fn stroke(&mut self, width: f32) -> Self::StrokeStyle {
        width
    }
    
    fn draw_path(&mut self, path: Self::Outline, fill: Option<&Self::Color>, stroke: Option<(&Self::Color, &Self::StrokeStyle)>) {
        (|| {
            write!(self.0, "<path")?;
            
            fn f(u: u8) -> f32 { u as f32 / 255. }
            if let Some(&(r, g, b, a)) = fill {
                write!(self.0, " fill=\"rgba({}, {}, {}, {})\"", r, g, b, f(a))?;
            }
            if let Some((&(r, g, b, a), &width)) = stroke {
                write!(self.0, " stroke=\"rgba({}, {}, {}, {})\" stroke-width=\"{}\"", r, g, b, f(a), width)?;
            }
            writeln!(self.0, " d=\"{:?}\" />", path.contours().iter().format(" "))
        })().unwrap()
    }
}
