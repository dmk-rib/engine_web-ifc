//! Rust port of `web-ifc/geometry/operations/boolean-utils/shared-position.h`.

use glam::{DVec2, DVec3};

use super::eps::{EPS_SMALL, TOLERANCE_COLLINEAR, TOLERANCE_POINT_ON_LINE};
use super::math::equals_vec3;

#[derive(Clone, Debug, Default)]
pub struct PlaneBasis {
    pub origin: DVec3,
    pub up: DVec3,
    pub left: DVec3,
    pub right: DVec3,
}

impl PlaneBasis {
    pub fn project(&self, pt: DVec3) -> DVec2 {
        let relative = pt - self.origin;
        DVec2::new(relative.dot(self.left), relative.dot(self.right))
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReferencePlane {
    pub plane_id: usize,
    pub point_id: usize,
    pub line_id: usize,
    pub location: DVec2,
}

#[derive(Clone, Debug, Default)]
pub struct ReferenceLine {
    pub line_id: usize,
    pub point_id: usize,
    pub location: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Line {
    pub id: usize,
    pub global_id: usize,
    pub origin: DVec3,
    pub direction: DVec3,
    pub points: Vec<(f64, usize)>,
}

impl Line {
    pub fn new() -> Self {
        static mut IDCOUNTER: usize = 0;
        unsafe {
            IDCOUNTER += 1;
            Line {
                global_id: IDCOUNTER,
                ..Default::default()
            }
        }
    }

    pub fn is_point_on_line(&self, a: DVec3) -> bool {
        let d = self.direction.normalize();
        let v = a - self.origin;
        let t = v.dot(d);
        let p = self.origin + t * d;
        p.distance(a) < TOLERANCE_POINT_ON_LINE
    }

    pub fn get_pos_on_line(&self, pos: DVec3) -> f64 {
        let unit_direction = self.direction.normalize();
        (pos - self.origin).dot(unit_direction)
    }

    pub fn get_pos_on_line_dist(&self, dist: f64) -> DVec3 {
        let unit_direction = self.direction.normalize();
        self.origin + dist * unit_direction
    }

    pub fn is_collinear(&self, other: &Line) -> bool {
        let unit_direction = self.direction.normalize();
        let unit_other = other.direction.normalize();
        equals_vec3(unit_other, unit_direction, TOLERANCE_COLLINEAR)
            || equals_vec3(unit_other, -unit_direction, TOLERANCE_COLLINEAR)
    }

    pub fn is_equal_to(&self, pos: DVec3, dir: DVec3) -> bool {
        let unit_dir = dir.normalize();
        let unit_direction = self.direction.normalize();
        if !(equals_vec3(unit_dir, unit_direction, EPS_SMALL)
            || equals_vec3(unit_dir, -unit_direction, EPS_SMALL))
        {
            return false;
        }
        self.is_point_on_line(pos)
    }

    pub fn add_point_to_line(&mut self, dist: f64, id: usize) {
        for (_, pid) in &self.points {
            if *pid == id {
                return;
            }
        }
        self.points.push((dist, id));
        self.points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }
}

