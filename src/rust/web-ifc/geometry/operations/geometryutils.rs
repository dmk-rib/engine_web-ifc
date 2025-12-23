//! Rust port of `web-ifc/geometry/operations/geometryutils.h`.

use glam::{DMat4, DVec2, DVec3};

use crate::earcutr;
use crate::web_ifc::geometry::operations::bim_geometry::epsilons;
use crate::web_ifc::geometry::operations::bim_geometry::utils as bim_utils;
use crate::web_ifc::geometry::operations::bim_geometry::{geometry as bim_geometry, utils};
use crate::web_ifc::geometry::representation::geometry::{
    IfcBound3D, IfcBoundType, IfcComposedMesh, IfcCrossSections, IfcCurve, IfcProfile,
};
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;

pub fn set_epsilons(tolerance_scalar_equality: f64, plane_refit_iterations: f64, boolean_union_threshold: f64) {
    epsilons::set_epsilons(tolerance_scalar_equality, plane_refit_iterations, boolean_union_threshold);
}

pub fn angle_conversion(angle: f64, angle_units: &str) -> f64 {
    if angle_units == "RADIAN" {
        angle
    } else {
        (angle / 360.0) * 2.0 * bim_utils::CONST_PI
    }
}

pub fn to_ifc_geometry(geom: bim_geometry::Geometry) -> IfcGeometry {
    let mut ifc_geom = IfcGeometry::default();
    ifc_geom.base.base = geom;
    ifc_geom
}

pub fn sweep(
    scaling: f64,
    closed: bool,
    profile: &IfcProfile,
    directrix: &IfcCurve,
    initial_directrix_normal: DVec3,
    rotate90: bool,
    optimize: bool,
) -> IfcGeometry {
    to_ifc_geometry(bim_utils::sweep_function(
        scaling,
        closed,
        &profile.curve.base.points,
        &directrix.base.points,
        initial_directrix_normal,
        rotate90,
        optimize,
    ))
}

pub fn sweep_circular(
    scaling: f64,
    closed: bool,
    profile: &IfcProfile,
    radius: f64,
    directrix: &IfcCurve,
    initial_directrix_normal: DVec3,
    rotate90: bool,
) -> IfcGeometry {
    let mut profile_vector = Vec::new();
    let mut directrix_vector = Vec::new();

    for i in 0..directrix.base.points.len() {
        if i < directrix.base.points.len().saturating_sub(1) {
            if directrix.base.points[i].distance(directrix.base.points[i + 1])
                > epsilons::EPS_BIG2 * scaling
            {
                directrix_vector.push(directrix.base.points[i]);
            }
        } else {
            directrix_vector.push(directrix.base.points[i]);
        }
    }

    for pt in &profile.curve.base.points {
        profile_vector.push(*pt);
    }

    to_ifc_geometry(bim_utils::sweep_circular(
        scaling,
        closed,
        &profile_vector,
        radius,
        &directrix_vector,
        initial_directrix_normal,
        rotate90,
    ))
}

pub fn compute_safe_normal(v1: DVec3, v2: DVec3, v3: DVec3, normal: &mut DVec3, eps: f64) -> bool {
    bim_utils::compute_safe_normal(v1, v2, v3, normal, eps)
}

pub fn get_basis_from_coplanar_points(points: &[DVec3], v1: &mut DVec3, v2: &mut DVec3, v3: &mut DVec3) -> bool {
    if points.is_empty() {
        return false;
    }

    *v1 = points[0];
    for p in points.iter() {
        if *v1 != *p {
            *v2 = *p;
            break;
        }
    }

    let mut normal = DVec3::ZERO;
    for i in 0..4 {
        let eps = match i {
            0 => 100.0,
            1 => 1.0,
            2 => 0.01,
            _ => 1e-3,
        };
        for p in points.iter() {
            if compute_safe_normal(*v1, *v2, *p, &mut normal, eps) {
                *v3 = *p;
                return true;
            }
        }
    }

    let mut d1 = 0.0;
    for p in points.iter() {
        let d2 = v1.distance(*p);
        if d1 < d2 {
            d1 = d2;
            *v2 = *p;
        }
    }

    d1 = 0.0;
    for p in points.iter() {
        let d2 = v1.distance(*p);
        let d3 = v2.distance(*p);
        if d1 < d2 + d3 {
            d1 = d2 + d3;
            *v3 = *p;
        }
    }

    compute_safe_normal(*v1, *v2, *v3, &mut normal, 1e-8)
}

