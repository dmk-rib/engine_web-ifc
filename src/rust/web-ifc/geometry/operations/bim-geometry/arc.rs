//! Arc curve generation.

use glam::DMat3;

use super::buffers::Buffers;
use super::curve::Curve;
use super::utils::{get_ellipse_curve, CONST_PI};

#[derive(Clone, Debug, Default)]
pub struct Arc {
    pub radius_x: f32,
    pub radius_y: f32,
    pub num_segments: i32,
    pub placement: DMat3,
    pub start_rad: f64,
    pub end_rad: f64,
    pub swap: bool,
    pub normal_to_center_ending: bool,
    pub points: Vec<glam::DVec3>,
}

impl Arc {
    pub fn set_values(
        &mut self,
        radius_x: f32,
        radius_y: f32,
        num_segments: i32,
        placement: Vec<f64>,
        start_rad: f64,
        end_rad: f64,
        swap: bool,
        normal_to_center_ending: bool,
    ) {
        self.radius_x = radius_x;
        self.radius_y = radius_y;
        self.num_segments = num_segments;
        self.start_rad = start_rad;
        self.end_rad = end_rad;
        self.swap = swap;
        self.normal_to_center_ending = normal_to_center_ending;

        if placement.len() != 9 {
            self.placement = DMat3::IDENTITY;
            return;
        }

        self.placement = DMat3::from_cols_array(&[
            placement[0], placement[1], placement[2],
            placement[3], placement[4], placement[5],
            placement[6], placement[7], placement[8],
        ]);
    }

    pub fn get_buffers(&mut self) -> Buffers {
        let mut buffers = Buffers::default();
        self.points.clear();
        let curve = get_ellipse_curve(
            self.radius_x,
            self.radius_y,
            self.num_segments,
            self.placement,
            self.start_rad,
            self.end_rad,
            self.swap,
            self.normal_to_center_ending,
        );
        for point in curve.points {
            buffers.add_point(point);
        }
        buffers
    }
}

impl Default for Arc {
    fn default() -> Self {
        Self {
            radius_x: 0.0,
            radius_y: 0.0,
            num_segments: 0,
            placement: DMat3::IDENTITY,
            start_rad: 0.0,
            end_rad: CONST_PI * 2.0,
            swap: true,
            normal_to_center_ending: false,
            points: Vec::new(),
        }
    }
}
