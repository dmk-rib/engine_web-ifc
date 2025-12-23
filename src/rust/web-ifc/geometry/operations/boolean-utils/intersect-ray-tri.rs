//! Rust port of `web-ifc/geometry/operations/boolean-utils/intersect-ray-tri.h`.

use glam::DVec3;

use super::eps::{
    TOLERANCE_PARALLEL_TIGHT, _TOLERANCE_BACK_DEVIATION_DISTANCE,
    _TOLERANCE_INSIDE_OUTSIDE_PERIMETER,
};

pub type Vec = DVec3;

pub fn intersect_ray_triangle(
    origin: &Vec,
    end: &Vec,
    v0: &Vec,
    v1: &Vec,
    v2: &Vec,
    hit_position: &mut Vec,
    t: &mut f64,
    d_plane: &mut f64,
    _infinite_length: bool,
) -> bool {
    let dir = *end - *origin;
    let v0v1 = *v1 - *v0;
    let v0v2 = *v2 - *v0;
    let n = v0v1.cross(v0v2);

    let ndot_ray_direction = n.dot(dir);
    if ndot_ray_direction.abs() < TOLERANCE_PARALLEL_TIGHT {
        return false;
    }

    let d = -n.dot(*v0);
    let d_origin = n.dot(*origin);
    *t = -(d_origin + d) / ndot_ray_direction;
    if *t < unsafe { -_TOLERANCE_BACK_DEVIATION_DISTANCE } {
        return false;
    }

    let p = *origin + dir * *t;
    *d_plane = (dir * *t).dot(n.normalize());

    let edge0 = *v1 - *v0;
    let v0p = p - *v0;
    let c0 = edge0.cross(v0p);
    let valdot0 = n.dot(c0);
    if valdot0 < unsafe { -_TOLERANCE_INSIDE_OUTSIDE_PERIMETER } {
        return false;
    }

    let edge1 = *v2 - *v1;
    let v1p = p - *v1;
    let c1 = edge1.cross(v1p);
    let valdot1 = n.dot(c1);
    if valdot1 < unsafe { -_TOLERANCE_INSIDE_OUTSIDE_PERIMETER } {
        return false;
    }

    let edge2 = *v0 - *v2;
    let v2p = p - *v2;
    let c2 = edge2.cross(v2p);
    let valdot2 = n.dot(c2);
    if valdot2 < unsafe { -_TOLERANCE_INSIDE_OUTSIDE_PERIMETER } {
        return false;
    }

    *hit_position = p;
    true
}
