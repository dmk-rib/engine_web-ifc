//! Rust port of `web-ifc/geometry/operations/boolean-utils/clip-mesh.h`.

use glam::DVec3;

use super::bvh::BVH;
use super::eps::{EPS_MINISCULE, MESSAGES};
use super::geometry::{Face, Geometry};
use super::is_inside_mesh::{is_inside_mesh, MeshLocation};
use super::math::{compute_normal, equals_vec3};

pub fn double_clip_single_mesh(
    mesh: &Geometry,
    bvh1: &mut BVH<'_>,
    bvh2: &mut BVH<'_>,
    result: &mut Geometry,
) {
    let mut bounding_list: Vec<(usize, super::aabb::AABB)> = Vec::new();

    for plane in &mesh.planes {
        result.has_planes = true;
        result.planes.push(plane.clone());
    }

    for i in 0..mesh.data as usize {
        let mut doit = false;
        let tri = mesh.get_face(i);
        let a = mesh.get_point(tri.i0 as usize);
        let b = mesh.get_point(tri.i1 as usize);
        let c = mesh.get_point(tri.i2 as usize);

        let aabb = mesh.get_face_box(i);

        if !aabb.intersects(&bvh2.box_) {
            // skip
        } else if !aabb.intersects(&bvh1.box_) {
            // skip
        }

        let mut do_next = true;
        for (index, prev_aabb) in &bounding_list {
            if aabb.intersects(prev_aabb) {
                let tri_temp = mesh.get_face(*index);
                let at = mesh.get_point(tri_temp.i0 as usize);
                let bt = mesh.get_point(tri_temp.i1 as usize);
                let ct = mesh.get_point(tri_temp.i2 as usize);

                if (equals_vec3(at, a, EPS_MINISCULE)
                    && equals_vec3(bt, b, EPS_MINISCULE)
                    && equals_vec3(ct, c, EPS_MINISCULE))
                    || (equals_vec3(at, b, EPS_MINISCULE)
                        && equals_vec3(bt, c, EPS_MINISCULE)
                        && equals_vec3(ct, a, EPS_MINISCULE))
                    || (equals_vec3(at, c, EPS_MINISCULE)
                        && equals_vec3(bt, a, EPS_MINISCULE)
                        && equals_vec3(ct, b, EPS_MINISCULE))
                {
                    do_next = false;
                    break;
                }
            }
        }

        if !do_next {
            continue;
        }

        bounding_list.push((i, aabb));

        let n = compute_normal(a, b, c);
        let tri_center = (a + b * 1.02 + c * 1.03) / 3.05;

        let raydir = compute_normal(a, b, c);

        let mut is_inside1_loc = is_inside_mesh(
            tri_center,
            n,
            bvh1.ptr.expect("BVH missing mesh"),
            bvh1,
            raydir,
            false,
        );
        let mut is_inside2_loc = is_inside_mesh(
            tri_center,
            n,
            bvh2.ptr.expect("BVH missing mesh"),
            bvh2,
            raydir,
            false,
        );

        let extra_dir1 = (raydir + DVec3::new(0.02, 0.01, 0.04)).normalize();
        let extra_dir2 = (raydir + DVec3::new(0.20, -0.1, 0.40)).normalize();

        let is_inside1_loc_b = is_inside_mesh(
            tri_center,
            n,
            bvh1.ptr.expect("BVH missing mesh"),
            bvh1,
            extra_dir1,
            false,
        );
        let is_inside2_loc_b = is_inside_mesh(
            tri_center,
            n,
            bvh2.ptr.expect("BVH missing mesh"),
            bvh2,
            extra_dir1,
            false,
        );

        if is_inside1_loc.loc != is_inside1_loc_b.loc {
            let is_inside1_loc_c = is_inside_mesh(
                tri_center,
                n,
                bvh1.ptr.expect("BVH missing mesh"),
                bvh1,
                extra_dir2,
                false,
            );
            if is_inside1_loc_c.loc == is_inside1_loc_b.loc {
                is_inside1_loc = is_inside1_loc_b;
            } else if is_inside1_loc_b.loc != is_inside1_loc_c.loc
                && is_inside1_loc.loc != is_inside1_loc_c.loc
            {
                is_inside1_loc = is_inside1_loc_b;
            }
        }

        if is_inside2_loc.loc != is_inside2_loc_b.loc {
            let is_inside2_loc_c = is_inside_mesh(
                tri_center,
                n,
                bvh2.ptr.expect("BVH missing mesh"),
                bvh2,
                extra_dir2,
                false,
            );
            if is_inside2_loc_c.loc == is_inside2_loc_b.loc {
                is_inside2_loc = is_inside2_loc_b;
            } else if is_inside2_loc_b.loc != is_inside2_loc_c.loc
                && is_inside2_loc.loc != is_inside2_loc_c.loc
            {
                is_inside2_loc = is_inside2_loc_b;
            }
        }

        let is_inside1 = is_inside1_loc.loc;
        let is_inside2 = is_inside2_loc.loc;

        if is_inside1 == MeshLocation::Outside && is_inside2 == MeshLocation::Outside {
            // both outside
        }
        if is_inside1 != MeshLocation::Boundary && is_inside2 != MeshLocation::Boundary {
            // neither boundary
        } else if is_inside1 == MeshLocation::Boundary && is_inside2 == MeshLocation::Boundary {
            let dot = is_inside1_loc.normal.dot(is_inside2_loc.normal);
            if dot < 0.0 {
                result.add_face_points(a, b, c, tri.p_id);
                doit = true;
            }
        } else if is_inside2 == MeshLocation::Inside || is_inside1 == MeshLocation::Outside {
            // skip
        } else if is_inside2 == MeshLocation::Boundary && is_inside1 == MeshLocation::Inside {
            if n.dot(is_inside2_loc.normal) < 0.0 {
                result.add_face_points(a, b, c, tri.p_id);
                doit = true;
            } else {
                result.add_face_points(b, a, c, tri.p_id);
                doit = true;
            }
        } else if is_inside1 == MeshLocation::Boundary {
            if n.dot(is_inside1_loc.normal) < 0.0 {
                result.add_face_points(b, a, c, tri.p_id);
                doit = true;
            } else {
                result.add_face_points(a, b, c, tri.p_id);
                doit = true;
            }
        } else {
            result.add_face_points(a, b, c, tri.p_id);
            doit = true;
        }

        if MESSAGES && doit {
            let _ = doit;
        }
    }

    for i in mesh.data as usize..mesh.num_faces as usize {
        let tri = mesh.get_face(i);
        let a = mesh.get_point(tri.i0 as usize);
        let b = mesh.get_point(tri.i1 as usize);
        let c = mesh.get_point(tri.i2 as usize);
        result.add_face_points(a, b, c, tri.p_id);
    }
}

