//! Minimal Rust port of bim-geometry curve types.

use glam::DVec3;

#[derive(Clone, Debug, Default)]
pub struct Curve {
    pub points: Vec<DVec3>,
}
