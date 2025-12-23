//! Rust port of `web-ifc/geometry/operations/boolean-utils/svg.h`.

use glam::DVec2;

use super::eps::MESSAGES;
use super::util::write_file;

#[derive(Clone, Debug, Default)]
pub struct Bounds {
    pub min: DVec2,
    pub max: DVec2,
}

impl Bounds {
    pub fn merge(&mut self, other: &Bounds) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

pub fn get_bounds(input: &[Vec<DVec2>], _size: DVec2, _offset: DVec2) -> Bounds {
    let mut min = DVec2::new(f64::MAX, f64::MAX);
    let mut max = DVec2::new(-f64::MAX, -f64::MAX);

    for loop_points in input {
        for point in loop_points {
            min = min.min(*point);
            max = max.max(*point);
        }
    }

    let width = max.x - min.x;
    let height = max.y - min.y;
    if width == 0.0 && height == 0.0 && MESSAGES {
        println!("asdf");
    }

    Bounds { min, max }
}

#[derive(Clone, Debug, Default)]
pub struct SVGLineSet {
    pub lines: Vec<Vec<DVec2>>,
    pub color: String,
}

impl SVGLineSet {
    pub fn get_bounds(&self, size: DVec2, offset: DVec2) -> Bounds {
        get_bounds(&self.lines, size, offset)
    }
}

#[derive(Clone, Debug, Default)]
pub struct SVGDrawing {
    pub sets: Vec<SVGLineSet>,
}

impl SVGDrawing {
    pub fn get_bounds(&self, size: DVec2, offset: DVec2) -> Bounds {
        let mut bounds = Bounds {
            min: DVec2::new(f64::MAX, f64::MAX),
            max: DVec2::new(-f64::MAX, -f64::MAX),
        };

        for set in &self.sets {
            bounds.merge(&set.get_bounds(size, offset));
        }

        bounds
    }
}

pub fn rescale(p: DVec2, bounds: &Bounds, size: DVec2, offset: DVec2) -> DVec2 {
    DVec2::new(
        ((p.x - bounds.min.x) / (bounds.max.x - bounds.min.x)) * size.x + offset.x,
        ((p.y - bounds.min.y) / (bounds.max.y - bounds.min.y)) * size.y + offset.y,
    )
}

pub fn svg_make_line(a: DVec2, b: DVec2, svg: &mut String, color: &str) {
    svg.push_str(&format!(
        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style = \"stroke:{};stroke-width:1\" />",
        a.x, a.y, b.x, b.y, color
    ));
}

pub fn svg_lines_to_string(
    bounds: &Bounds,
    size: DVec2,
    offset: DVec2,
    line_set: &SVGLineSet,
    svg: &mut String,
) {
    for line in &line_set.lines {
        if line.len() > 1 {
            for i in 1..line.len() {
                let a = rescale(line[i], bounds, size, offset);
                let b = rescale(line[i - 1], bounds, size, offset);
                svg_make_line(a, b, svg, &line_set.color);
            }
        } else if let Some(point) = line.first() {
            let a = rescale(*point, bounds, size, offset);
            svg.push_str(&format!(
                "<circle cx = \"{}\" cy = \"{}\" r = \"3\" style = \"stroke:rgb(0,0,255);stroke-width:2\" />",
                a.x, a.y
            ));
        }
    }
}

pub fn make_svg_lines(drawing: &SVGDrawing) -> String {
    let size = DVec2::new(2048.0, 2048.0);
    let offset = DVec2::new(5.0, 5.0);

    let bounds = drawing.get_bounds(size, offset);

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
        size.x + offset.x * 2.0,
        size.y + offset.y * 2.0
    ));

    for set in &drawing.sets {
        svg_lines_to_string(&bounds, size, offset, set, &mut svg);
    }

    svg.push_str("</svg>");
    svg
}

pub fn dump_svg_lines(lines: Vec<Vec<DVec2>>, filename: &str) {
    let set = SVGLineSet {
        lines,
        color: "rgb(255,0,0)".to_string(),
    };
    let drawing = SVGDrawing { sets: vec![set] };
    write_file(filename, &make_svg_lines(&drawing));
}
