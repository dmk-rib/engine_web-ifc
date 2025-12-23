//! Rust port of `web-ifc/geometry/nurbs.h`.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use crate::web_ifc::geometry::operations::bim_geometry::utils::area_of_triangle_2d;
use crate::web_ifc::geometry::representation::geometry::{IfcBound3D, IfcSurface, EPS_TINY};
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;

const ROTATIONS: usize = 6;
const PI2: f64 = PI * 2.0;

pub struct Nurbs<'a> {
    geometry: &'a mut IfcGeometry,
    bounds: &'a [IfcBound3D],
    surface: &'a IfcSurface,
    scaling: f64,
    initialized: bool,
    knots_u: Vec<f64>,
    knots_v: Vec<f64>,
    weights: Vec<f64>,
    control_points: Vec<DVec3>,
    num_u: usize,
    num_v: usize,
    min_error: f64,
    max_error: f64,
    range_knots_u: DVec2,
    range_knots_v: DVec2,
    dh: f64,
    dv: f64,
    pr: f64,
}

impl<'a> Nurbs<'a> {
    pub fn new(
        geometry: &'a mut IfcGeometry,
        bounds: &'a [IfcBound3D],
        surface: &'a IfcSurface,
        scaling: f64,
    ) -> Self {
        let num_u = surface.b_spline_surface.control_points.len();
        let num_v = surface
            .b_spline_surface
            .control_points
            .get(0)
            .map(|row| row.len())
            .unwrap_or(0);

        let mut nurbs = Self {
            geometry,
            bounds,
            surface,
            scaling,
            initialized: false,
            knots_u: Vec::new(),
            knots_v: Vec::new(),
            weights: Vec::new(),
            control_points: Vec::new(),
            num_u,
            num_v,
            min_error: 0.0001,
            max_error: 0.01,
            range_knots_u: DVec2::ZERO,
            range_knots_v: DVec2::ZERO,
            dh: 0.0,
            dv: 0.0,
            pr: 0.0,
        };
        nurbs.init();
        nurbs
    }

    pub fn fill_geometry(&mut self) {
        if !self.initialized {
            return;
        }

        let mut uv_points = self.get_uv_points();
        let mut indices = self.get_triangulation_uv_points(&uv_points);

        for _ in 0..3 {
            let mut new_indices = Vec::with_capacity(indices.len() / 3 * 12);
            let mut new_uv_points = Vec::with_capacity(indices.len() / 3 * 6);

            for tri in indices.chunks(3) {
                if tri.len() != 3 {
                    continue;
                }

                let p0 = uv_points[tri[0] as usize];
                let p1 = uv_points[tri[1] as usize];
                let p2 = uv_points[tri[2] as usize];

                let base = new_uv_points.len() as u32;
                new_uv_points.push(p0);
                new_uv_points.push(p1);
                new_uv_points.push(p2);
                new_uv_points.push((p0 + p1) * 0.5);
                new_uv_points.push((p0 + p2) * 0.5);
                new_uv_points.push((p1 + p2) * 0.5);

                new_indices.extend_from_slice(&[
                    base + 0,
                    base + 3,
                    base + 4,
                    base + 3,
                    base + 5,
                    base + 4,
                    base + 3,
                    base + 1,
                    base + 5,
                    base + 4,
                    base + 5,
                    base + 2,
                ]);
            }

            uv_points = new_uv_points;
            indices = new_indices;
        }

        for tri in indices.chunks(3) {
            if tri.len() != 3 {
                continue;
            }
            let p0 = uv_points[tri[0] as usize];
            let p1 = uv_points[tri[1] as usize];
            let p2 = uv_points[tri[2] as usize];
            let pt00 = self.surface_point(p0.x, p0.y);
            let pt01 = self.surface_point(p1.x, p1.y);
            let pt10 = self.surface_point(p2.x, p2.y);
            self.geometry
                .base
                .base
                .add_face_points(pt00, pt01, pt10, u32::MAX);
        }
    }

