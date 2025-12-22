//! Buffer helpers for simple triangle/vertex accumulation.

use glam::DVec3;

#[derive(Clone, Debug, Default)]
pub struct Buffers {
    pub fvertex_data: Vec<f32>,
    pub index_data: Vec<u32>,
}

impl Buffers {
    pub fn add_point(&mut self, pt: DVec3) {
        self.fvertex_data.push(pt.x as f32);
        self.fvertex_data.push(pt.y as f32);
        self.fvertex_data.push(pt.z as f32);
    }

    pub fn add_tri_indices(&mut self, p1: u32, p2: u32, p3: u32) {
        self.index_data.push(p1);
        self.index_data.push(p2);
        self.index_data.push(p3);
    }

    pub fn add_tri(&mut self, p1: DVec3, p2: DVec3, p3: DVec3) {
        self.add_point(p1);
        self.add_point(p2);
        self.add_point(p3);
        let id = (self.fvertex_data.len() / 3) as u32;
        self.add_tri_indices(id - 3, id - 2, id - 1);
    }
}
