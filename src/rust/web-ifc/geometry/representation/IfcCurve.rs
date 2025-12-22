//! Rust port of IfcCurve.

use glam::{DMat4, DVec2, DVec3};

use crate::web_ifc::geometry::operations::bim_geometry::curve::Curve;

#[derive(Clone, Debug, Default)]
pub struct IfcCurve {
    pub base: Curve,
    pub arc_segments: Vec<u32>,
    pub user_data: Vec<String>,
    pub indices: Vec<u16>,
    pub end_tangent: DVec3,
    pub segment_start_tangents: Vec<DVec3>,
}

impl IfcCurve {
    pub fn get_2d(&self, index: usize) -> DVec2 {
        let point = self.base.points.get(index).copied().unwrap_or(DVec3::ZERO);
        DVec2::new(point.x, point.y)
    }

    pub fn get_3d(&self, index: usize) -> DVec3 {
        self.base.points.get(index).copied().unwrap_or(DVec3::ZERO)
    }

    pub fn get_placement_at_distance(&self, _distance: f64, _mode: CurvePlacementMode) -> DMat4 {
        unimplemented!("IfcCurve::get_placement_at_distance not yet implemented");
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CurvePlacementMode {
    TangentAsZAxis,
    GlobalZAxis,
}