    fn init(&mut self) {
        if self.num_u == 0 || self.num_v == 0 {
            return;
        }

        let degree_u = self.surface.b_spline_surface.u_degree;
        let degree_v = self.surface.b_spline_surface.v_degree;
        if degree_u < 0.0 || degree_v < 0.0 {
            return;
        }

        if self.num_u < degree_u as usize + 1 || self.num_v < degree_v as usize + 1 {
            return;
        }

        self.control_points = self.get_control_points();
        self.weights = self.get_weights();
        if self.control_points.len() != self.num_u * self.num_v
            || self.weights.len() != self.num_u * self.num_v
        {
            return;
        }

        self.knots_u = self.get_knots(
            &self.surface.b_spline_surface.u_knots,
            &self.surface.b_spline_surface.u_multiplicity,
        );
        self.knots_v = self.get_knots(
            &self.surface.b_spline_surface.v_knots,
            &self.surface.b_spline_surface.v_multiplicity,
        );

        let degree_u = degree_u as usize;
        let degree_v = degree_v as usize;
        if self.knots_u.len() < degree_u + 2 || self.knots_v.len() < degree_v + 2 {
            return;
        }

        if !is_monotonic(&self.knots_u) || !is_monotonic(&self.knots_v) {
            return;
        }

        self.range_knots_u = DVec2::new(
            self.knots_u[degree_u],
            self.knots_u[self.knots_u.len() - degree_u - 1],
        );
        self.range_knots_v = DVec2::new(
            self.knots_v[degree_v],
            self.knots_v[self.knots_v.len() - degree_v - 1],
        );

        let ptc = self.surface_point(EPS_TINY, EPS_TINY);
        let pth = self.surface_point(1.0, EPS_TINY);
        let ptv = self.surface_point(EPS_TINY, 1.0);

        self.dh = ptc.distance(pth);
        self.dv = ptc.distance(ptv);
        self.pr = (self.dh + 1.0) / (self.dv + 1.0);
        if !self.pr.is_finite() {
            self.pr = 1.0;
        }

        self.min_error /= self.scaling;
        self.max_error /= self.scaling;
        self.initialized = true;
    }

    fn get_weights(&self) -> Vec<f64> {
        let total = self.num_u * self.num_v;
        let mut result = vec![1.0; total];

        if !self.surface.b_spline_surface.weight_points.is_empty() {
            let mut index = 0;
            for row in &self.surface.b_spline_surface.weight_points {
                for weight in row {
                    if index < total {
                        result[index] = *weight;
                    }
                    index += 1;
                }
            }
            return result;
        }

        if !self.surface.b_spline_surface.weights.is_empty() {
            let mut index = 0;
            for row in &self.surface.b_spline_surface.weights {
                for weight in row {
                    if index < total {
                        result[index] = *weight;
                    }
                    index += 1;
                }
            }
        }

        result
    }

    fn get_knots(&self, knots: &[f64], mults: &[u32]) -> Vec<f64> {
        if knots.is_empty() || mults.is_empty() {
            return Vec::new();
        }

        let cleaned = self.check_knots(knots);
        let mut expanded = Vec::new();
        for (knot, mult) in cleaned.iter().zip(mults.iter()) {
            for _ in 0..*mult {
                expanded.push(*knot);
            }
        }
        expanded
    }

    fn get_control_points(&self) -> Vec<DVec3> {
        let mut pts = Vec::new();
        for row in &self.surface.b_spline_surface.control_points {
            pts.extend_from_slice(row);
        }
        pts
    }

    fn get_uv_points(&self) -> Vec<DVec2> {
        if self.bounds.is_empty() {
            return Vec::new();
        }

        let bound_points = &self.bounds[0].curve.base.points;
        if bound_points.is_empty() {
            return Vec::new();
        }

        let mut points: Vec<DVec2> = bound_points
            .iter()
            .map(|point| {
                let uv = self.inverse_evaluation(*point);
                DVec2::new(uv.x, uv.y)
            })
            .collect();

        let eps = 1e-5;
        points.dedup_by(|a, b| (a.x - b.x).abs() < eps && (a.y - b.y).abs() < eps);
        points
    }

    fn get_approximation(
        &self,
        pt: DVec3,
        range_u: DVec2,
        range_v: DVec2,
    ) -> (f64, f64, f64, DVec2, DVec2) {
        let mut f_u = 0.0;
        let mut f_v = 0.0;
        let grid_size = 10;
        let mut min_distance = f64::MAX;
        let portion_u = ((range_u.y - range_u.x) / grid_size as f64).abs();
        let portion_v = ((range_v.y - range_v.x) / grid_size as f64).abs();
        let mut new_range_u = range_u;
        let mut new_range_v = range_v;

        for i in 0..grid_size {
            let step_u = portion_u * i as f64;
            let u = if range_u.x + step_u != 0.0 {
                step_u
            } else {
                step_u + f64::EPSILON
            };
            for j in 0..grid_size {
                let step_v = portion_v * i as f64;
                let v = if range_v.x + step_v != 0.0 {
                    step_v
                } else {
                    step_v + f64::EPSILON
                };
                let pt_grid = self.surface_point(u, v);
                let dist = pt_grid.distance(pt);
                if dist < min_distance {
                    min_distance = dist;
                    f_u = u;
                    f_v = v;
                    new_range_u = DVec2::new(
                        range_u.x + portion_u * i as f64,
                        range_u.x + portion_u * (i + 1) as f64,
                    );
                    new_range_v = DVec2::new(
                        range_v.x + portion_v * j as f64,
                        range_v.x + portion_v * (j + 1) as f64,
                    );
                }
            }
        }

        (min_distance, f_u, f_v, new_range_u, new_range_v)
    }

