//! Curve container utilities.

use glam::{DVec2, DVec3};

use super::epsilons::EPS_TINY_CURVE;
use super::utils::equals_vec3;

#[derive(Clone, Debug, Default)]
pub struct Curve {
    pub points: Vec<DVec3>,
}

impl Curve {
    pub fn add(&mut self, pt: DVec3, remove_coincident: bool) {
        if self.points.is_empty() || !remove_coincident {
            self.points.push(pt);
        } else if !equals_vec3(pt, *self.points.last().unwrap(), EPS_TINY_CURVE) {
            self.points.push(pt);
        }
    }

    pub fn add_2d(&mut self, pt: DVec2) {
        self.add(DVec3::new(pt.x, pt.y, 0.0), true);
    }

    pub fn invert(&mut self) {
        self.points.reverse();
    }

    pub fn is_ccw(&self) -> bool {
        let n = self.points.len();
        let mut sum = 0.0;
        for i in 0..n {
            let pt1 = self.points[(i + n - 1) % n];
            let pt2 = self.points[i];
            sum += (pt2.x - pt1.x) * (pt2.y + pt1.y);
        }
        sum < 0.0
    }
}