pub fn triangulate_bounds(geometry: &mut IfcGeometry, bounds: &mut [IfcBound3D], express_id: u32) {
    if bounds.len() == 1 && bounds[0].curve.base.points.len() == 3 {
        let c = &bounds[0].curve.base.points;
        geometry.base.base.add_face_points(c[0], c[1], c[2], u32::MAX);
        return;
    }

    if bounds.is_empty() || bounds[0].curve.base.points.len() < 3 {
        return;
    }

    if bounds.len() > 1 {
        let outer_index = bounds
            .iter()
            .position(|b| b.bound_type == IfcBoundType::OuterBound)
            .unwrap_or(0);
        bounds.swap(0, outer_index);
    }

    if bounds[0].bound_type != IfcBoundType::OuterBound {
        return;
    }

    let mut v1 = DVec3::ZERO;
    let mut v2 = DVec3::ZERO;
    let mut v3 = DVec3::ZERO;
    if !get_basis_from_coplanar_points(&bounds[0].curve.base.points, &mut v1, &mut v2, &mut v3) {
        return;
    }

    let mut v12 = (v3 - v2).normalize();
    let mut v13 = (v1 - v2).normalize();
    let mut n = v12.cross(v13).normalize();
    v12 = v13.cross(n);

    let mut test = IfcCurve::default();
    for pt in &bounds[0].curve.base.points {
        let pt2 = *pt - v1;
        let proj = DVec2::new(pt2.dot(v12), pt2.dot(v13));
        test.base.add(DVec3::new(proj.x, proj.y, 0.0), true);
    }

    if !test.base.is_ccw() {
        n *= -1.0;
        std::mem::swap(&mut v12, &mut v13);
    }

    let mut polygon: Vec<Vec<DVec2>> = vec![Vec::new(); bounds.len()];
    for (i, bound) in bounds.iter().enumerate() {
        for pt in &bound.curve.base.points {
            let pt2 = *pt - v1;
            let proj = DVec2::new(pt2.dot(v12), pt2.dot(v13));
            polygon[i].push(proj);
        }
    }

    for bound in bounds.iter() {
        for pt in &bound.curve.base.points {
            geometry.base.base.add_point(*pt, n);
        }
    }

    let mut data: Vec<f64> = Vec::new();
    let mut hole_indices: Vec<usize> = Vec::new();
    let mut vertex_count = 0usize;
    for (i, ring) in polygon.iter().enumerate() {
        if i > 0 {
            hole_indices.push(vertex_count);
        }
        for pt in ring.iter() {
            data.push(pt.x);
            data.push(pt.y);
            vertex_count += 1;
        }
    }

    let indices = earcutr::earcut(&data, Some(&hole_indices), 2);
    let offset = geometry.base.base.num_points - vertex_count as u32;
    for tri in indices.chunks(3) {
        if tri.len() == 3 {
            geometry
                .base
                .base
                .add_face_indices(offset + tri[0] as u32, offset + tri[1] as u32, offset + tri[2] as u32, u32::MAX);
        }
    }

    let _ = express_id;
}