    fn inverse_evaluation(&self, pt: DVec3) -> DVec2 {
        self.inverse_method(pt)
    }

    fn inverse_method(&self, pt: DVec3) -> DVec2 {
        let mut f_u = 0.5;
        let mut f_v = 0.5;
        let mut divisor = 100.0;
        let mut max_distance = f64::MAX;

        while max_distance > self.max_error && divisor < 10000.0 {
            for r in 1..5 {
                let mut round = 0;
                let mul_divisor = (r * r) as f64 * divisor;
                while max_distance > self.min_error && round < 3 {
                    for i in 0..ROTATIONS {
                        let rads = (i as f64 / ROTATIONS as f64) * PI2;
                        let mut inc_u = rads.sin() / mul_divisor;
                        let mut inc_v = rads.cos() / mul_divisor;
                        if self.pr > 1.0 {
                            inc_v *= self.pr;
                        } else {
                            inc_u /= self.pr;
                        }
                        loop {
                            let mut ff_u = f_u + inc_u;
                            let mut ff_v = f_v + inc_v;
                            if ff_u < self.range_knots_u.x {
                                ff_u = self.range_knots_u.y - (self.range_knots_u.x - ff_u);
                            } else if ff_u > self.range_knots_u.y {
                                ff_u = self.range_knots_u.x + (ff_u - self.range_knots_u.y);
                            }
                            if ff_v < self.range_knots_v.x {
                                ff_v = self.range_knots_v.y - (self.range_knots_v.x - ff_v);
                            } else if ff_v > self.range_knots_v.y {
                                ff_v = self.range_knots_v.x + (ff_v - self.range_knots_v.y);
                            }

                            let pt00 = self.surface_point(ff_u, ff_v);
                            let di = pt00.distance(pt);
                            if di < max_distance {
                                max_distance = di;
                                f_u = ff_u;
                                f_v = ff_v;
                            } else {
                                break;
                            }
                        }
                    }
                    round += 1;
                }
            }
            divisor *= 3.0;
        }

        DVec2::new(f_u, f_v)
    }

    fn get_triangulation_uv_points(&self, uv_points: &[DVec2]) -> Vec<u32> {
        if uv_points.len() < 3 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity((uv_points.len() - 2) * 3);
        for i in 1..uv_points.len() - 1 {
            let a = uv_points[0];
            let b = uv_points[i];
            let c = uv_points[i + 1];
            let area = area_of_triangle_2d(a, b, c);
            let eps = 1e-2;
            if area < eps * eps {
                continue;
            }
            result.push(0);
            result.push(i as u32);
            result.push((i + 1) as u32);
        }
        result
    }

    fn get_zscores(&self, knots: &[f64]) -> Vec<f64> {
        if knots.is_empty() {
            return Vec::new();
        }

        let mean = knots.iter().sum::<f64>() / knots.len() as f64;
        let sq_sum = knots.iter().map(|k| k * k).sum::<f64>();
        let variance = (sq_sum / knots.len() as f64) - mean * mean;
        let stdev = variance.max(0.0).sqrt();
        if stdev == 0.0 {
            return vec![0.0; knots.len()];
        }

        knots.iter().map(|k| (k - mean) / stdev).collect()
    }

    fn check_knots(&self, knots: &[f64]) -> Vec<f64> {
        if knots.len() == 2 {
            return vec![0.0, 1.0];
        }

        let threshold = 3.0;
        let zscores = self.get_zscores(knots);
        let mut result = Vec::with_capacity(knots.len());
        for (i, knot) in knots.iter().enumerate() {
            if i < zscores.len() && zscores[i].abs() > threshold {
                if i == 0 {
                    result.push(knots[i + 1]);
                } else if i == knots.len() - 1 {
                    result.push(knots[i - 1]);
                } else {
                    result.push((knots[i - 1] + knots[i + 1]) / 2.0);
                }
            } else {
                result.push(*knot);
            }
        }
        result
    }

