//! Rust port of `web-ifc/geometry/operations/curve-utils.h`.

use glam::{DMat3, DVec2, DVec3};

use crate::web_ifc::geometry::operations::bim_geometry::utils::CONST_PI;
use crate::web_ifc::geometry::representation::ifc_curve::IfcCurve;

pub fn is_convex_or_colinear(a: DVec2, b: DVec2, c: DVec2) -> bool {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) >= 0.0
}

pub fn build_3d_arc_3pt(p1: DVec3, p2: DVec3, p3: DVec3, circle_segments: u16, eps_minsize: f64) -> IfcCurve {
    let v1 = p2 - p1;
    let v2 = p3 - p1;
    if v1.cross(v2).length() < eps_minsize {
        return IfcCurve::default();
    }

    let cx = p2.x - p1.x;
    let cy = p2.y - p1.y;
    let cz = p2.z - p1.z;
    let bx = p3.x - p1.x;
    let by = p3.y - p1.y;
    let bz = p3.z - p1.z;
    let b2 = p1.x * p1.x - p3.x * p3.x + p1.y * p1.y - p3.y * p3.y + p1.z * p1.z - p3.z * p3.z;
    let c2 = p1.x * p1.x - p2.x * p2.x + p1.y * p1.y - p2.y * p2.y + p1.z * p1.z - p2.z * p2.z;

    let cbyz = cy * bz - cz * by;
    let cbxz = cx * bz - cz * bx;
    let cbxy = cx * by - cy * bx;
    let zz1 = -((bz - cz * bx / cx) / (by - cy * bx / cx));
    let z01 = -((b2 - bx / cx * c2) / (2.0 * (by - cy * bx / cx)));
    let zz2 = -((zz1 * cy + cz) / cx);
    let z02 = -((2.0 * z01 * cy + c2) / (2.0 * cx));

    let dz = -((z02 - p1.x) * cbyz - (z01 - p1.y) * cbxz - p1.z * cbxy) / (zz2 * cbyz - zz1 * cbxz + cbxy);
    let dx = zz2 * dz + z02;
    let dy = zz1 * dz + z01;

    let center = DVec3::new(dx, dy, dz);
    let radius = center.distance(p1);

    let mut point_list = vec![p1, p2, p3];
    while point_list.len() < circle_segments as usize {
        let mut temp = Vec::new();
        for j in 0..point_list.len().saturating_sub(1) {
            let mut pt = (point_list[j] + point_list[j + 1]) / 2.0;
            let vc = (pt - center).normalize();
            pt = center + vc * radius;
            temp.push(point_list[j]);
            temp.push(pt);
        }
        temp.push(*point_list.last().unwrap());
        point_list = temp;
    }

    let mut curve = IfcCurve::default();
    for pt in point_list {
        curve.base.add(pt, true);
    }
    curve
}

pub fn build_arc_3pt(p1: DVec2, p2: DVec2, p3: DVec2, circle_segments: u16) -> IfcCurve {
    let f1 = p1.x * p1.x - p2.x * p2.x + p1.y * p1.y - p2.y * p2.y;
    let f2 = p1.x * p1.x - p3.x * p3.x + p1.y * p1.y - p3.y * p3.y;
    let v = 2.0 * (p1.x - p2.x) * (p1.y - p3.y) - 2.0 * (p1.x - p3.x) * (p1.y - p2.y);

    let cen_x = ((p1.y - p3.y) * f1 - (p1.y - p2.y) * f2) / v;
    let den1 = 2.0 * (p1.y - p3.y);
    let den2 = 2.0 * (p1.y - p2.y);
    let cen_ya = (f2 - 2.0 * cen_x * (p1.x - p3.x)) / den1;
    let cen_yb = (f1 - 2.0 * cen_x * (p1.x - p2.x)) / den2;
    let mut cen_y = if den1.abs() > den2.abs() { cen_ya } else { cen_yb };
    if !cen_y.is_finite() {
        if cen_ya.is_finite() {
            cen_y = cen_ya;
        } else if cen_yb.is_finite() {
            cen_y = cen_yb;
        }
    }

    let center = DVec2::new(cen_x, cen_y);
    let radius = ((cen_x - p1.x).powi(2) + (cen_y - p1.y).powi(2)).sqrt();

    let mut point_list = vec![p1, p2, p3];
    while point_list.len() < circle_segments as usize {
        let mut temp = Vec::new();
        for j in 0..point_list.len().saturating_sub(1) {
            let mut pt = (point_list[j] + point_list[j + 1]) / 2.0;
            let vc = (pt - center).normalize();
            pt = center + vc * radius;
            temp.push(point_list[j]);
            temp.push(pt);
        }
        temp.push(*point_list.last().unwrap());
        point_list = temp;
    }

    let mut curve = IfcCurve::default();
    for pt in point_list {
        curve.base.add(DVec3::new(pt.x, pt.y, 0.0), true);
    }
    curve
}

