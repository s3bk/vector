use std::fmt::{Write};
use crate::{Surface, Vector, Rgba8, PathStyle};
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
    type Style = PathStyle;
    
    fn new(size: Vector) -> Self {
        let mut w = String::with_capacity(1024);
        writeln!(w, "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\">", size.x(), size.y()).unwrap();
        Svg(w)
    }
    fn build_style(&mut self, style: PathStyle) -> Self::Style {
        style
    }
    fn draw_path(&mut self, path: Self::Outline, style: &Self::Style) {
        write!(self.0, "<path style=\"").unwrap();
        
        fn f(u: u8) -> f32 { u as f32 / 255. }
        if let Some((r, g, b, a)) = style.fill {
            write!(self.0, "fill: rgba({}, {}, {}, {}); ", r, g, b, f(a)).unwrap();
        }
        if let Some(((r, g, b, a), width)) = style.stroke {
            write!(self.0, "stroke: #{:02x}{:02x}{:02x}; stroke-width: {}; stroke-opacity: {}", r, g, b, width, f(a)).unwrap();
        }
        writeln!(self.0, "\" d=\"{:?}\" />", path.contours().iter().format(" ")).unwrap()
    }
}
