//! Minimal plane definition.

use glam::{DVec3, DVec4};

#[derive(Copy, Clone, Debug, Default)]
pub struct Plane {
    pub normal: DVec3,
    pub offset: f64,
}

impl Plane {
    pub fn from_vec4(data: DVec4) -> Self {
        Self {
            normal: data.truncate(),
            offset: data.w,
        }
    }
}