pub fn sectioned_surface(profiles: IfcCrossSections, build_caps: bool) -> IfcGeometry {
    let mut profile_groups = Vec::new();
    for curve in profiles.curves {
        profile_groups.push(curve.base.points.clone());
    }

    let mut geom = to_ifc_geometry(bim_utils::sectioned_surface(&profile_groups, build_caps, epsilons::EPS_SMALL));
    if build_caps && profile_groups.len() > 1 {
        let mut cap_geom = IfcGeometry::default();
        let mut profile = profile_groups[0].clone();
        if !profile.is_empty() {
            let last_to_first = profile[0] - *profile.last().unwrap();
            if last_to_first.length() > 1e-8 {
                profile.push(profile[0]);
            }
        }

        let dir = (profile_groups[1][0] - profile_groups[0][0]).normalize_or_zero();
        let distance = (profile_groups[1][0] - profile_groups[0][0]).length();

        let mut holes_indices_hash = Vec::new();
        let mut polygon: Vec<Vec<[f64; 3]>> = vec![Vec::new(); profile_groups.len()];
        for pt in &profile {
            cap_geom.base.base.add_point_vec4((pt + dir * distance).extend(1.0), dir);
            polygon[0].push([pt.x, pt.y, pt.z]);
        }
        holes_indices_hash.resize(profile.len(), false);
        for (i, hole) in profile_groups.iter().enumerate().skip(1) {
            for (j, pt) in hole.iter().enumerate() {
                holes_indices_hash.push(j == 0);
                cap_geom.base.base.add_point_vec4((*pt + dir * distance).extend(1.0), dir);
                polygon[i].push([pt.x, pt.y, pt.z]);
            }
        }

        let proj = bim_utils::best_projection(&polygon[0]);
        let polygon2d = bim_utils::project_to_2d(&polygon, proj);
        let indices = earcutr::earcut(&polygon2d, None, 3);
        if indices.len() >= 3 {
            let winding = bim_utils::get_winding_of_triangle(
                cap_geom.base.base.get_point(indices[0] as usize),
                cap_geom.base.base.get_point(indices[1] as usize),
                cap_geom.base.base.get_point(indices[2] as usize),
            );
            let flip_winding = !winding;
            for i in (0..indices.len()).step_by(3) {
                if flip_winding {
                    cap_geom.base.base.add_face_indices(indices[i] as u32, indices[i + 2] as u32, indices[i + 1] as u32, u32::MAX);
                } else {
                    cap_geom.base.base.add_face_indices(indices[i] as u32, indices[i + 1] as u32, indices[i + 2] as u32, u32::MAX);
                }
            }
        }

        geom.merge_geometry(cap_geom.base.base);
    }

    geom
}

pub fn extrude(profile: IfcProfile, dir: DVec3, distance: f64, cutting_plane_normal: DVec3, cutting_plane_pos: DVec3) -> IfcGeometry {
    let mut contours: Vec<Vec<DVec3>> = Vec::new();
    let mut outer = profile.curve.base.points.clone();
    if !outer.is_empty() {
        let last_to_first = outer[0] - *outer.last().unwrap();
        if last_to_first.length() > 1e-8 {
            outer.push(outer[0]);
        }
    }
    contours.push(outer);
    for hole in profile.holes {
        contours.push(hole.base.points.clone());
    }
    to_ifc_geometry(bim_utils::extrude(contours, dir, distance, cutting_plane_normal, cutting_plane_pos))
}

pub fn sweep_fixed_reference(
    linear_scaling_factor: f64,
    closed: bool,
    profile: &IfcProfile,
    directrix: &IfcCurve,
    fixed_reference: DVec3,
) -> IfcGeometry {
    let ref_dir = fixed_reference.normalize_or_zero();
    let z_axis = DVec3::new(0.0, 0.0, 1.0);
    let rotation_axis = z_axis.cross(ref_dir);
    let angle = z_axis.dot(ref_dir).acos();
    let orientation = if rotation_axis.length() > epsilons::EPS_SMALL {
        DMat4::from_axis_angle(rotation_axis.normalize(), angle)
    } else {
        DMat4::IDENTITY
    };

    let profile_points = &profile.curve.base.points;
    let path_points = &directrix.base.points;
    if path_points.is_empty() {
        return IfcGeometry::default();
    }

    let segments = if closed {
        path_points.len()
    } else {
        path_points.len().saturating_sub(1)
    };

    let mut geom = IfcGeometry::default();
    let mut start_profile = Vec::new();
    let mut end_profile = Vec::new();
    let start_pos = path_points[0];
    for pt in profile_points {
        let transformed = orientation * pt.extend(1.0);
        start_profile.push(start_pos + transformed.truncate() * linear_scaling_factor);
    }

    for i in 0..segments {
        let pos = path_points[i];
        let next_pos = path_points[(i + 1) % path_points.len()];

        let mut current_profile = Vec::new();
        for pt in profile_points {
            let transformed = orientation * pt.extend(1.0);
            current_profile.push(pos + transformed.truncate() * linear_scaling_factor);
        }

        let mut next_profile = Vec::new();
        if !closed || i < segments - 1 {
            for pt in profile_points {
                let transformed = orientation * pt.extend(1.0);
                next_profile.push(next_pos + transformed.truncate() * linear_scaling_factor);
            }
        } else {
            next_profile = start_profile.clone();
        }

        if i == segments - 1 {
            end_profile = next_profile.clone();
        }

        for j in 0..profile_points.len() {
            let j_next = (j + 1) % profile_points.len();
            geom.base.base.add_face_points(current_profile[j], next_profile[j], next_profile[j_next], u32::MAX);
            geom.base.base.add_face_points(current_profile[j], next_profile[j_next], current_profile[j_next], u32::MAX);
        }
    }

    if !closed {
        let mut start_cap = profile.clone();
        start_cap.curve.base.points = start_profile;
        geom.merge_geometry(extrude(start_cap, DVec3::new(0.0, 0.0, -1.0), 0.0, DVec3::ZERO, DVec3::ZERO).base.base);

        let mut end_cap = profile.clone();
        end_cap.curve.base.points = end_profile;
        geom.merge_geometry(extrude(end_cap, DVec3::new(0.0, 0.0, 1.0), 0.0, DVec3::ZERO, DVec3::ZERO).base.base);
    }

    geom
}

