//! Rust port of `web-ifc/geometry/operations/boolean-utils/math.h`.

use glam::{DMat3, DMat4, DVec2, DVec3};

use super::eps::{
    EPS_SMALL, MESSAGES, TOLERANCE_IS_INSIDE_CENTER_EXTENTS, TOLERANCE_SCALAR_EQUALITY,
    TOLERANCE_VECTOR_EQUALITY, _TOLERANCE_PLANE_DEVIATION,
};

pub type Vec = DVec3;

pub fn compute_safe_normal(v1: DVec3, v2: DVec3, v3: DVec3, normal: &mut DVec3, eps: f64) -> bool {
    let v12 = v2 - v1;
    let v13 = v3 - v1;
    let norm = v12.cross(v13);
    let len = norm.length();
    if len.is_nan() || len <= eps {
        return false;
    }
    *normal = norm / len;
    true
}

pub fn compute_normal(v1: DVec3, v2: DVec3, v3: DVec3) -> DVec3 {
    let v12 = v2 - v1;
    let v13 = v3 - v1;
    v12.cross(v13).normalize()
}

pub fn is_inside_center_extents(pt: DVec3, center: DVec3, extents: DVec3) -> bool {
    let mut delta = pt - center;
    delta = delta.abs();
    let offset = delta - extents;
    offset.x < TOLERANCE_IS_INSIDE_CENTER_EXTENTS
        && offset.y < TOLERANCE_IS_INSIDE_CENTER_EXTENTS
        && offset.z < TOLERANCE_IS_INSIDE_CENTER_EXTENTS
}

pub fn area_of_triangle(a: DVec3, b: DVec3, c: DVec3) -> f64 {
    let ab = b - a;
    let ac = c - a;
    ab.cross(ac).length() / 2.0
}

pub fn matrix_flips_triangles_mat4(mat: DMat4) -> bool {
    mat.determinant() < 0.0
}

pub fn matrix_flips_triangles_mat3(mat: DMat3) -> bool {
    mat.determinant() < 0.0
}

pub fn equals2d(a: DVec2, b: DVec2, eps: f64) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps
}

pub fn equals_vec3(a: DVec3, b: DVec3, eps: f64) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps
}

