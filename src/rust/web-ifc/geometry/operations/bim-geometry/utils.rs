//! Geometry utility functions for bim-geometry.

use glam::{DMat3, DMat4, DVec2, DVec3};

use super::curve::Curve;
use super::epsilons::{EPS_BIG2, EPS_MINISCULE, EPS_SMALL, EPS_TINY_CURVE};
use super::geometry::Geometry;
use super::plane::Plane;

pub const CONST_PI: f64 = 3.141592653589793238462643383279502884;
pub const VERTEX_FORMAT_SIZE_FLOATS: i32 = 6;

#[inline]
pub fn cross2d(point1: DVec2, point2: DVec2) -> f64 {
    point1.x * point2.y - point1.y * point2.x
}

#[inline]
pub fn equals_vec3(a: DVec3, b: DVec3, eps: f64) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps
}

#[inline]
pub fn equals(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

#[inline]
pub fn area_of_triangle(a: DVec3, b: DVec3, c: DVec3) -> f64 {
    let ab = b - a;
    let ac = c - a;
    let norm = ab.cross(ac);
    norm.length() / 2.0
}

#[inline]
pub fn area_of_triangle_2d(a: DVec2, b: DVec2, c: DVec2) -> f64 {
    let ab = b - a;
    let ac = c - a;
    let norm = cross2d(ab, ac) / 2.0;
    norm.abs()
}

#[inline]
pub fn compute_safe_normal(v1: DVec3, v2: DVec3, v3: DVec3, normal: &mut DVec3, eps: f64) -> bool {
    let v12 = v2 - v1;
    let v13 = v3 - v1;
    let norm = v12.cross(v13);
    let len = norm.length();
    if len <= eps {
        return false;
    }
    *normal = norm / len;
    true
}

#[inline]
pub fn vector_to_angle(x: f64, y: f64) -> f64 {
    let dd = (x * x + y * y).sqrt();
    if dd.abs() < EPS_MINISCULE {
        return 0.0;
    }
    let xx = x / dd;
    let yy = y / dd;

    let mut angle = xx.acos();
    let mut cosv = angle.cos();
    let mut sinv = angle.sin();
    if (xx - cosv).abs() > 1e-5 || (yy - sinv).abs() > 1e-5 {
        angle = yy.asin();
        sinv = angle.sin();
        cosv = angle.cos();
        if (xx - cosv).abs() > 1e-5 || (yy - sinv).abs() > 1e-5 {
            angle = angle + (CONST_PI - angle) * 2.0;
            sinv = angle.sin();
            cosv = angle.cos();
            if (xx - cosv).abs() > 1e-5 || (yy - sinv).abs() > 1e-5 {
                angle += CONST_PI;
            }
        }
    }

    (angle / (2.0 * CONST_PI)) * 360.0
}

#[inline]
pub fn get_winding_of_triangle(a: DVec3, b: DVec3, c: DVec3) -> bool {
    let v12 = b - a;
    let v13 = c - a;
    let norm = v12.cross(v13).normalize();
    norm.dot(DVec3::new(0.0, 0.0, 1.0)) > 0.0
}

pub fn revolution(transform: DMat4, start_degrees: f64, end_degrees: f64, profile: Vec<DVec3>, num_rots: f64) -> Geometry {
    let mut geometry = Geometry::default();

    let cent = transform.w_axis.truncate();
    let vec_x = transform.x_axis.truncate().normalize();
    let vec_y = transform.y_axis.truncate().normalize();
    let vec_z = transform.z_axis.truncate().normalize();

    let mut new_points: Vec<Vec<DVec3>> = vec![Vec::new(); num_rots as usize];

    let start_rad = start_degrees / 180.0 * CONST_PI;
    let end_rad = end_degrees / 180.0 * CONST_PI;
    let rad_span = end_rad - start_rad;
    let rad_step = rad_span / (num_rots - 1.0);

    for pt in profile {
        let xx = pt.x - cent.x;
        let yy = pt.y - cent.y;
        let zz = pt.z - cent.z;

        let dx = vec_x.dot(DVec3::new(xx, yy, zz));
        let dy = vec_y.dot(DVec3::new(xx, yy, zz));
        let dz = vec_z.dot(DVec3::new(xx, yy, zz));
        let dd = (dx * dx + dy * dy).sqrt();

        for r in 0..(num_rots as usize) {
            let angle = start_rad + (r as f64) * rad_step;
            let dtemp_x = angle.sin() * dd;
            let dtemp_y = angle.cos() * dd;
            let new_px = dtemp_x * vec_x.x + dtemp_y * vec_y.x + dz * vec_z.x + cent.x;
            let new_py = dtemp_x * vec_x.y + dtemp_y * vec_y.y + dz * vec_z.y + cent.y;
            let new_pz = dtemp_x * vec_x.z + dtemp_y * vec_y.z + dz * vec_z.z + cent.z;
            new_points[r].push(DVec3::new(new_px, new_py, new_pz));
        }
    }

    for r in 0..(new_points.len() - 1) {
        let cap_size = new_points[r].len();
        for j in 1..cap_size {
            let bl = new_points[r][j - 1];
            let br = new_points[r][j];
            let tl = new_points[r + 1][j - 1];
            let tr = new_points[r + 1][j];

            geometry.add_face_points(tl, br, bl, u32::MAX);
            geometry.add_face_points(tl, tr, br, u32::MAX);
        }
    }

    geometry
}

pub fn revolve_cylinder(
    transform: DMat4,
    start_degrees: f64,
    end_degrees: f64,
    min_z: f64,
    max_z: f64,
    num_rots: i32,
    radius: f64,
) -> Geometry {
    let mut profile = vec![
        DVec3::new(radius, 0.0, min_z),
        DVec3::new(radius, 0.0, max_z),
    ];
    revolution(transform, start_degrees, end_degrees, profile, num_rots as f64)
}

#[inline]
pub fn matrix_flips_triangles_mat3(mat: DMat3) -> bool {
    let cx = mat.x_axis;
    let cy = mat.y_axis;
    let cz = mat.z_axis;
    let dx = cy.cross(cz);
    let dy = cx.cross(cz);
    let dz = cx.cross(cy);
    let fac1 = cx.dot(dx);
    let fac2 = -cy.dot(dy);
    let fac3 = cz.dot(dz);
    fac1 * fac2 * fac3 < 0.0
}

#[inline]
pub fn matrix_flips_triangles(mat: DMat4) -> bool {
    matrix_flips_triangles_mat3(DMat3::from_cols(mat.x_axis.truncate(), mat.y_axis.truncate(), mat.z_axis.truncate()))
}

pub fn convert_2d_alignments_to_3d(horizontal: &[DVec3], vertical: &[DVec3]) -> Vec<DVec3> {
    let mut points = Vec::new();
    for h in horizontal {
        points.push(DVec3::new(h.x, h.y, h.z));
    }
    for v in vertical {
        points.push(DVec3::new(v.x, v.y, v.z));
    }
    points
}

pub fn get_ellipse_curve(
    radius_x: f32,
    radius_y: f32,
    num_segments: i32,
    placement: DMat3,
    start_rad: f64,
    end_rad: f64,
    swap: bool,
    normal_to_center_ending: bool,
) -> Curve {
    let mut curve = Curve::default();
    let n = num_segments.max(3) as usize;
    let span = end_rad - start_rad;
    let step = span / (n as f64);

    for i in 0..=n {
        let angle = start_rad + (i as f64) * step;
        let mut x = angle.cos() * radius_x as f64;
        let mut y = angle.sin() * radius_y as f64;
        if swap {
            std::mem::swap(&mut x, &mut y);
        }
        let local = DVec3::new(x, y, 0.0);
        let world = placement * local;
        curve.add(world, true);
    }

    if normal_to_center_ending {
        if let Some(last) = curve.points.last().copied() {
            let center = placement * DVec3::ZERO;
            let dir = (center - last).normalize_or_zero();
            curve.add(last + dir * EPS_TINY_CURVE, false);
        }
    }

    curve
}

pub fn solve_parabola(
    segments: u16,
    start_point: DVec2,
    horizontal_length: f64,
    start_height: f64,
    start_gradient: f64,
    end_gradient: f64,
) -> Vec<DVec2> {
    let mut points = Vec::with_capacity(segments as usize + 1);
    let d = horizontal_length;
    let a = (end_gradient - start_gradient) / (2.0 * d);
    let b = start_gradient;
    let c = start_height;
    let step = d / segments as f64;

    for i in 0..=segments {
        let x = i as f64 * step;
        let y = a * x * x + b * x + c;
        points.push(DVec2::new(start_point.x + x, start_point.y + y));
    }
    points
}

pub fn solve_clothoid(
    segments: u16,
    start_point: DVec2,
    ifc_start_direction: f64,
    start_radius_of_curvature: f64,
    end_radius_of_curvature: f64,
    segment_length: f64,
) -> Vec<DVec2> {
    let mut points = Vec::with_capacity(segments as usize + 1);

    let mut angle = ifc_start_direction;
    let mut pos = DVec2::new(start_point.x, start_point.y);
    points.push(pos);

    let step = segment_length / segments as f64;
    let curvature_delta = (1.0 / end_radius_of_curvature) - (1.0 / start_radius_of_curvature);

    for i in 1..=segments {
        let t = i as f64 / segments as f64;
        let curvature = (1.0 / start_radius_of_curvature) + curvature_delta * t;
        angle += curvature * step;
        pos += DVec2::new(angle.cos(), angle.sin()) * step;
        points.push(pos);
    }

    points
}

pub fn extrude(points: &[DVec3], dir: DVec3, len: f64) -> Geometry {
    let mut geom = Geometry::default();
    if points.len() < 2 {
        return geom;
    }
    for j in 0..(points.len() - 1) {
        let j2 = j + 1;
        let npt1 = points[j] + dir * len;
        let npt2 = points[j2] + dir * len;
        geom.add_face_points(points[j], points[j2], npt1, u32::MAX);
        geom.add_face_points(points[j2], npt2, npt1, u32::MAX);
    }
    geom
}

#[derive(Copy, Clone, Debug)]
pub enum Projection {
    XY,
    XZ,
    YZ,
}

type Point = [f64; 3];

pub fn best_projection(poly: &[Point]) -> Projection {
    let area2d = |p: &[Point], i1: usize, i2: usize| -> f64 {
        let mut area = 0.0;
        for i in 0..p.len() {
            let a = p[i];
            let b = p[(i + 1) % p.len()];
            area += (a[i1] * b[i2]) - (b[i1] * a[i2]);
        }
        (area * 0.5).abs()
    };

    let area_xy = area2d(poly, 0, 1);
    let area_xz = area2d(poly, 0, 2);
    let area_yz = area2d(poly, 1, 2);

    if area_xy >= area_xz && area_xy >= area_yz {
        Projection::XY
    } else if area_xz >= area_yz {
        Projection::XZ
    } else {
        Projection::YZ
    }
}

pub fn project_to_2d(poly3d: &[Vec<Point>], proj: Projection) -> Vec<Vec<Point>> {
    let mut poly2d = vec![Vec::new(); poly3d.len()];
    for (i, poly) in poly3d.iter().enumerate() {
        for pt in poly {
            match proj {
                Projection::XY => poly2d[i].push([pt[0], pt[1], pt[2]]),
                Projection::XZ => poly2d[i].push([pt[0], pt[2], pt[1]]),
                Projection::YZ => poly2d[i].push([pt[1], pt[2], pt[0]]),
            }
        }
    }
    poly2d
}

pub fn extrude_with_profiles(
    mut profile: Vec<Vec<DVec3>>,
    dir: DVec3,
    distance: f64,
    cutting_plane_normal: DVec3,
    cutting_plane_pos: DVec3,
) -> Geometry {
    let mut geom = Geometry::default();
    let mut holes_indices_hash: Vec<bool> = Vec::new();

    if !profile.is_empty() {
        let last_to_first = profile[0].first().unwrap() - profile[0].last().unwrap();
        if last_to_first.length() > 1e-8 {
            profile[0].push(*profile[0].first().unwrap());
        }
    }

    let polygon_count = profile.len();
    if polygon_count == 0 {
        return geom;
    }

    let mut polygon: Vec<Vec<Point>> = vec![Vec::new(); polygon_count];
    let mut normal = dir;

    for pt in &profile[0] {
        let et = *pt + dir * distance;
        geom.add_point_vec4(et.extend(1.0), normal);
        polygon[0].push([pt.x, pt.y, pt.z]);
    }

    holes_indices_hash.resize(profile[0].len(), false);

    for (i, hole) in profile.iter().enumerate().skip(1) {
        for (j, pt) in hole.iter().enumerate() {
            holes_indices_hash.push(j == 0);
            let et = *pt + dir * distance;
            profile[0].push(*pt);
            geom.add_point_vec4(et.extend(1.0), normal);
            polygon[i].push([pt.x, pt.y, pt.z]);
        }
    }

    let proj = best_projection(&polygon[0]);
    let polygon2d = project_to_2d(&polygon, proj);
    let indices = earcutr::earcut(&polygon2d, None, 3);

    let mut offset = 0u32;
    if indices.len() >= 3 {
        let winding = get_winding_of_triangle(
            geom.get_point((offset + indices[0]) as usize),
            geom.get_point((offset + indices[1]) as usize),
            geom.get_point((offset + indices[2]) as usize),
        );
        let flip_winding = !winding;
        for i in (0..indices.len()).step_by(3) {
            let i0 = offset + indices[i];
            let i1 = offset + indices[i + 1];
            let i2 = offset + indices[i + 2];
            if flip_winding {
                geom.add_face_indices(i0, i2, i1, u32::MAX);
            } else {
                geom.add_face_indices(i0, i1, i2, u32::MAX);
            }
        }
    }

    offset += geom.num_points;
    normal = -dir;

    for pt in &profile[0] {
        let mut et = *pt;
        if cutting_plane_normal != DVec3::ZERO {
            let trans_dir = dir;
            let ldotn = trans_dir.dot(cutting_plane_normal);
            if ldotn != 0.0 {
                let dpos = cutting_plane_pos - et;
                let dist = dpos.dot(cutting_plane_normal) / ldotn;
                et = et + trans_dir * dist;
            }
        }
        geom.add_point_vec4(et.extend(1.0), normal);
    }

    if indices.len() >= 3 {
        let flip_winding = !get_winding_of_triangle(
            geom.get_point((offset + indices[0]) as usize),
            geom.get_point((offset + indices[1]) as usize),
            geom.get_point((offset + indices[2]) as usize),
        );
        for i in (0..indices.len()).step_by(3) {
            let i0 = offset + indices[i];
            let i1 = offset + indices[i + 1];
            let i2 = offset + indices[i + 2];
            if flip_winding {
                geom.add_face_indices(i0, i1, i2, u32::MAX);
            } else {
                geom.add_face_indices(i0, i2, i1, u32::MAX);
            }
        }
    }

    let cap_size = profile[0].len() as u32;
    for i in 1..cap_size {
        if holes_indices_hash[i as usize] {
            continue;
        }
        let bl = i - 1;
        let br = i;
        let tl = cap_size + i - 1;
        let tr = cap_size + i;
        geom.add_face_points(geom.get_point(tl as usize), geom.get_point(br as usize), geom.get_point(bl as usize), u32::MAX);
        geom.add_face_points(geom.get_point(tl as usize), geom.get_point(tr as usize), geom.get_point(br as usize), u32::MAX);
    }

    geom
}

pub fn project_onto_plane(origin: DVec3, normal: DVec3, point: DVec3, dir: DVec3) -> DVec3 {
    let ldotn = dir.dot(normal);
    if ldotn == 0.0 {
        DVec3::ZERO
    } else {
        let dpos = origin - point;
        let dist = dpos.dot(normal) / ldotn;
        point + dist * dir
    }
}

pub fn sweep_function(
    scaling: f64,
    closed: bool,
    profile_points: &[DVec3],
    directrix: &[DVec3],
    initial_directrix_normal: DVec3,
    rotate90: bool,
    optimize: bool,
) -> Geometry {
    let mut geom = Geometry::default();
    let mut dpts: Vec<DVec3> = Vec::new();

    for i in 0..directrix.len() {
        if i < directrix.len() - 1 {
            if directrix[i].distance(directrix[i + 1]) > EPS_BIG2 * scaling || !optimize {
                dpts.push(directrix[i]);
            }
        } else {
            dpts.push(directrix[i]);
        }
    }

    if closed && dpts.len() >= 2 {
        let dir_start = dpts[dpts.len() - 2] - dpts[dpts.len() - 1];
        let dir_end = dpts[1] - dpts[0];
        let mut new_dpts = Vec::with_capacity(dpts.len() + 2);
        new_dpts.push(dpts[0] + dir_start);
        new_dpts.extend(dpts.iter().copied());
        new_dpts.push(dpts[dpts.len() - 1] + dir_end);
        dpts = new_dpts;
    }

    if dpts.len() <= 1 {
        return geom;
    }

    let mut curves: Vec<Curve> = Vec::new();

    for i in 0..dpts.len() {
        let mut segment = Curve::default();
        let (plane_normal, directrix_segment_normal, plane_origin) = if i == 0 {
            let n = (dpts[1] - dpts[0]).normalize();
            (n, n, dpts[0])
        } else if i == dpts.len() - 1 {
            let n = (dpts[i] - dpts[i - 1]).normalize();
            (n, n, dpts[i])
        } else {
            let mut n1 = (dpts[i] - dpts[i - 1]).normalize();
            let mut n2 = (dpts[i + 1] - dpts[i]).normalize();
            let p = n1.cross(n2).normalize();
            let mut u1 = n1.cross(p).normalize();
            let mut u2 = n2.cross(p).normalize();
            if n1.dot(n2) < -0.9 {
                n2 = -n2;
                u2 = -u2;
            }
            let au = (u1 + u2).normalize();
            let plane_normal = au.cross(p).normalize();
            (plane_normal, n1, dpts[i])
        };

        if curves.is_empty() {
            let (mut left, mut right) = if initial_directrix_normal == DVec3::ZERO {
                let mut left = directrix_segment_normal.cross(DVec3::new(
                    directrix_segment_normal.y,
                    directrix_segment_normal.x,
                    directrix_segment_normal.z,
                ));
                if left == DVec3::ZERO {
                    left = directrix_segment_normal.cross(DVec3::new(
                        directrix_segment_normal.x,
                        directrix_segment_normal.z,
                        directrix_segment_normal.y,
                    ));
                }
                if left == DVec3::ZERO {
                    left = directrix_segment_normal.cross(DVec3::new(
                        directrix_segment_normal.z,
                        directrix_segment_normal.y,
                        directrix_segment_normal.x,
                    ));
                }
                let right = directrix_segment_normal.cross(left).normalize();
                let left = directrix_segment_normal.cross(right).normalize();
                (left, right)
            } else {
                let left = directrix_segment_normal.cross(initial_directrix_normal);
                let side = initial_directrix_normal.normalize();
                let right = directrix_segment_normal.cross(left).normalize();
                let left = directrix_segment_normal.cross(right).normalize();
                (left, right * side)
            };

            for pt2d in profile_points {
                let mut pt = -pt2d.x * left + -pt2d.y * right + plane_origin;
                if rotate90 {
                    pt = -pt2d.x * right - pt2d.y * left + plane_origin;
                }
                let proj = project_onto_plane(plane_origin, plane_normal, pt, directrix_segment_normal);
                segment.add(proj, true);
            }
        } else {
            let prev_curve = curves.last().unwrap();
            for pt in &prev_curve.points {
                let proj = project_onto_plane(plane_origin, plane_normal, *pt, directrix_segment_normal);
                segment.add(proj, true);
            }
        }

        if !closed || (i != 0 && i != dpts.len() - 1) {
            curves.push(segment);
        }
    }

    if closed {
        dpts.pop();
        dpts.remove(0);
    }

    for i in 1..dpts.len() {
        let c1 = &curves[i - 1].points;
        let c2 = &curves[i].points;
        let cap_size = c1.len();
        for j in 1..cap_size {
            let bl = c1[j - 1];
            let br = c1[j];
            let tl = c2[j - 1];
            let tr = c2[j];
            geom.add_face_points(tl, br, bl, u32::MAX);
            geom.add_face_points(tl, tr, br, u32::MAX);
        }
    }

    geom
}

pub fn sweep_circular(
    scaling: f64,
    closed: bool,
    profile: &[DVec3],
    radius: f64,
    directrix: &[DVec3],
    initial_directrix_normal: DVec3,
    rotate90: bool,
) -> Geometry {
    let mut geom = Geometry::default();
    let mut dpts: Vec<DVec3> = Vec::new();

    for i in 0..directrix.len() {
        if i < directrix.len() - 1 {
            if directrix[i].distance(directrix[i + 1]) > EPS_BIG2 * scaling {
                dpts.push(directrix[i]);
            }
        } else {
            dpts.push(directrix[i]);
        }
    }

    if closed && dpts.len() >= 2 {
        let dir_start = dpts[dpts.len() - 2] - dpts[dpts.len() - 1];
        let dir_end = dpts[1] - dpts[0];
        let mut new_dpts = Vec::with_capacity(dpts.len() + 2);
        new_dpts.push(dpts[0] + dir_start);
        new_dpts.extend(dpts.iter().copied());
        new_dpts.push(dpts[dpts.len() - 1] + dir_end);
        dpts = new_dpts;
    }

    if dpts.len() <= 1 {
        return geom;
    }

    let mut curves: Vec<Vec<DVec3>> = Vec::new();

    for i in 0..dpts.len() {
        let mut segment: Vec<DVec3> = Vec::new();

        let (plane_normal, directrix_segment_normal, plane_origin) = if i == 0 {
            let n = (dpts[1] - dpts[0]).normalize();
            (n, n, dpts[0])
        } else if i == dpts.len() - 1 {
            let n = (dpts[i] - dpts[i - 1]).normalize();
            (n, n, dpts[i])
        } else {
            let mut n1 = (dpts[i] - dpts[i - 1]).normalize();
            let mut n2 = (dpts[i + 1] - dpts[i]).normalize();
            let p = n1.cross(n2).normalize();
            let mut u1 = n1.cross(p).normalize();
            let mut u2 = n2.cross(p).normalize();
            if n1.dot(n2) < -0.9 {
                n2 = -n2;
                u2 = -u2;
            }
            let au = (u1 + u2).normalize();
            let plane_normal = au.cross(p).normalize();
            (plane_normal, n1, dpts[i])
        };

        if curves.is_empty() {
            let (left, right) = if initial_directrix_normal == DVec3::ZERO {
                let mut left = directrix_segment_normal.cross(DVec3::new(
                    directrix_segment_normal.y,
                    directrix_segment_normal.x,
                    directrix_segment_normal.z,
                ));
                if left == DVec3::ZERO {
                    left = directrix_segment_normal.cross(DVec3::new(
                        directrix_segment_normal.x,
                        directrix_segment_normal.z,
                        directrix_segment_normal.y,
                    ));
                }
                if left == DVec3::ZERO {
                    left = directrix_segment_normal.cross(DVec3::new(
                        directrix_segment_normal.z,
                        directrix_segment_normal.y,
                        directrix_segment_normal.x,
                    ));
                }
                let right = directrix_segment_normal.cross(left).normalize();
                let left = directrix_segment_normal.cross(right).normalize();
                (left, right)
            } else {
                let left = directrix_segment_normal.cross(initial_directrix_normal);
                let side = initial_directrix_normal.normalize();
                let right = directrix_segment_normal.cross(left).normalize();
                let left = directrix_segment_normal.cross(right).normalize();
                (left, right * side)
            };

            for pt2d in profile {
                let mut pt = -pt2d.x * left + -pt2d.y * right + plane_origin;
                if rotate90 {
                    pt = -pt2d.x * right - pt2d.y * left + plane_origin;
                }
                let proj = project_onto_plane(plane_origin, plane_normal, pt, directrix_segment_normal);
                segment.push(proj);
            }
        } else {
            let prev_curve = curves.last().unwrap();
            for pt in prev_curve {
                let proj = project_onto_plane(plane_origin, plane_normal, *pt, directrix_segment_normal);
                segment.push(proj);
            }
        }

        if !closed || (i != 0 && i != dpts.len() - 1) {
            curves.push(segment);
        }
    }

    if closed {
        dpts.pop();
        dpts.remove(0);
    }

    for i in 1..dpts.len() {
        let c1 = &curves[i - 1];
        let c2 = &curves[i];
        let cap_size = c1.len();
        for j in 1..cap_size {
            let bl = c1[j - 1];
            let br = c1[j];
            let tl = c2[j - 1];
            let tr = c2[j];
            geom.add_face_points(tl, br, bl, u32::MAX);
            geom.add_face_points(tl, tr, br, u32::MAX);
        }
    }

    geom
}

pub fn sectioned_surface(mut profiles: Vec<Vec<DVec3>>, build_caps: bool, _eps: f64) -> Geometry {
    let mut geom = Geometry::default();
    if profiles.is_empty() {
        return geom;
    }
    for profile in &mut profiles {
        if profile.first() != profile.last() {
            if let Some(first) = profile.first().copied() {
                profile.push(first);
            }
        }
    }

    for i in 1..profiles.len() {
        let c1 = &profiles[i - 1];
        let c2 = &profiles[i];
        let cap_size = c1.len();
        for j in 1..cap_size {
            let bl = c1[j - 1];
            let br = c1[j];
            let tl = c2[j - 1];
            let tr = c2[j];
            geom.add_face_points(tl, br, bl, u32::MAX);
            geom.add_face_points(tl, tr, br, u32::MAX);
        }
    }

    if build_caps {
        for profile in profiles {
            for i in 1..profile.len() - 1 {
                geom.add_face_points(profile[0], profile[i], profile[i + 1], u32::MAX);
            }
        }
    }

    geom
}

pub fn get_rectangle_curve(xdim: f64, ydim: f64, placement: DMat4, num_segments: i32, radius: f64) -> Curve {
    let mut curve = Curve::default();
    let half_x = xdim / 2.0;
    let half_y = ydim / 2.0;

    let points = vec![
        DVec3::new(-half_x, -half_y, 0.0),
        DVec3::new(half_x, -half_y, 0.0),
        DVec3::new(half_x, half_y, 0.0),
        DVec3::new(-half_x, half_y, 0.0),
        DVec3::new(-half_x, -half_y, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_i_shaped_curve(
    width: f64,
    depth: f64,
    web_thickness: f64,
    flange_thickness: f64,
    has_fillet: bool,
    fillet_radius: f64,
    placement: DMat4,
) -> Curve {
    let mut curve = Curve::default();
    let half_w = width / 2.0;
    let half_d = depth / 2.0;
    let mut points = vec![
        DVec3::new(-half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d + flange_thickness, 0.0),
        DVec3::new(web_thickness / 2.0, -half_d + flange_thickness, 0.0),
        DVec3::new(web_thickness / 2.0, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d, 0.0),
        DVec3::new(-half_w, half_d, 0.0),
        DVec3::new(-half_w, half_d - flange_thickness, 0.0),
        DVec3::new(-web_thickness / 2.0, half_d - flange_thickness, 0.0),
        DVec3::new(-web_thickness / 2.0, -half_d + flange_thickness, 0.0),
        DVec3::new(-half_w, -half_d + flange_thickness, 0.0),
        DVec3::new(-half_w, -half_d, 0.0),
    ];

    for pt in points.drain(..) {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_c_shaped_curve(
    width: f64,
    depth: f64,
    web_thickness: f64,
    flange_thickness: f64,
    _has_fillet: bool,
    _fillet_radius: f64,
    placement: DMat4,
) -> Curve {
    let mut curve = Curve::default();
    let half_w = width / 2.0;
    let half_d = depth / 2.0;

    let points = vec![
        DVec3::new(-half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d + flange_thickness, 0.0),
        DVec3::new(half_w - web_thickness, -half_d + flange_thickness, 0.0),
        DVec3::new(half_w - web_thickness, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d, 0.0),
        DVec3::new(-half_w, half_d, 0.0),
        DVec3::new(-half_w, -half_d, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_z_shaped_curve(
    width: f64,
    depth: f64,
    web_thickness: f64,
    flange_thickness: f64,
    _has_fillet: bool,
    _fillet_radius: f64,
    placement: DMat4,
) -> Curve {
    let mut curve = Curve::default();
    let half_w = width / 2.0;
    let half_d = depth / 2.0;

    let points = vec![
        DVec3::new(-half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d + flange_thickness, 0.0),
        DVec3::new(-half_w + web_thickness, -half_d + flange_thickness, 0.0),
        DVec3::new(-half_w + web_thickness, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d, 0.0),
        DVec3::new(-half_w, half_d, 0.0),
        DVec3::new(-half_w, -half_d, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_t_shaped_curve(
    width: f64,
    depth: f64,
    thickness: f64,
    _has_fillet: bool,
    _fillet_radius: f64,
    radius: f64,
    slope: f64,
    placement: DMat4,
) -> Curve {
    let mut curve = Curve::default();
    let half_w = width / 2.0;
    let half_d = depth / 2.0;
    let points = vec![
        DVec3::new(-half_w, half_d, 0.0),
        DVec3::new(half_w, half_d, 0.0),
        DVec3::new(half_w, half_d - thickness, 0.0),
        DVec3::new(thickness / 2.0, half_d - thickness, 0.0),
        DVec3::new(thickness / 2.0, -half_d, 0.0),
        DVec3::new(-thickness / 2.0, -half_d, 0.0),
        DVec3::new(-thickness / 2.0, half_d - thickness, 0.0),
        DVec3::new(-half_w, half_d - thickness, 0.0),
        DVec3::new(-half_w, half_d, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_l_shaped_curve(
    width: f64,
    depth: f64,
    thickness: f64,
    _has_fillet: bool,
    _fillet_radius: f64,
    radius: f64,
    slope: f64,
    _num_segments: u16,
    placement: DMat4,
) -> Curve {
    let mut curve = Curve::default();
    let half_w = width / 2.0;
    let half_d = depth / 2.0;
    let points = vec![
        DVec3::new(-half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d + thickness, 0.0),
        DVec3::new(-half_w + thickness, -half_d + thickness, 0.0),
        DVec3::new(-half_w + thickness, half_d, 0.0),
        DVec3::new(-half_w, half_d, 0.0),
        DVec3::new(-half_w, -half_d, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_u_shaped_curve(
    depth: f64,
    flange_width: f64,
    web_thickness: f64,
    flange_thickness: f64,
    _fillet_radius: f64,
    _edge_radius: f64,
    _flange_slope: f64,
    placement: DMat4,
) -> Curve {
    let mut curve = Curve::default();
    let half_d = depth / 2.0;
    let half_w = flange_width / 2.0;

    let points = vec![
        DVec3::new(-half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d, 0.0),
        DVec3::new(half_w, -half_d + flange_thickness, 0.0),
        DVec3::new(web_thickness / 2.0, -half_d + flange_thickness, 0.0),
        DVec3::new(web_thickness / 2.0, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d - flange_thickness, 0.0),
        DVec3::new(half_w, half_d, 0.0),
        DVec3::new(-half_w, half_d, 0.0),
        DVec3::new(-half_w, -half_d, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn get_trapezium_curve(bottom_xdim: f64, top_xdim: f64, y_dim: f64, top_x_offset: f64, placement: DMat4) -> Curve {
    let mut curve = Curve::default();
    let half_bottom = bottom_xdim / 2.0;
    let half_top = top_xdim / 2.0;
    let half_y = y_dim / 2.0;

    let points = vec![
        DVec3::new(-half_bottom, -half_y, 0.0),
        DVec3::new(half_bottom, -half_y, 0.0),
        DVec3::new(half_top + top_x_offset, half_y, 0.0),
        DVec3::new(-half_top + top_x_offset, half_y, 0.0),
        DVec3::new(-half_bottom, -half_y, 0.0),
    ];

    for pt in points {
        let world = (placement * pt.extend(1.0)).truncate();
        curve.add(world, true);
    }

    curve
}

pub fn set_epsilons(tolerance_scalar: f64, plane_refit: f64, boolean_union: f64) {
    super::epsilons::set_epsilons(tolerance_scalar, plane_refit, boolean_union);
}
