//! Rust port of `test/io_helpers.cpp`.

#![allow(dead_code)]

use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;

use glam::{DMat4, DVec2, DVec3, DVec4};

use crate::web_ifc::geometry::ifc_geometry_processor::IfcGeometryProcessor;
use crate::web_ifc::geometry::representation::geometry::{
    normalize_ifc, IfcAlignment, IfcComposedMesh, IfcCrossSections, IfcFlatMesh,
};
use crate::web_ifc::geometry::representation::ifc_curve::IfcCurve;
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;

pub mod io {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub id: i32,
        pub is_boundary: bool,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self {
                x,
                y,
                id: -1,
                is_boundary: false,
            }
        }

        pub fn from_vec2(p: DVec2) -> Self {
            Self::new(p.x, p.y)
        }

        pub fn as_vec2(&self) -> DVec2 {
            DVec2::new(self.x, self.y)
        }
    }

    impl Default for Point {
        fn default() -> Self {
            Self::new(0.0, 0.0)
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Triangle {
        pub a: Point,
        pub b: Point,
        pub c: Point,
        pub id: i32,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Edge {
        pub a: i32,
        pub b: i32,
    }

    #[derive(Clone, Debug)]
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

    #[derive(Clone, Debug)]
    pub struct SVGLineSet {
        pub lines: Vec<Vec<DVec2>>,
        pub color: String,
    }

    impl Default for SVGLineSet {
        fn default() -> Self {
            Self {
                lines: Vec::new(),
                color: "rgb(255,0,0)".to_string(),
            }
        }
    }

    impl SVGLineSet {
        pub fn get_bounds(&self, size: DVec2, offset: DVec2) -> Bounds {
            get_bounds(self.lines.clone(), size, offset)
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

    fn rescale_point(p: DVec2, bounds: Bounds, size: DVec2, offset: DVec2) -> DVec2 {
        DVec2::new(
            ((p.x - bounds.min.x) / (bounds.max.x - bounds.min.x)) * size.x + offset.x,
            ((p.y - bounds.min.y) / (bounds.max.y - bounds.min.y)) * size.y + offset.y,
        )
    }

    fn rescale_point_inverted(
        p: DVec2,
        size: DVec2,
        offset: DVec2,
        min: DVec2,
        max: DVec2,
    ) -> DVec2 {
        let width = max.x - min.x;
        let height = max.y - min.y;
        let max_size = width.max(height);
        if max_size <= f64::EPSILON {
            let center = offset + size * 0.5;
            return center;
        }

        DVec2::new(
            ((p.x - min.x) / max_size) * size.x + offset.x,
            (size.y - ((p.y - min.y) / max_size) * size.y) + offset.y,
        )
    }

    fn rescale(points: &[DVec2], size: DVec2, offset: DVec2) -> Vec<DVec2> {
        let mut min = DVec2::new(f64::MAX, f64::MAX);
        let mut max = DVec2::new(-f64::MAX, -f64::MAX);

        for pt in points {
            min = min.min(*pt);
            max = max.max(*pt);
        }

        points
            .iter()
            .map(|pt| rescale_point_inverted(*pt, size, offset, min, max))
            .collect()
    }

    pub fn write_file(filename: &str, data: &str) {
        if let Ok(mut file) = File::create(filename) {
            let _ = file.write_all(data.as_bytes());
        }
    }

    pub fn dump_svg_curve(points: Vec<DVec3>, filename: &str, indices: Vec<u32>) {
        let points_2d: Vec<DVec2> = points.iter().map(|pt| DVec2::new(pt.x, pt.z)).collect();
        write_file(filename, &make_svg_lines(points_2d, indices));
    }

    pub fn svg_make_line(a: DVec2, b: DVec2, svg: &mut String, col: &str) {
        let _ = write!(
            svg,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style = \"stroke:{};stroke-width:1\" />",
            a.x, a.y, b.x, b.y, col
        );
    }

    pub fn svg_lines_to_string(
        bounds: Bounds,
        size: DVec2,
        offset: DVec2,
        line_set: SVGLineSet,
        svg: &mut String,
    ) {
        for line in line_set.lines {
            if line.len() > 1 {
                for i in 1..line.len() {
                    let a = rescale_point(line[i], bounds.clone(), size, offset);
                    let b = rescale_point(line[i - 1], bounds.clone(), size, offset);
                    svg_make_line(a, b, svg, &line_set.color);
                }
            } else if let Some(point) = line.first() {
                let a = rescale_point(*point, bounds.clone(), size, offset);
                let _ = write!(
                    svg,
                    "<circle cx = \"{}\" cy = \"{}\" r = \"3\" style = \"stroke:rgb(0,0,255);stroke-width:2\" />",
                    a.x, a.y
                );
            }
        }
    }

    pub fn make_svg_drawing(drawing: SVGDrawing) -> String {
        let size = DVec2::new(2048.0, 2048.0);
        let offset = DVec2::new(5.0, 5.0);
        let bounds = drawing.get_bounds(size, offset);

        let mut svg = String::new();
        let _ = write!(
            svg,
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            size.x + offset.x * 2.0,
            size.y + offset.y * 2.0
        );

        for set in drawing.sets {
            svg_lines_to_string(bounds.clone(), size, offset, set, &mut svg);
        }

        svg.push_str("</svg>");
        svg
    }

    pub fn to_obj_geometry(
        geom: &IfcGeometry,
        offset: &mut usize,
        transform: DMat4,
        input_scale: f64,
    ) -> String {
        let mut obj = String::new();
        let scale = input_scale;

        for i in 0..geom.base.base.num_points as usize {
            let point = geom.base.base.get_point(i);
            let t = transform * DVec4::new(point.x, point.y, point.z, 1.0);
            let _ = writeln!(obj, "v {} {} {}", t.x * scale, t.y * scale, t.z * scale);
        }

        for i in 0..geom.base.base.num_faces as usize {
            let face = geom.base.base.get_face(i);
            let _ = writeln!(
                obj,
                "f {}// {}// {}//",
                face.i0 + 1 + *offset as u32,
                face.i1 + 1 + *offset as u32,
                face.i2 + 1 + *offset as u32
            );
        }

        *offset += geom.base.base.num_points as usize;
        obj
    }

    pub fn to_obj_mesh(
        mesh: &IfcComposedMesh,
        processor: &mut IfcGeometryProcessor,
        offset: &mut usize,
        mat: DMat4,
    ) -> String {
        let mut complete = String::new();
        let trans = mat * mesh.transformation;
        let geom = processor.get_geometry(mesh.express_id).clone();
        complete.push_str(&to_obj_geometry(&geom, offset, trans, 1.0));

        for child in &mesh.children {
            complete.push_str(&to_obj_mesh(child, processor, offset, trans));
        }

        complete
    }

    pub fn dump_gradient_curve(
        curves: Vec<IfcCurve>,
        curve: IfcCurve,
        filename_v: &str,
        filename_h: &str,
    ) {
        write_file(filename_v, &gradient_vertical_to_obj(&curves));
        write_file(filename_h, &gradient_horizontal_to_obj(&curve));
    }

    pub fn dump_section_curves(curves: Vec<IfcCurve>, filename: &str) {
        write_file(filename, &gradient_vertical_to_obj(&curves));
    }

    pub fn dump_alignment(align: Vec<IfcAlignment>, filename_v: &str, filename_h: &str) {
        write_file(filename_v, &v_alignment_to_obj(&align));
        write_file(filename_h, &h_alignment_to_obj(&align));
    }

    pub fn dump_cross_sections(cross_section: Vec<IfcCrossSections>, filename: &str) {
        write_file(filename, &cross_section_to_obj(&cross_section));
    }

    pub fn dump_ifc_geometry_to_path(geom: &IfcGeometry, path: &str, input_scale: f64) {
        let mut offset = 0usize;
        if let Ok(mut file) = File::create(path) {
            let data = to_obj_geometry(geom, &mut offset, DMat4::IDENTITY, input_scale);
            let _ = file.write_all(data.as_bytes());
        }
    }

    pub fn to_obj_geometry_default(
        geom: &IfcGeometry,
        offset: &mut usize,
        transform: DMat4,
    ) -> String {
        to_obj_geometry(geom, offset, transform, 1.0)
    }

    pub fn dump_ifc_geometry(geom: &IfcGeometry, filename: &str) {
        let mut offset = 0usize;
        write_file(
            filename,
            &to_obj_geometry_default(geom, &mut offset, DMat4::IDENTITY),
        );
    }

    pub fn dump_flat_mesh(
        mesh: &IfcFlatMesh,
        processor: &mut IfcGeometryProcessor,
        filename: &str,
    ) {
        let mut offset = 0usize;
        write_file(
            filename,
            &to_obj_flat_mesh(mesh, processor, &mut offset, DMat4::IDENTITY),
        );
    }

    pub fn to_obj_flat_mesh(
        mesh: &IfcFlatMesh,
        processor: &mut IfcGeometryProcessor,
        offset: &mut usize,
        mat: DMat4,
    ) -> String {
        let mut complete = String::new();
        for geom in &mesh.geometries {
            let flat_geom = processor.get_geometry(geom.geometry_express_id).clone();
            let trans = mat * geom.transformation;
            complete.push_str(&to_obj_geometry(&flat_geom, offset, trans, 1.0));
        }
        complete
    }

    pub fn dump_svg_lines(lines: Vec<Vec<DVec2>>, filename: &str) {
        write_file(filename, &make_svg_lines_list(lines));
    }

    pub fn make_svg_lines(input: Vec<DVec2>, indices: Vec<u32>) -> String {
        let size = DVec2::new(512.0, 512.0);
        let offset = DVec2::new(5.0, 5.0);
        let rescaled = rescale(&input, size, offset);

        let mut svg = String::new();
        let _ = write!(
            svg,
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" >",
            size.x + offset.x * 2.0,
            size.y + offset.y * 2.0
        );

        if !rescaled.is_empty() {
            for i in 1..2 {
                let start = rescaled[i - 1];
                let end = rescaled[i];
                let _ = write!(
                    svg,
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style = \"stroke:rgb(0,255,0);stroke-width:2\" />",
                    start.x, start.y, end.x, end.y
                );
            }

            for i in 2..rescaled.len().saturating_sub(1) {
                let start = rescaled[i - 1];
                let end = rescaled[i];
                let _ = write!(
                    svg,
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style = \"stroke:rgb(0,0,0);stroke-width:2\" />",
                    start.x, start.y, end.x, end.y
                );
            }

            for i in rescaled.len().saturating_sub(1)..rescaled.len() {
                let start = rescaled[i - 1];
                let end = rescaled[i];
                let _ = write!(
                    svg,
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style = \"stroke:rgb(255,0,0);stroke-width:2\" />",
                    start.x, start.y, end.x, end.y
                );
            }
        }

        for indices in indices.chunks(3) {
            if indices.len() < 3 {
                continue;
            }
            let a = rescaled[indices[0] as usize];
            let b = rescaled[indices[1] as usize];
            let c = rescaled[indices[2] as usize];
            let _ = write!(
                svg,
                "<polygon points=\"{},{} {},{} {},{}\" style=\"fill:gray; stroke:none; stroke - width:0\" />`;",
                a.x, a.y, b.x, b.y, c.x, c.y
            );
        }

        svg.push_str("</svg>");
        svg
    }

    pub fn make_svg_lines_list(lines: Vec<Vec<DVec2>>) -> String {
        let size = DVec2::new(2048.0, 2048.0);
        let offset = DVec2::new(5.0, 5.0);
        let bounds = get_bounds(lines.clone(), size, offset);

        let mut svg = String::new();
        let _ = write!(
            svg,
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            size.x + offset.x * 2.0,
            size.y + offset.y * 2.0
        );

        for line in lines {
            if line.len() > 1 {
                for i in 1..line.len() {
                    let a = rescale_point(line[i], bounds.clone(), size, offset);
                    let b = rescale_point(line[i - 1], bounds.clone(), size, offset);
                    svg_make_line(a, b, &mut svg, "rgb(255,0,0)");
                }
            } else if let Some(point) = line.first() {
                let a = rescale_point(*point, bounds.clone(), size, offset);
                let _ = write!(
                    svg,
                    "<circle cx = \"{}\" cy = \"{}\" r = \"3\" style = \"stroke:rgb(0,0,255);stroke-width:2\" />",
                    a.x, a.y
                );
            }
        }

        svg.push_str("</svg>");
        svg
    }

    pub fn dump_mesh(mesh: &IfcComposedMesh, processor: &mut IfcGeometryProcessor, filename: &str) {
        let mut offset = 0usize;
        write_file(
            filename,
            &to_obj_mesh(mesh, processor, &mut offset, normalize_ifc()),
        );
    }

    pub fn gradient_vertical_to_obj(geom: &[IfcCurve]) -> String {
        let mut obj = String::new();

        for curve in geom {
            if !curve.base.points.is_empty() {
                for point in &curve.base.points {
                    let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                    let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                }
            }
        }

        let mut idx = 0u32;
        for curve in geom {
            if !curve.base.points.is_empty() {
                for _ in 0..curve.base.points.len().saturating_sub(1) {
                    let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                    idx += 1;
                }
            }
        }

        obj
    }

    pub fn gradient_horizontal_to_obj(geom: &IfcCurve) -> String {
        let mut obj = String::new();

        for point in &geom.base.points {
            let t = DVec4::new(point.x, point.y, 0.0, 1.0);
            let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
        }

        let mut idx = 0u32;
        for _ in 0..geom.base.points.len().saturating_sub(1) {
            let _ = writeln!(obj, "l {} {}", idx, idx + 1);
            idx += 1;
        }

        obj
    }

    pub fn v_alignment_to_obj(geom: &[IfcAlignment]) -> String {
        let mut obj = String::new();

        for alignment in geom {
            for curve in &alignment.vertical.curves {
                for point in &curve.base.points {
                    let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                    let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                }
            }
        }

        let mut idx = 0u32;
        for alignment in geom {
            if !alignment.vertical.curves.is_empty() {
                for curve in &alignment.vertical.curves {
                    if !curve.base.points.is_empty() {
                        for _ in 0..curve.base.points.len().saturating_sub(1) {
                            let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                            idx += 1;
                        }
                    }
                }
            }
        }

        obj
    }

    pub fn h_alignment_to_obj(geom: &[IfcAlignment]) -> String {
        let mut obj = String::new();

        for alignment in geom {
            for curve in &alignment.horizontal.curves {
                if !curve.base.points.is_empty() {
                    for point in &curve.base.points {
                        let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                        let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                    }
                }
            }
        }

        let mut idx = 0u32;
        for alignment in geom {
            for curve in &alignment.horizontal.curves {
                if !curve.base.points.is_empty() {
                    for _ in 0..curve.base.points.len().saturating_sub(1) {
                        let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                        idx += 1;
                    }
                }
            }
        }

        obj
    }

    pub fn cross_section_to_obj(geom: &[IfcCrossSections]) -> String {
        let mut obj = String::new();

        for section in geom {
            for curve in &section.curves {
                for point in &curve.base.points {
                    let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                    let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                }
            }
        }

        let mut idx = 0u32;
        for section in geom {
            for curve in &section.curves {
                for _ in 0..curve.base.points.len().saturating_sub(1) {
                    let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                    idx += 1;
                }
            }
        }

        obj
    }

    pub fn get_bounds(input: Vec<Vec<DVec2>>, _size: DVec2, _offset: DVec2) -> Bounds {
        let mut min = DVec2::new(f64::MAX, f64::MAX);
        let mut max = DVec2::new(-f64::MAX, -f64::MAX);

        for loop_points in input {
            for point in loop_points {
                min = min.min(point);
                max = max.max(point);
            }
        }

        Bounds { min, max }
    }
}

pub use io::*;
