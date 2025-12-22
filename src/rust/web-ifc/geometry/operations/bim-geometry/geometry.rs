//! Minimal Rust port of bim-geometry geometry types.

use glam::{DMat4, DVec3};

#[derive(Clone, Debug, Default)]
pub struct Geometry {
    pub vertices: Vec<DVec3>,
    pub indices: Vec<u32>,
}

impl Geometry {
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn volume(&self, _transform: DMat4) -> f64 {
        unimplemented!("Geometry::volume not yet implemented");
    }
}
