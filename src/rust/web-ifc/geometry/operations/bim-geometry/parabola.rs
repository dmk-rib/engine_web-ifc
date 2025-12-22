//! Parabola curve generation.

use glam::{DVec2, DVec3};

use super::buffers::Buffers;
use super::utils::solve_parabola;

#[derive(Clone, Debug, Default)]
pub struct Parabola {
    pub segments: u16,
    pub start_point: DVec3,
    pub horizontal_length: f64,
    pub start_height: f64,
    pub start_gradient: f64,
    pub end_gradient: f64,
    pub points: Vec<DVec3>,
}

impl Parabola {
    pub fn set_values(
        &mut self,
        segments: u16,
        start_point_x: f64,
        start_point_y: f64,
        start_point_z: f64,
        horizontal_length: f64,
        start_height: f64,
        start_gradient: f64,
        end_gradient: f64,
    ) {
        self.segments = segments;
        self.start_point = DVec3::new(start_point_x, start_point_y, start_point_z);
        self.horizontal_length = horizontal_length;
        self.start_height = start_height;
        self.start_gradient = start_gradient;
        self.end_gradient = end_gradient;
    }

    pub fn get_buffers(&mut self) -> Buffers {
        let mut buffers = Buffers::default();
        self.points.clear();
        let points_2d = solve_parabola(
            self.segments,
            DVec2::new(self.start_point.x, self.start_point.y),
            self.horizontal_length,
            self.start_height,
            self.start_gradient,
            self.end_gradient,
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