pub fn interpolate_rational_bspline_curve_with_knots(
    t: f64,
    degree: i32,
    points: &[DVec3],
    knots: &[f64],
    weights: &[f64],
) -> DVec3 {
    let domain_low = degree as usize;
    let domain_high = knots.len() - 1 - degree as usize;
    let low = knots[domain_low];
    let high = knots[domain_high];

    let t_prime = t * (high - low) + low;
    if t_prime < low || t_prime > high {
        return DVec3::ZERO;
    }

    let mut s = 0usize;
    for i in domain_low..domain_high {
        if knots[i] <= t_prime && t_prime < knots[i + 1] {
            s = i;
            break;
        }
    }
    if s == 0 {
        s = domain_high - 1;
    }

    let mut v = vec![DVec3::ZERO; degree as usize + 1];
    for i in 0..=degree as usize {
        v[i] = points[s - degree as usize + i] * weights[s - degree as usize + i];
    }

    for l in 1..=degree as usize {
        for i in (l..=degree as usize).rev() {
            let alpha = (t_prime - knots[s - degree as usize + i])
                / (knots[i + s + 1 - l] - knots[s - degree as usize + i]);
            v[i] = v[i - 1] * (1.0 - alpha) + v[i] * alpha;
        }
    }

    let mut weight = 0.0;
    for i in 0..=degree as usize {
        let b = if t_prime == knots[s + 1] && s == domain_high - 1 {
            1.0
        } else {
            0.0
        };
        weight += b * weights[s - degree as usize + i];
    }

    if weight == 0.0 {
        v[degree as usize]
    } else {
        v[degree as usize] / weight
    }
}

pub fn interpolate_rational_bspline_curve_with_knots_2d(
    t: f64,
    degree: i32,
    points: &[DVec2],
    knots: &[f64],
    weights: &[f64],
) -> DVec2 {
    let pts3: Vec<DVec3> = points.iter().map(|p| DVec3::new(p.x, p.y, 0.0)).collect();
    let pt = interpolate_rational_bspline_curve_with_knots(t, degree, &pts3, knots, weights);
    DVec2::new(pt.x, pt.y)
}

pub fn get_rational_bspline_curve_with_knots(
    degree: i32,
    points: &[DVec3],
    knots: &[f64],
    weights: &[f64],
    num_curve_points: f64,
) -> Vec<DVec3> {
    let mut curve = Vec::new();
    let num = num_curve_points.max(2.0) as usize;
    for i in 0..num {
        curve.push(interpolate_rational_bspline_curve_with_knots(
            i as f64 / (num - 1) as f64,
            degree,
            points,
            knots,
            weights,
        ));
    }
    curve
}

pub fn get_rational_bspline_curve_with_knots_2d(
    degree: i32,
    points: &[DVec2],
    knots: &[f64],
    weights: &[f64],
) -> Vec<DVec2> {
    let mut curve = Vec::new();
    let num = 64usize;
    for i in 0..num {
        curve.push(interpolate_rational_bspline_curve_with_knots_2d(
            i as f64 / (num - 1) as f64,
            degree,
            points,
            knots,
            weights,
        ));
    }
    curve
}

pub fn is_curve_convex(curve: &IfcCurve) -> bool {
    if curve.base.points.len() < 3 {
        return true;
    }
    for i in 0..curve.base.points.len() {
        let a = curve.base.points[i];
        let b = curve.base.points[(i + 1) % curve.base.points.len()];
        let c = curve.base.points[(i + 2) % curve.base.points.len()];
        if !is_convex_or_colinear(DVec2::new(a.x, a.y), DVec2::new(b.x, b.y), DVec2::new(c.x, c.y)) {
            return false;
        }
    }
    true
}