    fn surface_point(&self, u: f64, v: f64) -> DVec3 {
        surface_point_from_surface(
            self.surface,
            &self.knots_u,
            &self.knots_v,
            &self.weights,
            &self.control_points,
            self.num_u,
            self.num_v,
            u,
            v,
        )
    }
}

fn is_monotonic(knots: &[f64]) -> bool {
    knots.windows(2).all(|pair| pair[1] >= pair[0])
}

fn basis_function(i: usize, degree: usize, t: f64, knots: &[f64]) -> f64 {
    if degree == 0 {
        if knots[i] <= t && t < knots[i + 1] {
            return 1.0;
        }
        return 0.0;
    }

    let mut term1 = 0.0;
    let denom1 = knots[i + degree] - knots[i];
    if denom1 != 0.0 {
        term1 = (t - knots[i]) / denom1 * basis_function(i, degree - 1, t, knots);
    }

    let mut term2 = 0.0;
    let denom2 = knots[i + degree + 1] - knots[i + 1];
    if denom2 != 0.0 {
        term2 = (knots[i + degree + 1] - t) / denom2 * basis_function(i + 1, degree - 1, t, knots);
    }

    term1 + term2
}

fn surface_point_from_surface(
    surface: &IfcSurface,
    knots_u: &[f64],
    knots_v: &[f64],
    weights: &[f64],
    control_points: &[DVec3],
    num_u: usize,
    num_v: usize,
    u: f64,
    v: f64,
) -> DVec3 {
    let p = surface.b_spline_surface.u_degree.max(0.0) as usize;
    let q = surface.b_spline_surface.v_degree.max(0.0) as usize;
    let mut numerator = DVec3::ZERO;
    let mut denom = 0.0;

    for i in 0..num_u {
        let bu = basis_function(i, p, u, knots_u);
        for j in 0..num_v {
            let bv = basis_function(j, q, v, knots_v);
            let weight = weights.get(i * num_v + j).copied().unwrap_or(1.0);
            let basis = bu * bv * weight;
            if let Some(control) = control_points.get(i * num_v + j) {
                numerator += *control * basis;
            }
            denom += basis;
        }
    }

    if denom == 0.0 {
        numerator
    } else {
        numerator / denom
    }
}

pub fn inverse_evaluation(surface: &IfcSurface, pt: DVec3) -> DVec2 {
    let num_u = surface.b_spline_surface.control_points.len();
    let num_v = surface
        .b_spline_surface
        .control_points
        .get(0)
        .map(|row| row.len())
        .unwrap_or(0);
    if num_u == 0 || num_v == 0 {
        return DVec2::ZERO;
    }

    let knots_u = surface
        .b_spline_surface
        .u_knots
        .iter()
        .zip(surface.b_spline_surface.u_multiplicity.iter())
        .flat_map(|(knot, mult)| std::iter::repeat(*knot).take(*mult as usize))
        .collect::<Vec<_>>();
    let knots_v = surface
        .b_spline_surface
        .v_knots
        .iter()
        .zip(surface.b_spline_surface.v_multiplicity.iter())
        .flat_map(|(knot, mult)| std::iter::repeat(*knot).take(*mult as usize))
        .collect::<Vec<_>>();

    if knots_u.is_empty() || knots_v.is_empty() {
        return DVec2::ZERO;
    }

    let mut control_points = Vec::new();
    for row in &surface.b_spline_surface.control_points {
        control_points.extend_from_slice(row);
    }

    let mut weights = Vec::new();
    if !surface.b_spline_surface.weight_points.is_empty() {
        for row in &surface.b_spline_surface.weight_points {
            weights.extend_from_slice(row);
        }
    }
    if weights.len() != control_points.len() {
        weights = vec![1.0; control_points.len()];
    }

    let mut best_uv = DVec2::ZERO;
    let mut min_distance = f64::MAX;
    let steps = 16;
    let u_min = *knots_u.first().unwrap_or(&0.0);
    let u_max = *knots_u.last().unwrap_or(&1.0);
    let v_min = *knots_v.first().unwrap_or(&0.0);
    let v_max = *knots_v.last().unwrap_or(&1.0);

    for i in 0..=steps {
        let u = u_min + (u_max - u_min) * (i as f64 / steps as f64);
        for j in 0..=steps {
            let v = v_min + (v_max - v_min) * (j as f64 / steps as f64);
            let candidate = surface_point_from_surface(
                surface,
                &knots_u,
                &knots_v,
                &weights,
                &control_points,
                num_u,
                num_v,
                u,
                v,
            );
            let dist = candidate.distance(pt);
            if dist < min_distance {
                min_distance = dist;
                best_uv = DVec2::new(u, v);
            }
        }
    }

    best_uv
}
