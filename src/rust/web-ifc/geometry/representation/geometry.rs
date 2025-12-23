//! Rust port of geometry representation types.

use std::collections::HashMap;

use glam::{DMat4, DVec2, DVec3, DVec4};

use super::ifc_curve::IfcCurve;

pub const CONST_PI: f64 = 3.141592653589793238462643383279502884;

pub const EPS_SMALL: f64 = 1e-6;
pub const EPS_TINY: f64 = 1e-9;

pub fn horizontal_alignment_type() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("LINE", 1),
        ("CIRCULARARC", 2),
        ("CLOTHOID", 3),
        ("CUBICSPIRAL", 4),
        ("BIQUADRATICPARABOLA", 5),
        ("BLOSSCURVE", 6),
        ("COSINECURVE", 7),
        ("SINECURVE", 8),
        ("VIENNESEBEND", 9),
    ])
}

pub fn vertical_alignment_type() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("CONSTANTGRADIENT", 1),
        ("CIRCULARARC", 2),
        ("PARABOLICARC", 3),
        ("CLOTHOID", 4),
    ])
}

pub const EXTRUSION_DISTANCE_HALFSPACE_M: f64 = 100.0;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IfcTrimmingSelectType {
    TrimNone,
    TrimByPosition,
    TrimByParameter,
    TrimByLength,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrimSense {
    Same = 1,
    Reverse = 0,
}

#[derive(Clone, Debug)]
pub struct IfcTrimmingSelect {
    pub trim_type: IfcTrimmingSelectType,
    pub value: f64,
    pub pos: DVec2,
    pub pos3d: DVec3,
}

impl Default for IfcTrimmingSelect {
    fn default() -> Self {
        Self {
            trim_type: IfcTrimmingSelectType::TrimNone,
            value: 0.0,
            pos: DVec2::ZERO,
            pos3d: DVec3::ZERO,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IfcSegmentIndexSelect {
    pub r#type: String,
    pub indexs: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct IfcProfile {
    pub r#type: String,
    pub curve: IfcCurve,
    pub holes: Vec<IfcCurve>,
    pub is_convex: bool,
    pub is_composite: bool,
    pub profiles: Vec<IfcProfile>,
    pub tags: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct IfcAlignmentSegment {
    pub curves: Vec<IfcCurve>,
}

#[derive(Clone, Debug, Default)]
pub struct SweptDiskSolid {
    pub profiles: Vec<IfcProfile>,
    pub axis: Vec<IfcCurve>,
    pub profile_radius: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Cylinder {
    pub active: bool,
    pub radius: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BSpline {
    pub active: bool,
    pub u_degree: f64,
    pub v_degree: f64,
    pub closed_u: String,
    pub closed_v: String,
    pub curve_type: String,
    pub weights: Vec<Vec<f64>>,
    pub control_points: Vec<Vec<DVec3>>,
    pub u_multiplicity: Vec<u32>,
    pub v_multiplicity: Vec<u32>,
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
    pub weight_points: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, Default)]
pub struct Revolution {
    pub active: bool,
    pub direction: DMat4,
    pub profile: IfcProfile,
}

#[derive(Clone, Debug, Default)]
pub struct Extrusion {
    pub active: bool,
    pub direction: DVec3,
    pub profile: IfcProfile,
    pub length: f64,
}

#[derive(Clone, Debug, Default)]
pub struct IfcCrossSections {
    pub curves: Vec<IfcCurve>,
    pub express_id: Vec<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct IfcAlignment {
    pub horizontal: IfcAlignmentSegment,
    pub vertical: IfcAlignmentSegment,
    pub absolute: IfcAlignmentSegment,
    pub placement_express_id: u32,
}

impl IfcAlignment {
    pub fn transform(&mut self, coordination_matrix: DMat4) {
        for ic in 0..self.horizontal.curves.len() {
            if ic > 0 {
                let last_index = self.horizontal.curves[ic - 1]
                    .base
                    .points
                    .len()
                    .saturating_sub(1);
                let current_last_index = self.horizontal.curves[ic]
                    .base
                    .points
                    .len()
                    .saturating_sub(1);
                if !self.horizontal.curves[ic - 1].base.points.is_empty()
                    && !self.horizontal.curves[ic].base.points.is_empty()
                {
                    let d1 = self.horizontal.curves[ic].base.points[0]
                        .distance(self.horizontal.curves[ic - 1].base.points[last_index]);
                    let d2 = self.horizontal.curves[ic].base.points[current_last_index]
                        .distance(self.horizontal.curves[ic - 1].base.points[last_index]);
                    if d1 > d2 {
                        self.horizontal.curves[ic].base.points.reverse();
                    }
                }
            }
        }

        for curve in &mut self.horizontal.curves {
            for point in &mut curve.base.points {
                let transformed = coordination_matrix * DVec4::new(point.x, 0.0, -point.y, 1.0);
                *point = DVec3::new(transformed.x, -transformed.z, transformed.y);
            }
        }

        let y_offset = coordination_matrix.w_axis.y;
        for curve in &mut self.vertical.curves {
            for point in &mut curve.base.points {
                *point = DVec3::new(point.x, point.y + y_offset, 1.0);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IfcPlacedGeometry {
    pub color: DVec4,
    pub transformation: DMat4,
    pub flat_transformation: [f64; 16],
    pub geometry_express_id: u32,
}

impl IfcPlacedGeometry {
    pub fn set_flat_transformation(&mut self) {
        self.flat_transformation = flatten_transformation(self.transformation);
    }
}

#[derive(Clone, Debug, Default)]
pub struct IfcFlatMesh {
    pub geometries: Vec<IfcPlacedGeometry>,
    pub express_id: u32,
}

#[derive(Clone, Debug, Default)]
pub struct IfcComposedMesh {
    pub color: DVec4,
    pub transformation: DMat4,
    pub express_id: u32,
    pub has_geometry: bool,
    pub has_color: bool,
    pub children: Vec<IfcComposedMesh>,
}

impl IfcComposedMesh {
    pub fn get_color(&self) -> Option<DVec4> {
        if self.has_color {
            Some(self.color)
        } else {
            for child in &self.children {
                if let Some(color) = child.get_color() {
                    return Some(color);
                }
            }
            None
        }
    }
}

pub fn normalize_ifc() -> DMat4 {
    DMat4::from_cols(
        DVec4::new(1.0, 0.0, 0.0, 0.0),
        DVec4::new(0.0, 0.0, -1.0, 0.0),
        DVec4::new(0.0, 1.0, 0.0, 0.0),
        DVec4::new(0.0, 0.0, 0.0, 1.0),
    )
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IfcBoundType {
    OuterBound,
    Bound,
}

#[derive(Clone, Debug)]
pub struct IfcBound3D {
    pub bound_type: IfcBoundType,
    pub orientation: bool,
    pub curve: IfcCurve,
}

#[derive(Clone, Debug, Default)]
pub struct IfcSurface {
    pub transformation: DMat4,
    pub b_spline_surface: BSpline,
    pub cylinder_surface: Cylinder,
    pub revolution_surface: Revolution,
    pub extrusion_surface: Extrusion,
}

impl IfcSurface {
    pub fn normal(&self) -> DVec3 {
        if !self.cylinder_surface.active
            && !self.b_spline_surface.active
            && !self.revolution_surface.active
        {
            self.transformation.z_axis.truncate()
        } else {
            DVec3::ZERO
        }
    }
}

pub fn flatten_transformation(transformation: DMat4) -> [f64; 16] {
    let cols = transformation.to_cols_array();
    [
        cols[0], cols[1], cols[2], cols[3], cols[4], cols[5], cols[6], cols[7], cols[8], cols[9],
        cols[10], cols[11], cols[12], cols[13], cols[14], cols[15],
    ]
}
