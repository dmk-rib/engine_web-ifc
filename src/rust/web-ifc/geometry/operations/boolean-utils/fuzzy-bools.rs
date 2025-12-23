//! Rust port of `web-ifc/geometry/operations/boolean-utils/fuzzy-bools.h`.

use super::bvh::make_bvh;
use super::clip_mesh::{clip_join, clip_subtract};
use super::eps::{
    _BOOLSTATUS, _TOLERANCE_BACK_DEVIATION_DISTANCE, _TOLERANCE_BOUNDING_BOX,
    _TOLERANCE_INSIDE_OUTSIDE_PERIMETER, _TOLERANCE_PLANE_DEVIATION, _TOLERANCE_PLANE_INTERSECTION,
};
use super::geometry::Geometry;

pub fn set_epsilons(
    tolerance_plane_intersection: f64,
    tolerance_plane_deviation: f64,
    tolerance_back_deviation_distance: f64,
    tolerance_inside_outside_perimeter: f64,
    tolerance_bounding_box: f64,
    bool_status: f64,
) {
    unsafe {
        _TOLERANCE_PLANE_INTERSECTION = tolerance_plane_intersection;
        _TOLERANCE_PLANE_DEVIATION = tolerance_plane_deviation;
        _TOLERANCE_BACK_DEVIATION_DISTANCE = tolerance_back_deviation_distance;
        _TOLERANCE_INSIDE_OUTSIDE_PERIMETER = tolerance_inside_outside_perimeter;
        _TOLERANCE_BOUNDING_BOX = tolerance_bounding_box;
        _BOOLSTATUS = bool_status;
    }
}

pub fn subtract(a: &Geometry, b: &Geometry) -> Geometry {
    let bvh1 = make_bvh(a);
    let bvh2 = make_bvh(b);
    clip_subtract(a, bvh1, bvh2)
}

pub fn union(a: &Geometry, b: &Geometry) -> Geometry {
    let bvh1 = make_bvh(a);
    let bvh2 = make_bvh(b);
    clip_join(a, bvh1, bvh2)
}
