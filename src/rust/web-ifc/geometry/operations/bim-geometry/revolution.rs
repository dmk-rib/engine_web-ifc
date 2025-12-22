//! Revolution of a profile.

use glam::DMat4;

use super::buffers::Buffers;
use super::geometry::Geometry;
use super::utils::revolution;

#[derive(Clone, Debug, Default)]
pub struct Revolve {
    pub num_rots: f64,
    pub transform: DMat4,
    pub start_degrees: f64,
    pub end_degrees: f64,
    pub profile: Vec<glam::DVec3>,
}

impl Revolve {
    pub fn set_values(
        &mut self,
        profile: Vec<f64>,
        transform: Vec<f64>,
        start_degrees: f64,
        end_degrees: f64,
        num_rots: u32,
    ) {
        self.num_rots = num_rots as f64;
        self.profile.clear();
        for chunk in profile.chunks(3) {
            if chunk.len() == 3 {
                self.profile.push(glam::DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
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
    }

    pub fn get_buffers(&self) -> Buffers {
        let mut buffers = Buffers::default();
        let geom: Geometry = revolution(
            self.transform,
            self.start_degrees,
            self.end_degrees,
            self.profile.clone(),
            self.num_rots,
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
