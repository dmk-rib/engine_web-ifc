//! Plane representation.

use glam::DVec3;

use super::epsilons::TOLERANCE_VECTOR_EQUALITY;

#[derive(Clone, Debug, Default)]
pub struct Plane {
    pub id: usize,
    pub normal: DVec3,
    pub distance: f64,
}

impl Plane {
    pub fn is_equal_to(&self, normal: DVec3, distance: f64) -> bool {
        let eps = TOLERANCE_VECTOR_EQUALITY;
        (self.normal.x - normal.x).abs() <= eps
            && (self.normal.y - normal.y).abs() <= eps
            && (self.normal.z - normal.z).abs() <= eps
            && (self.distance - distance).abs() <= eps
    }
}