pub fn matrix_flips_triangles(mat: DMat3) -> bool {
    mat.determinant() < 0.0
}

pub fn get_ellipse_curve(
    radius_x: f64,
    radius_y: f64,
    num_segments: usize,
    placement: DMat3,
    start_rad: f64,
    end_rad: f64,
    swap: bool,
) -> IfcCurve {
    let mut curve = IfcCurve::default();
    let step = (end_rad - start_rad) / (num_segments as f64);
    for i in 0..=num_segments {
        let angle = start_rad + (i as f64) * step;
        let mut pt = if swap {
            DVec3::new(radius_x * angle.cos(), radius_y * angle.sin(), 0.0)
        } else {
            DVec3::new(radius_x * angle.sin(), radius_y * angle.cos(), 0.0)
        };
        pt = placement * pt;
        curve.base.add(pt, true);
    }
    curve
}

pub fn get_circle_curve(radius: f64, num_segments: usize, placement: DMat3) -> IfcCurve {
    get_ellipse_curve(radius, radius, num_segments, placement, 0.0, CONST_PI * 2.0, true)
}

pub fn get_rectangle_curve(xdim: f64, ydim: f64, placement: DMat3) -> IfcCurve {
    let mut curve = IfcCurve::default();
    let pts = [
        DVec3::new(-xdim / 2.0, -ydim / 2.0, 0.0),
        DVec3::new(xdim / 2.0, -ydim / 2.0, 0.0),
        DVec3::new(xdim / 2.0, ydim / 2.0, 0.0),
        DVec3::new(-xdim / 2.0, ydim / 2.0, 0.0),
        DVec3::new(-xdim / 2.0, -ydim / 2.0, 0.0),
    ];
    for pt in pts {
        curve.base.add(placement * pt, true);
    }
    curve
}

pub fn get_i_shaped_curve(width: f64, depth: f64) -> IfcCurve {
    get_rectangle_curve(width, depth, DMat3::IDENTITY)
}

pub fn get_u_shaped_curve(depth: f64, flange_width: f64) -> IfcCurve {
    get_rectangle_curve(flange_width, depth, DMat3::IDENTITY)
}

pub fn get_l_shaped_curve(width: f64, depth: f64) -> IfcCurve {
    get_rectangle_curve(width, depth, DMat3::IDENTITY)
}

pub fn get_t_shaped_curve(width: f64, depth: f64) -> IfcCurve {
    get_rectangle_curve(width, depth, DMat3::IDENTITY)
}

pub fn get_c_shaped_curve(width: f64, depth: f64) -> IfcCurve {
    get_rectangle_curve(width, depth, DMat3::IDENTITY)
}

pub fn get_z_shaped_curve(depth: f64, flange_width: f64) -> IfcCurve {
    get_rectangle_curve(flange_width, depth, DMat3::IDENTITY)
}

pub fn get_trapezium_curve(bottom_x_dim: f64, top_x_dim: f64, y_dim: f64, top_x_offset: f64) -> IfcCurve {
    let mut curve = IfcCurve::default();
    let pts = [
        DVec3::new(-bottom_x_dim / 2.0, -y_dim / 2.0, 0.0),
        DVec3::new(bottom_x_dim / 2.0, -y_dim / 2.0, 0.0),
        DVec3::new(top_x_offset + top_x_dim / 2.0, y_dim / 2.0, 0.0),
        DVec3::new(top_x_offset - top_x_dim / 2.0, y_dim / 2.0, 0.0),
        DVec3::new(-bottom_x_dim / 2.0, -y_dim / 2.0, 0.0),
    ];
    for pt in pts {
        curve.base.add(pt, true);
    }
    curve
}

pub fn build_arc(scale: f64, pos: DVec3, axis: DVec3, angle_rad: f64, circle_segments: u16) -> IfcCurve {
    let mut curve = IfcCurve::default();
    let num = circle_segments.max(2) as usize;
    for i in 0..=num {
        let t = angle_rad * (i as f64) / (num as f64);
        let pt = DVec3::new(axis.x * t.sin(), axis.y * t.cos(), axis.z * t.sin()) * scale + pos;
        curve.base.add(pt, true);
    }
    curve
}
