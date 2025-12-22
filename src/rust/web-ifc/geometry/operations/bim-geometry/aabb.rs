//! Minimal AABB definition.

use glam::DVec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct AABB {
    pub min: DVec3,
    pub max: DVec3,
}

impl AABB {
    pub fn new(min: DVec3, max: DVec3) -> Self {
        Self { min, max }
    }
}
