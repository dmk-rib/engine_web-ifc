//! Axis-aligned bounding box helpers.

use glam::DVec3;

use super::buffers::Buffers;
use super::epsilons::EPS_BIG;

#[derive(Clone, Debug)]
pub struct Aabb {
    pub index: u32,
    pub min: DVec3,
    pub max: DVec3,
    pub center: DVec3,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            index: 0,
            min: DVec3::new(f64::MAX, f64::MAX, f64::MAX),
            max: DVec3::new(-f64::MAX, -f64::MAX, -f64::MAX),
            center: DVec3::ZERO,
        }
    }
}

impl Aabb {
    pub fn intersects(&self, other: &Aabb) -> bool {
        let eps = EPS_BIG;
        self.max.x + eps >= other.min.x
            && other.max.x + eps >= self.min.x
            && self.max.y + eps >= other.min.y
            && other.max.y + eps >= self.min.y
            && self.max.z + eps >= other.min.z
            && other.max.z + eps >= self.min.z
    }

    pub fn contains(&self, pos: DVec3) -> bool {
        let eps = EPS_BIG;
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

    pub fn merge_point(&mut self, other: DVec3) {
        self.min = self.min.min(other);
        self.max = self.max.max(other);
    }

    pub fn intersect_ray(&self, origin: DVec3, dir: DVec3) -> bool {
        let dirfrac = DVec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);

        let t1 = (self.min.x - origin.x) * dirfrac.x;
        let t2 = (self.max.x - origin.x) * dirfrac.x;
        let t3 = (self.min.y - origin.y) * dirfrac.y;
        let t4 = (self.max.y - origin.y) * dirfrac.y;
        let t5 = (self.min.z - origin.z) * dirfrac.z;
        let t6 = (self.max.z - origin.z) * dirfrac.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < -EPS_BIG {
            return false;
        }

        if tmin > tmax + EPS_BIG {
            return false;
        }

        true
    }

    pub fn set_values(&mut self, min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) {
        self.min = DVec3::new(min_x, min_y, min_z);
        self.max = DVec3::new(max_x, max_y, max_z);
        self.center = DVec3::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0, (min_z + max_z) / 2.0);
    }

    pub fn get_buffers(&self) -> Buffers {
        let mut buffers = Buffers::default();

        buffers.add_point(DVec3::new(self.max.x, self.max.y, self.min.z));
        buffers.add_point(DVec3::new(self.max.x, self.min.y, self.min.z));
        buffers.add_point(DVec3::new(self.max.x, self.min.y, self.max.z));
        buffers.add_point(DVec3::new(self.max.x, self.max.y, self.max.z));
        buffers.add_point(DVec3::new(self.min.x, self.max.y, self.min.z));
        buffers.add_point(DVec3::new(self.min.x, self.min.y, self.min.z));
        buffers.add_point(DVec3::new(self.min.x, self.min.y, self.max.z));
        buffers.add_point(DVec3::new(self.min.x, self.max.y, self.max.z));

        buffers.add_tri_indices(0, 1, 3);
        buffers.add_tri_indices(3, 1, 2);
        buffers.add_tri_indices(5, 2, 1);
        buffers.add_tri_indices(2, 5, 6);
        buffers.add_tri_indices(7, 0, 4);
        buffers.add_tri_indices(3, 0, 7);
        buffers.add_tri_indices(7, 4, 5);
        buffers.add_tri_indices(5, 6, 7);
        buffers.add_tri_indices(6, 7, 3);
        buffers.add_tri_indices(6, 2, 3);
        buffers.add_tri_indices(5, 1, 4);
        buffers.add_tri_indices(1, 0, 4);

        buffers
    }
}

/// C++-style alias for API parity.
pub type AABB = Aabb;