pub fn double_clip_single_mesh2(
    mesh: &Geometry,
    bvh1: &mut BVH<'_>,
    bvh2: &mut BVH<'_>,
    result: &mut Geometry,
) {
    for plane in &mesh.planes {
        result.has_planes = true;
        result.planes.push(plane.clone());
    }

    for i in 0..mesh.data as usize {
        let tri = mesh.get_face(i);
        let a = mesh.get_point(tri.i0 as usize);
        let b = mesh.get_point(tri.i1 as usize);
        let c = mesh.get_point(tri.i2 as usize);

        let n = compute_normal(a, b, c);
        let _area = super::math::area_of_triangle(a, b, c);
        let tri_center = (a + b * 2.0 + c * 3.0) / 6.0;

        let raydir = compute_normal(a, b, c);

        let mut is_inside1_loc = is_inside_mesh(
            tri_center,
            n,
            bvh1.ptr.expect("BVH missing mesh"),
            bvh1,
            raydir,
            true,
        );
        let mut is_inside2_loc = is_inside_mesh(
            tri_center,
            n,
            bvh2.ptr.expect("BVH missing mesh"),
            bvh2,
            raydir,
            true,
        );

        let extra_dir1 = DVec3::new(1.1, 1.4, 1.2).normalize();
        let extra_dir2 = DVec3::new(-2.1, 1.4, -3.2).normalize();

        let is_inside1_loc_b = is_inside_mesh(
            tri_center,
            n,
            bvh1.ptr.expect("BVH missing mesh"),
            bvh1,
            extra_dir1,
            true,
        );
        let is_inside2_loc_b = is_inside_mesh(
            tri_center,
            n,
            bvh2.ptr.expect("BVH missing mesh"),
            bvh2,
            extra_dir1,
            true,
        );

        if is_inside1_loc.loc != is_inside1_loc_b.loc {
            let is_inside1_loc_c = is_inside_mesh(
                tri_center,
                n,
                bvh1.ptr.expect("BVH missing mesh"),
                bvh1,
                extra_dir2,
                true,
            );
            if is_inside1_loc_c.loc == is_inside1_loc_b.loc {
                is_inside1_loc = is_inside1_loc_b;
            } else if is_inside1_loc_b.loc != is_inside1_loc_c.loc
                && is_inside1_loc.loc != is_inside1_loc_c.loc
            {
                is_inside1_loc = is_inside1_loc_b;
            }
        }

        if is_inside2_loc.loc != is_inside2_loc_b.loc {
            let is_inside2_loc_c = is_inside_mesh(
                tri_center,
                n,
                bvh2.ptr.expect("BVH missing mesh"),
                bvh2,
                extra_dir2,
                true,
            );
            if is_inside2_loc_c.loc == is_inside2_loc_b.loc {
                is_inside2_loc = is_inside2_loc_b;
            } else if is_inside2_loc_b.loc != is_inside2_loc_c.loc
                && is_inside2_loc.loc != is_inside2_loc_c.loc
            {
                is_inside2_loc = is_inside2_loc_b;
            }
        }

        let is_inside1 = is_inside1_loc.loc;
        let is_inside2 = is_inside2_loc.loc;

        if is_inside1 == MeshLocation::Outside && is_inside2 == MeshLocation::Outside {
            // both outside
        } else if is_inside1 == MeshLocation::Inside || is_inside2 == MeshLocation::Inside {
            // keep boundaries only
        } else if is_inside1 == MeshLocation::Boundary && is_inside2 == MeshLocation::Boundary {
            let dot = is_inside1_loc.normal.dot(is_inside2_loc.normal);
            if dot > 0.0 {
                result.add_face_points(a, b, c, tri.p_id);
            }
        } else if is_inside1 == MeshLocation::Boundary && is_inside2 == MeshLocation::Outside {
            if n.dot(is_inside1_loc.normal) < 0.0 {
                result.add_face_points(b, a, c, tri.p_id);
            } else {
                result.add_face_points(a, b, c, tri.p_id);
            }
        } else if is_inside2 == MeshLocation::Boundary && is_inside1 == MeshLocation::Outside {
            if n.dot(is_inside2_loc.normal) < 0.0 {
                result.add_face_points(b, a, c, tri.p_id);
            } else {
                result.add_face_points(a, b, c, tri.p_id);
            }
        }
    }

    for i in mesh.data as usize..mesh.num_faces as usize {
        let tri = mesh.get_face(i);
        let a = mesh.get_point(tri.i0 as usize);
        let b = mesh.get_point(tri.i1 as usize);
        let c = mesh.get_point(tri.i2 as usize);
        result.add_face_points(a, b, c, tri.p_id);
    }
}

pub fn clip_join(mesh: &Geometry, mut bvh1: BVH<'_>, mut bvh2: BVH<'_>) -> Geometry {
    let mut resulting_mesh = Geometry::default();
    double_clip_single_mesh2(mesh, &mut bvh1, &mut bvh2, &mut resulting_mesh);
    resulting_mesh
}

pub fn clip_subtract(mesh: &Geometry, mut bvh1: BVH<'_>, mut bvh2: BVH<'_>) -> Geometry {
    let mut resulting_mesh = Geometry::default();
    double_clip_single_mesh(mesh, &mut bvh1, &mut bvh2, &mut resulting_mesh);
    resulting_mesh
}
