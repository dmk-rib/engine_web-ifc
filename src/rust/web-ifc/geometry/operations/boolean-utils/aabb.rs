//! Rust port of `web-ifc/geometry/operations/boolean-utils/aabb.h`.

use glam::DVec3;

use super::eps::{TOLERANCE_AABB, _TOLERANCE_BOUNDING_BOX};

pub type Vec = DVec3;

#[derive(Clone, Debug)]
pub struct Aabb {
    pub index: u32,
    pub min: Vec,
    pub max: Vec,
    pub center: Vec,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            index: 0,
            min: Vec::new(f64::MAX, f64::MAX, f64::MAX),
            max: Vec::new(-f64::MAX, -f64::MAX, -f64::MAX),
            center: Vec::ZERO,
        }
    }
}

impl Aabb {
    pub fn intersects(&self, other: &Aabb) -> bool {
        let eps = TOLERANCE_AABB;
        self.max.x + eps >= other.min.x
            && other.max.x + eps >= self.min.x
            && self.max.y + eps >= other.min.y
            && other.max.y + eps >= self.min.y
            && self.max.z + eps >= other.min.z
            && other.max.z + eps >= self.min.z
    }

    pub fn contains(&self, pos: &Vec) -> bool {
        let eps = TOLERANCE_AABB;
        pos.x + eps >= self.min.x
            && pos.x - eps <= self.max.x
            && pos.y + eps >= self.min.y
            && pos.y - eps <= self.max.y
            && pos.z + eps >= self.min.z
            && pos.z - eps <= self.max.z
    }

    pub fn merge(&mut self, other: &Aabb) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    pub fn merge_point(&mut self, other: &Vec) {
        self.min = self.min.min(*other);
        self.max = self.max.max(*other);
    }

    pub fn intersect(&self, origin: &Vec, dir: &Vec) -> bool {
        let dirfrac = Vec::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);

        let t1 = (self.min.x - origin.x) * dirfrac.x;
        let t2 = (self.max.x - origin.x) * dirfrac.x;
        let t3 = (self.min.y - origin.y) * dirfrac.y;
        let t4 = (self.max.y - origin.y) * dirfrac.y;
        let t5 = (self.min.z - origin.z) * dirfrac.z;
        let t6 = (self.max.z - origin.z) * dirfrac.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        let tolerance = unsafe { _TOLERANCE_BOUNDING_BOX };
        if tmax < -tolerance {
            return false;
        }

        if tmin > tmax + tolerance {
            return false;
        }

        true
    }
}

/// C++-style alias for API parity.
pub type AABB = Aabb;
