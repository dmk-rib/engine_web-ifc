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
        let global_z = DVec3::new(0.0, 0.0, 1.0);
        let eps = 1e-6;

        if self.base.points.is_empty() {
            return DMat4::IDENTITY;
        }

        let mut pos = if self.base.points.len() == 1 {
            self.base.points[0]
        } else {
            DVec3::ZERO
        };
        let mut tan = DVec3::new(1.0, 0.0, 0.0);

        let mut total_distance = 0.0;
        let mut found = false;
        for i in 0..self.base.points.len().saturating_sub(1) {
            let seg_length = self.base.points[i].distance(self.base.points[i + 1]);
            if seg_length < eps {
                continue;
            }

            total_distance += seg_length;
            if total_distance >= _distance - eps {
                let seg_start = total_distance - seg_length;
                let factor = ((_distance - seg_start) / seg_length).clamp(0.0, 1.0);
                pos = self.base.points[i] * (1.0 - factor) + self.base.points[i + 1] * factor;
                tan = self.base.points[i + 1] - self.base.points[i];
                found = true;
                break;
            }
        }

        if !found && self.base.points.len() > 1 {
            pos = *self.base.points.last().unwrap();
            tan = self.base.points[self.base.points.len() - 1]
                - self.base.points[self.base.points.len() - 2];
        }

        if tan.length() > eps {
            tan = tan.normalize();
        } else {
            tan = DVec3::new(1.0, 0.0, 0.0);
        }

        let (mut vx, mut vy, mut vz);
        if _mode == CurvePlacementMode::GlobalZAxis {
            vx = tan;
            vz = global_z;

            vy = vz.cross(vx);
            if vy.length() < eps {
                vy = DVec3::new(0.0, 1.0, 0.0);
            }
            vy = vy.normalize();
            vx = vy.cross(vz).normalize();
        } else {
            vz = tan;
            vx = global_z.cross(vz);
            if vx.length() < eps {
                vx = DVec3::new(1.0, 0.0, 0.0);
            }
            vx = vx.normalize();
            vy = vz.cross(vx).normalize();
            vx = vy.cross(vz).normalize();
        }

        DMat4::from_cols(
            vx.extend(0.0),
            vy.extend(0.0),
            vz.extend(0.0),
            pos.extend(1.0),
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CurvePlacementMode {
    TangentAsZAxis,
    GlobalZAxis,
}
