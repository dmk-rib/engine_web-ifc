//! Clothoid curve generation.

use glam::{DVec2, DVec3};

use super::buffers::Buffers;
use super::utils::solve_clothoid;

#[derive(Clone, Debug, Default)]
pub struct Clothoid {
    pub segments: u16,
    pub start_point: DVec3,
    pub ifc_start_direction: f64,
    pub start_radius_of_curvature: f64,
    pub end_radius_of_curvature: f64,
    pub segment_length: f64,
    pub points: Vec<DVec3>,
}

impl Clothoid {
    pub fn set_values(
        &mut self,
        segments: u16,
        start_point_x: f64,
        start_point_y: f64,
        start_point_z: f64,
        ifc_start_direction: f64,
        start_radius_of_curvature: f64,
        end_radius_of_curvature: f64,
        segment_length: f64,
    ) {
        self.segments = segments;
        self.start_point = DVec3::new(start_point_x, start_point_y, start_point_z);
        self.ifc_start_direction = ifc_start_direction;
        self.start_radius_of_curvature = start_radius_of_curvature;
        self.end_radius_of_curvature = end_radius_of_curvature;
        self.segment_length = segment_length;
    }

    pub fn get_buffers(&mut self) -> Buffers {
        let mut buffers = Buffers::default();
        self.points.clear();
        let points_2d = solve_clothoid(
            self.segments,
            DVec2::new(self.start_point.x, self.start_point.y),
            self.ifc_start_direction,
            self.start_radius_of_curvature,
            self.end_radius_of_curvature,
            self.segment_length,
        );
        for pt in points_2d {
            self.points.push(DVec3::new(pt.x, pt.y, 0.0));
        }
        for point in &self.points {
            buffers.add_point(*point);
        }
        buffers
    }
}
