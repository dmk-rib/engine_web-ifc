//! Rust port of `web-ifc/geometry/operations/boolean-utils/is-inside-mesh.h`.

use glam::DVec3;

use super::bvh::BVH;
use super::eps::{TOLERANCE_PARALLEL, _TOLERANCE_PLANE_DEVIATION};
use super::geometry::{Face, Geometry};
use super::intersect_ray_tri::intersect_ray_triangle;
use super::math::compute_normal;

pub type Vec = DVec3;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MeshLocation {
    Inside,
    Outside,
    Boundary,
}

#[derive(Copy, Clone, Debug)]
pub struct InsideResult {
    pub loc: MeshLocation,
    pub normal: Vec,
}

pub fn is_inside_mesh(
    pt: Vec,
    normal: Vec,
    geom: &Geometry,
    bvh: &BVH<'_>,
    mut dir: Vec,
    union: bool,
) -> InsideResult {
    let mut winding = 0;
    dir += Vec::new(0.02, 0.01, 0.04);
    let mut result = InsideResult {
        loc: MeshLocation::Boundary,
        normal: DVec3::ZERO,
    };

    let has_result = bvh.intersect_ray(pt, dir, |i| {
        let f: Face = geom.get_face(i as usize);
        let a = geom.get_point(f.i0 as usize);
        let b = geom.get_point(f.i1 as usize);
        let c = geom.get_point(f.i2 as usize);

        let mut intersection = DVec3::ZERO;
        let mut distance = 0.0;
        let mut d_plane = 0.0;
        let has_intersection = intersect_ray_triangle(
            &pt,
            &(pt + dir),
            &a,
            &b,
            &c,
            &mut intersection,
            &mut distance,
            &mut d_plane,
            true,
        );
        if has_intersection {
            let other_normal = compute_normal(a, b, c);
            let dn = other_normal.dot(normal);
            if d_plane.abs() < unsafe { _TOLERANCE_PLANE_DEVIATION } {
                if dn > 1.0 - TOLERANCE_PARALLEL {
                    result.loc = MeshLocation::Boundary;
                    result.normal = normal;
                    return true;
                } else if dn < -1.0 + TOLERANCE_PARALLEL {
                    if !union {
                        result.loc = MeshLocation::Outside;
                        result.normal = normal;
                        return true;
                    }
                    result.loc = MeshLocation::Boundary;
                    result.normal = normal;
                    return true;
                } else {
                    result.loc = MeshLocation::Boundary;
                    result.normal = other_normal;
                    return true;
                }
            }

            winding += 1;
        }

        false
    });

    if has_result {
        return result;
    }

    result.loc = if winding % 2 == 1 {
        MeshLocation::Inside
    } else {
        MeshLocation::Outside
    };
    result
}