pub fn vector_to_angle_2d(x: f64, y: f64) -> f64 {
    bim_utils::vector_to_angle(x, y)
}

pub fn matrix_flips_triangles(mat: DMat4) -> bool {
    mat.determinant() < 0.0
}

pub fn equals_vec3(a: DVec3, b: DVec3, eps: f64) -> bool {
    bim_utils::equals_vec3(a, b, eps)
}

pub fn equals(a: f64, b: f64, eps: f64) -> bool {
    bim_utils::equals(a, b, eps)
}

pub fn area_of_triangle(a: DVec3, b: DVec3, c: DVec3) -> f64 {
    bim_utils::area_of_triangle(a, b, c)
}

pub fn area_of_triangle_2d(a: DVec2, b: DVec2, c: DVec2) -> f64 {
    bim_utils::area_of_triangle_2d(a, b, c)
}

pub fn random_double(lo: f64, hi: f64) -> f64 {
    static mut SEED: u64 = 0x1234_5678_9ABC_DEF0;
    unsafe {
        SEED = SEED.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let normalized = (SEED >> 11) as f64 / ((1u64 << 53) as f64);
        lo + normalized * (hi - lo)
    }
}

fn get_origin_rec(mesh: &IfcComposedMesh, geometry_map: &std::collections::HashMap<u32, IfcGeometry>, mat: DMat4) -> Option<DVec3> {
    let new_mat = mat * mesh.transformation;
    let geom_it = geometry_map.get(&mesh.express_id);

    if let Some(mesh_geom) = geom_it {
        if mesh_geom.base.base.num_faces != 0 {
            let face = mesh_geom.base.base.get_face(0);
            let a = new_mat * mesh_geom.base.base.get_point(face.i0 as usize).extend(1.0);
            return Some(a.truncate());
        }
    }

    for child in &mesh.children {
        if let Some(v) = get_origin_rec(child, geometry_map, new_mat) {
            return Some(v);
        }
    }

    None
}

pub fn get_origin(mesh: &IfcComposedMesh, geometry_map: &std::collections::HashMap<u32, IfcGeometry>) -> DVec3 {
    get_origin_rec(mesh, geometry_map, DMat4::IDENTITY).unwrap_or(DVec3::ZERO)
}