pub fn equals(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

pub fn sign2d(p: DVec2, a: DVec2, b: DVec2) -> f64 {
    (p.x - b.x) * (a.y - b.y) - (a.x - b.x) * (p.y - b.y)
}

pub fn cross2d(point1: DVec2, point2: DVec2) -> f64 {
    point1.x * point2.y - point1.y * point2.x
}

pub fn sign_one_zero(x: f64) -> f64 {
    (x > 0.0) as i32 as f64 - (x < 0.0) as i32 as f64
}

pub fn comparable_angle(p: DVec2, a: DVec2, b: DVec2) -> f64 {
    let up_down = if sign2d(p, a, b) >= 0.0 { 1.0 } else { -1.0 };
    let dot = (1.0 + (a - b).normalize().dot((p - b).normalize())) / 2.0;
    up_down * dot
}

pub fn all_equal(b1: bool, b2: bool, b3: bool, b4: bool) -> bool {
    b1 == b2 && b1 == b3 && b1 == b4
}

#[derive(Clone, Debug, Default)]
pub struct LineLineIsect2D {
    pub isect: bool,
    pub pt: DVec2,
    pub dist: f64,
}

pub fn do_line_segments_intersect(
    p: DVec2,
    p2: DVec2,
    q: DVec2,
    q2: DVec2,
    eps: f64,
) -> LineLineIsect2D {
    let mut result = LineLineIsect2D::default();

    let r = p2 - p;
    let s = q2 - q;

    let u_numerator = cross2d(q - p, r);
    let denominator = cross2d(r, s);

    if u_numerator == 0.0 && denominator == 0.0 {
        if equals2d(p, q, eps)
            || equals2d(p, q2, eps)
            || equals2d(p2, q, eps)
            || equals2d(p2, q2, eps)
        {
            result.isect = true;
            return result;
        }

        result.isect = !all_equal(
            q.x - p.x < 0.0,
            q.x - p2.x < 0.0,
            q2.x - p.x < 0.0,
            q2.x - p2.x < 0.0,
        ) || !all_equal(
            q.y - p.y < 0.0,
            q.y - p2.y < 0.0,
            q2.y - p.y < 0.0,
            q2.y - p2.y < 0.0,
        );
        return result;
    }

    if denominator == 0.0 {
        result.isect = false;
        return result;
    }

    let u = u_numerator / denominator;
    let t = cross2d(q - p, s) / denominator;

    result.isect = (t >= -eps) && (t <= 1.0 + eps) && (u >= -eps) && (u <= 1.0 + eps);
    result.pt = p + r * t;
    result.dist = p.distance(result.pt);

    result
}

#[derive(Clone, Debug, Default)]
pub struct LineLineIsect {
    pub param1: f64,
    pub param2: f64,
    pub point1: DVec3,
    pub point2: DVec3,
    pub distance: f64,
}

pub fn line_line_intersection(p0: DVec3, p1: DVec3, q0: DVec3, q1: DVec3) -> LineLineIsect {
    let p1mp0 = p1 - p0;
    let q1mq0 = q1 - q0;
    let p0mq0 = p0 - q0;
    let a = p1mp0.dot(p1mp0);
    let b = p1mp0.dot(q1mq0);
    let c = q1mq0.dot(q1mq0);
    let d = p1mp0.dot(p0mq0);
    let e = q1mq0.dot(p0mq0);
    let det = a * c - b * b;
    let zero = 0.0;
    let one = 1.0;
    let (mut s, mut t);
    let mut nd;
    let mut bmd;
    let mut bte;
    let mut ctd;
    let mut bpe;
    let mut ate;
    let mut btd;

    if det > zero {
        bte = b * e;
        ctd = c * d;
        if bte <= ctd {
            s = zero;
            if e <= zero {
                t = zero;
                nd = -d;
                if nd >= a {
                    s = one;
                } else if nd > zero {
                    s = nd / a;
                }
            } else if e < c {
                t = e / c;
            } else {
                t = one;
                bmd = b - d;
                if bmd >= a {
                    s = one;
                } else if bmd > zero {
                    s = bmd / a;
                }
            }
        } else {
            s = bte - ctd;
            if s >= det {
                s = one;
                bpe = b + e;
                if bpe <= zero {
                    t = zero;
                    nd = -d;
                    if nd <= zero {
                        s = zero;
                    } else if nd < a {
                        s = nd / a;
                    }
                } else if bpe < c {
                    t = bpe / c;
                } else {
                    t = one;
                    bmd = b - d;
                    if bmd <= zero {
                        s = zero;
                    } else if bmd < a {
                        s = bmd / a;
                    }
                }
            } else {
                ate = a * e;
                btd = b * d;
                if ate <= btd {
                    t = zero;
                    nd = -d;
                    if nd <= zero {
                        s = zero;
                    } else if nd >= a {
                        s = one;
                    } else {
                        s = nd / a;
                    }
                } else {
                    t = ate - btd;
                    if t >= det {
                        t = one;
                        bmd = b - d;
                        if bmd <= zero {
                            s = zero;
                        } else if bmd >= a {
                            s = one;
                        } else {
                            s = bmd / a;
                        }
                    } else {
                        s /= det;
                        t /= det;
                    }
                }
            }
        }
    } else if e <= zero {
        t = zero;
        nd = -d;
        if nd <= zero {
            s = zero;
        } else if nd >= a {
            s = one;
        } else {
            s = nd / a;
        }
    } else if e >= c {
        t = one;
        bmd = b - d;
        if bmd <= zero {
            s = zero;
        } else if bmd >= a {
            s = one;
        } else {
            s = bmd / a;
        }
    } else {
        s = zero;
        t = e / c;
    }

    let mut result = LineLineIsect::default();
    result.param1 = s;
    result.param2 = t;
    result.point1 = p0 + p1mp0 * s;
    result.point2 = q0 + q1mq0 * t;

    let diff = result.point1 - result.point2;
    let total_dist = diff.dot(diff).sqrt();
    if total_dist < 1e-12 {
        result.distance = total_dist;
        return result;
    }

    if total_dist > TOLERANCE_SCALAR_EQUALITY {
        result.distance = total_dist;
        return result;
    }

    let v1 = p1mp0.normalize();
    let v2 = q1mq0.normalize();
    if v1.dot(v2).abs() > 1.0 - unsafe { _TOLERANCE_PLANE_DEVIATION } {
        result.distance = total_dist;
        return result;
    }

    let normal = p1mp0.cross(q1mq0);
    let normal_length = normal.length();
    let normal = normal / normal_length;
    let perp_dist = (diff.dot(normal)).abs();
    let mut subs = total_dist * total_dist - perp_dist * perp_dist;
    if subs < 0.0 {
        subs = 0.0;
    }
    result.distance = subs.sqrt();
    result
}

#[derive(Clone, Debug, Default)]
pub struct PlanePlaneIsectResult {
    pub pos: DVec3,
    pub dir: DVec3,
}

pub fn plane_plane_isect(norm1: DVec3, d1: f64, norm2: DVec3, d2: f64) -> PlanePlaneIsectResult {
    let mut result = PlanePlaneIsectResult::default();
    result.dir = norm1.cross(norm2);
    let mut det = result.dir.length();
    let _det2 = DMat3::from_cols(norm1, norm2, result.dir).determinant();
    det *= det;
    result.pos = (result.dir.cross(norm2) * -d1 + norm1.cross(result.dir) * -d2) / det;
    result.dir = result.dir.normalize();
    result
}

pub fn distance_point_to_line_segment_2d(v: DVec2, w: DVec2, p: DVec2) -> f64 {
    let l2 = (v - w).length();
    if l2 == 0.0 {
        return p.distance(v);
    }
    let t = (p - v).dot(w - v) / (l2 * l2);
    let t = t.clamp(0.0, 1.0);
    let projection = v + (w - v) * t;
    p.distance(projection)
}

pub fn point_on_line_segment_2d(v: DVec2, w: DVec2, p: DVec2, eps: f64) -> bool {
    distance_point_to_line_segment_2d(v, w, p) <= eps
}

pub fn on_edge_2d(p: DVec2, a: DVec2, b: DVec2, eps: f64) -> bool {
    sign2d(p, a, b).abs() <= eps
}

pub fn is_vector_ccw(points: &[DVec2]) -> bool {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let pt1 = points[i];
        let pt2 = points[(i + 1) % points.len()];
        sum += (pt2.x - pt1.x) * (pt2.y + pt1.y);
    }
    sum < 0.0
}

