//! Boolean operations wrapper.

use super::buffers::Buffers;
use super::geometry::Geometry;
use super::boolean_utils::bool_process;

#[derive(Clone, Debug, Default)]
pub struct Boolean {
    pub r#type: i32,
    pub op: String,
    pub geometry: Geometry,
    pub seconds: Vec<Geometry>,
}

impl Boolean {
    pub fn get_buffers(&self) -> Buffers {
        let mut buffers = Buffers::default();
        let geom = bool_process(self.geometry.clone(), &mut self.seconds.clone(), &self.op);
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

    pub fn set_values(&mut self, triangles: Vec<f64>, op: String) {
        self.op = op;
        self.geometry = Geometry::default();
        let mut i = 0;
        while i + 8 < triangles.len() {
            self.geometry.add_face_points(
                glam::DVec3::new(triangles[i], triangles[i + 1], triangles[i + 2]),
                glam::DVec3::new(triangles[i + 3], triangles[i + 4], triangles[i + 5]),
                glam::DVec3::new(triangles[i + 6], triangles[i + 7], triangles[i + 8]),
                u32::MAX,
            );
            i += 9;
        }
        self.geometry.build_planes();
    }

    pub fn set_second(&mut self, triangles: Vec<f64>) {
        let mut new_geometry = Geometry::default();
        let mut i = 0;
        while i + 8 < triangles.len() {
            new_geometry.add_face_points(
                glam::DVec3::new(triangles[i], triangles[i + 1], triangles[i + 2]),
                glam::DVec3::new(triangles[i + 3], triangles[i + 4], triangles[i + 5]),
                glam::DVec3::new(triangles[i + 6], triangles[i + 7], triangles[i + 8]),
                u32::MAX,
            );
            i += 9;
        }
        new_geometry.build_planes();
        self.seconds.push(new_geometry);
    }

    pub fn clear(&mut self) {
        self.geometry = Geometry::default();
        self.seconds.clear();
        self.op.clear();
    }
}