fn flatten_recursive(mesh: &IfcComposedMesh, geometry_map: &std::collections::HashMap<u32, IfcGeometry>, geoms: &mut Vec<IfcGeometry>, mat: DMat4) {
    let new_mat = mat * mesh.transformation;
    let transformation_breaks_winding = matrix_flips_triangles(new_mat);

    if let Some(mesh_geom) = geometry_map.get(&mesh.express_id) {
        if !mesh_geom.part.is_empty() {
            for part in &mesh_geom.part {
                if part.base.base.num_faces != 0 {
                    let mut new_geom = IfcGeometry::default();
                    new_geom.half_space = part.half_space;
                    if new_geom.half_space {
                        new_geom.half_space_origin = (new_mat * part.half_space_origin.extend(1.0)).truncate();
                        new_geom.half_space_x = (new_mat * part.half_space_x.extend(1.0)).truncate();
                        new_geom.half_space_y = (new_mat * part.half_space_y.extend(1.0)).truncate();
                        new_geom.half_space_z = (new_mat * part.half_space_z.extend(1.0)).truncate();
                    }

                    for i in 0..part.base.base.num_faces as usize {
                        let f = part.base.base.get_face(i);
                        let a = (new_mat * part.base.base.get_point(f.i0 as usize).extend(1.0)).truncate();
                        let b = (new_mat * part.base.base.get_point(f.i1 as usize).extend(1.0)).truncate();
                        let c = (new_mat * part.base.base.get_point(f.i2 as usize).extend(1.0)).truncate();
                        if transformation_breaks_winding {
                            new_geom.base.base.add_face_points(b, a, c, f.p_id);
                        } else {
                            new_geom.base.base.add_face_points(a, b, c, f.p_id);
                        }
                    }
                    geoms.push(new_geom);
                }
            }
        } else if mesh_geom.base.base.num_faces != 0 {
            let mut new_geom = IfcGeometry::default();
            new_geom.half_space = mesh_geom.half_space;
            if new_geom.half_space {
                new_geom.half_space_origin = (new_mat * mesh_geom.half_space_origin.extend(1.0)).truncate();
                new_geom.half_space_x = (new_mat * mesh_geom.half_space_x.extend(1.0)).truncate();
                new_geom.half_space_y = (new_mat * mesh_geom.half_space_y.extend(1.0)).truncate();
                new_geom.half_space_z = (new_mat * mesh_geom.half_space_z.extend(1.0)).truncate();
            }

            for i in 0..mesh_geom.base.base.num_faces as usize {
                let f = mesh_geom.base.base.get_face(i);
                let a = (new_mat * mesh_geom.base.base.get_point(f.i0 as usize).extend(1.0)).truncate();
                let b = (new_mat * mesh_geom.base.base.get_point(f.i1 as usize).extend(1.0)).truncate();
                let c = (new_mat * mesh_geom.base.base.get_point(f.i2 as usize).extend(1.0)).truncate();
                if transformation_breaks_winding {
                    new_geom.base.base.add_face_points(b, a, c, f.p_id);
                } else {
                    new_geom.base.base.add_face_points(a, b, c, f.p_id);
                }
            }
            geoms.push(new_geom);
        }
    }

    for child in &mesh.children {
        flatten_recursive(child, geometry_map, geoms, new_mat);
    }
}

pub fn flatten(mesh: &IfcComposedMesh, geometry_map: &std::collections::HashMap<u32, IfcGeometry>, mat: DMat4) -> Vec<IfcGeometry> {
    let mut geoms = Vec::new();
    flatten_recursive(mesh, geometry_map, &mut geoms, mat);
    geoms
}

pub fn flatten_solids(geoms: &[IfcGeometry]) -> Vec<IfcGeometry> {
    let mut new_geom = IfcGeometry::default();
    let mut new_geoms = Vec::new();

    for geom in geoms {
        if !geom.half_space && geom.base.base.num_faces != 0 {
            for i in 0..geom.base.base.num_faces as usize {
                let f = geom.base.base.get_face(i);
                let a = geom.base.base.get_point(f.i0 as usize);
                let b = geom.base.base.get_point(f.i1 as usize);
                let c = geom.base.base.get_point(f.i2 as usize);
                new_geom.base.base.add_face_points(a, b, c, f.p_id);
            }
        }
    }
    new_geoms.push(new_geom);

    for geom in geoms {
        if geom.half_space {
            new_geoms.push(geom.clone());
        }
    }

    new_geoms
}

pub fn flatten_transformation(transformation: DMat4) -> [f64; 16] {
    transformation.to_cols_array()
}

pub fn not_present(pt: DVec3, points: &[DVec3]) -> bool {
    for pt2 in points {
        if pt.x == pt2.x && pt.y == pt2.y && pt.z == pt2.z {
            return false;
        }
    }
    true
}
