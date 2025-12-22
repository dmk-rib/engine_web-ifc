//! Cylindrical revolution (sweep) helper.

use glam::DMat4;

use super::buffers::Buffers;
use super::geometry::Geometry;
use super::utils::revolve_cylinder;

#[derive(Clone, Debug, Default)]
pub struct CylindricalRevolution {
    pub num_rots: f64,
    pub transform: DMat4,
    pub start_degrees: f64,
    pub end_degrees: f64,
    pub min_z: f64,
    pub max_z: f64,
    pub radius: f64,
}

impl CylindricalRevolution {
    pub fn set_values(
        &mut self,
        transform: Vec<f64>,
        start_degrees: f64,
        end_degrees: f64,
        min_z: f64,
        max_z: f64,
        num_rots: f64,
        radius: f64,
    ) {
        if transform.len() == 16 {
            self.transform = DMat4::from_cols_array(&[
                transform[0], transform[1], transform[2], transform[3],
                transform[4], transform[5], transform[6], transform[7],
                transform[8], transform[9], transform[10], transform[11],
                transform[12], transform[13], transform[14], transform[15],
            ]);
        }
        self.start_degrees = start_degrees;
        self.end_degrees = end_degrees;
        self.max_z = max_z;
        self.min_z = min_z;
        self.radius = radius;
        self.num_rots = num_rots;
    }

    pub fn get_buffers(&self) -> Buffers {
        let mut buffers = Buffers::default();
        let geom: Geometry = revolve_cylinder(
            self.transform,
            self.start_degrees,
            self.end_degrees,
            self.min_z,
            self.max_z,
            self.num_rots as i32,
            self.radius,
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
