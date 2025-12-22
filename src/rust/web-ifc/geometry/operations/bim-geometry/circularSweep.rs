//! Circular sweep around a directrix.

use glam::DVec3;

use super::buffers::Buffers;
use super::geometry::Geometry;
use super::utils::sweep_circular;

#[derive(Clone, Debug, Default)]
pub struct CircularSweep {
    pub scaling: f64,
    pub closed: bool,
    pub profile_points: Vec<DVec3>,
    pub radius: f64,
    pub directrix: Vec<DVec3>,
    pub initial_directrix_normal: DVec3,
    pub rotate90: bool,
}

impl CircularSweep {
    pub fn set_values(
        &mut self,
        scaling: f64,
        closed: bool,
        profile_points: Vec<f64>,
        radius: f64,
        directrix: Vec<f64>,
        initial_directrix_normal: Vec<f64>,
        rotate90: bool,
    ) {
        self.profile_points.clear();
        for chunk in profile_points.chunks(3) {
            if chunk.len() == 3 {
                self.profile_points.push(DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
        self.directrix.clear();
        for chunk in directrix.chunks(3) {
            if chunk.len() == 3 {
                self.directrix.push(DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
        if initial_directrix_normal.len() == 3 {
            self.initial_directrix_normal = DVec3::new(
                initial_directrix_normal[0],
                initial_directrix_normal[1],
                initial_directrix_normal[2],
            );
        }
        self.scaling = scaling;
        self.closed = closed;
        self.radius = radius;
        self.rotate90 = rotate90;
    }

    pub fn get_buffers(&self) -> Buffers {
        let mut buffers = Buffers::default();
        let geom: Geometry = sweep_circular(
            self.scaling,
            self.closed,
            &self.profile_points,
            self.radius,
            &self.directrix,
            self.initial_directrix_normal,
            self.rotate90,
        );
        for r in 0..geom.num_faces {
            let f = geom.get_face(r as usize);
            buffers.add_tri(
                geom.get_point(f.i0 as usize),
                geom.get_point(f.i1 as usize),
                geom.get_point(f.i2 as usize),
            );
        }
        buffers
    }
}
