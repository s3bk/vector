use std::fmt::{Write};
use crate::{Surface, Vector, Rgba8, PathStyle, FillRule};
use itertools::Itertools;
use pathfinder_content::outline::{Outline as PaOutline};

pub struct Svg(String, usize);
impl Svg {
    pub fn finish(mut self) -> String {
        writeln!(self.0, "</svg>").unwrap();
        self.0
    }
}

fn fill_rule_str(r: FillRule) -> &'static str {
    match r {
        FillRule::NonZero => "nonzero",
        FillRule::EvenOdd => "evenodd"
    }
}

impl Surface for Svg {
    type Outline = PaOutline;
    type Style = PathStyle;
    type ClipPath = usize;
    
    fn new(size: Vector) -> Self {
        let mut w = String::with_capacity(1024);
        writeln!(w, "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\">", size.x(), size.y()).unwrap();
        Svg(w, 0)
    }
    fn build_style(&mut self, style: PathStyle) -> Self::Style {
        style
    }
    fn draw_path(&mut self, path: Self::Outline, style: &Self::Style, clip: Option<&Self::ClipPath>) {
        write!(self.0, "<path style=\"").unwrap();
        
        fn f(u: u8) -> f32 { u as f32 / 255. }
        if let Some((r, g, b, a)) = style.fill {
            write!(self.0, "fill: #{:02x}{:02x}{:02x}; ", r, g, b).unwrap();
            if a != 255 {
                write!(self.0, "fill-opacity: {}", f(a)).unwrap();
            }
        } else {
            write!(self.0, "fill: none; ").unwrap();
        }
        if let Some(((r, g, b, a), width)) = style.stroke {
            write!(self.0, "stroke: #{:02x}{:02x}{:02x}; stroke-width: {}; ", r, g, b, width).unwrap();
            if a != 255 {
                write!(self.0, "stroke-opacity: {}", a as f32 / 255.).unwrap();
            }
        }
        write!(self.0, "\" fill-rule=\"{}\"", fill_rule_str(style.fill_rule)).unwrap();
        
        if let Some(&id) = clip {
            write!(self.0, " clip-path=\"clip_{}\"", id).unwrap();
        }

        writeln!(self.0, " d=\"{:?}\" />", path.contours().iter().format(" ")).unwrap()
    }
    fn clip_path(&mut self, path: Self::Outline, fill_rule: FillRule) -> Self::ClipPath {
        let id = self.1;
        self.1 += 1;

        writeln!(self.0,
            "<clipPath id=\"clip_{}\"><path clip-rule=\"{}\" d=\"{:?}\" /></clipPath>",
            id, fill_rule_str(fill_rule),
            path.contours().iter().format(" ")
        ).unwrap();
        id
    }
}