pub fn area_of_triangle_2d(a: DVec2, b: DVec2, c: DVec2) -> f64 {
    let ab = b - a;
    let ac = c - a;
    (cross2d(ab, ac) / 2.0).abs()
}

pub fn to_bary(a: DVec3, b: DVec3, c: DVec3, pt: DVec3) -> DVec3 {
    let e1 = b - a;
    let e2 = c - a;
    let rov0 = pt - a;
    let n = e1.cross(e2);
    let dir = -n;
    let q = rov0.cross(dir);
    let d = dir.dot(n);

    if d == 0.0 && MESSAGES {
        println!("bary conversion perp");
    }

    let det = 1.0 / d;
    let u = det * e2.dot(q * -1.0);
    let v = det * e1.dot(q);
    let w = 1.0 - u - v;

    DVec3::new(w, u, v)
}

pub fn from_bary_2d(a: DVec2, b: DVec2, c: DVec2, pt: DVec3) -> DVec2 {
    pt.x * a + pt.y * b + pt.z * c
}

pub fn to_bary2(pt: DVec2) -> DVec3 {
    let v = pt.x;
    let w = pt.y;
    let u = 1.0 - v - w;
    DVec3::new(u, v, w)
}

pub fn from_bary_3d(a: DVec3, b: DVec3, c: DVec3, pt: DVec3) -> DVec3 {
    pt.x * a + pt.y * b + pt.z * c
}

pub fn random_double(lo: f64, hi: f64) -> f64 {
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEED: AtomicU64 = AtomicU64::new(0x1234_5678_9abc_def0);
    let mut x = SEED.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    SEED.store(x, Ordering::Relaxed);
    let rand = (x as f64) / (u64::MAX as f64);
    lo + rand * (hi - lo)
}

pub fn dot(a: DVec2, b: DVec2) -> f64 {
    a.dot(b)
}

pub fn safe_dot(a: DVec3, b: DVec3) -> f64 {
    a.dot(b)
}

pub fn equals_vec3_tolerance(a: DVec3, b: DVec3) -> bool {
    equals_vec3(a, b, TOLERANCE_VECTOR_EQUALITY)
}
