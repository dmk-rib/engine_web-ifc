//! Rust port of `web-ifc/geometry/nurbs.h`.

use glam::{DVec2, DVec3};

use crate::web_ifc::geometry::representation::geometry::{IfcBound3D, IfcSurface};
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;

const DEFAULT_STEPS: usize = 16;

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
        };
        nurbs.init();
        nurbs
    }

    pub fn fill_geometry(&mut self) {
        if !self.initialized {
            return;
        }

        let steps_u = (self.num_u.max(2) * 2).max(DEFAULT_STEPS);
        let steps_v = (self.num_v.max(2) * 2).max(DEFAULT_STEPS);

        let u_min = *self.knots_u.first().unwrap_or(&0.0);
        let u_max = *self.knots_u.last().unwrap_or(&1.0);
        let v_min = *self.knots_v.first().unwrap_or(&0.0);
        let v_max = *self.knots_v.last().unwrap_or(&1.0);

        let mut grid: Vec<Vec<DVec3>> = vec![vec![DVec3::ZERO; steps_v + 1]; steps_u + 1];
        for i in 0..=steps_u {
            let u = u_min + (u_max - u_min) * (i as f64 / steps_u as f64);
            for j in 0..=steps_v {
                let v = v_min + (v_max - v_min) * (j as f64 / steps_v as f64);
                grid[i][j] = self.surface_point(u, v);
            }
        }

        for i in 0..steps_u {
            for j in 0..steps_v {
                let p00 = grid[i][j];
                let p10 = grid[i + 1][j];
                let p01 = grid[i][j + 1];
                let p11 = grid[i + 1][j + 1];
                self.geometry.base.base.add_face_points(p00, p10, p11, u32::MAX);
                self.geometry.base.base.add_face_points(p00, p11, p01, u32::MAX);
            }
        }

        let _ = self.bounds;
    }

    fn init(&mut self) {
        if self.num_u == 0 || self.num_v == 0 {
            return;
        }

        self.control_points = self.get_control_points();
        self.weights = self.get_weights();
        self.knots_u = self.get_knots(
            &self.surface.b_spline_surface.u_knots,
            &self.surface.b_spline_surface.u_multiplicity,
            self.surface.b_spline_surface.u_degree as usize,
            self.num_u,
        );
        self.knots_v = self.get_knots(
            &self.surface.b_spline_surface.v_knots,
            &self.surface.b_spline_surface.v_multiplicity,
            self.surface.b_spline_surface.v_degree as usize,
            self.num_v,
        );

        self.initialized = true;
        let _ = self.scaling;
    }

    fn get_weights(&self) -> Vec<f64> {
        if !self.surface.b_spline_surface.weight_points.is_empty() {
            let mut weights = Vec::new();
            for row in &self.surface.b_spline_surface.weight_points {
                for w in row {
                    weights.push(*w);
                }
            }
            if weights.len() == self.num_u * self.num_v {
                return weights;
            }
        }

        vec![1.0; self.num_u * self.num_v]
    }

    fn get_knots(
        &self,
        knots: &[f64],
        mults: &[u32],
        degree: usize,
        num_ctrl: usize,
    ) -> Vec<f64> {
        if !knots.is_empty() && !mults.is_empty() {
            let mut expanded = Vec::new();
            for (knot, mult) in knots.iter().zip(mults.iter()) {
                for _ in 0..*mult {
                    expanded.push(*knot);
                }
            }
            if expanded.len() >= degree + num_ctrl + 1 {
                return expanded;
            }
        }

        let m = degree + num_ctrl + 1;
        let mut uniform = Vec::with_capacity(m);
        for i in 0..m {
            uniform.push(i as f64 / (m - 1) as f64);
        }
        uniform
    }

    fn get_control_points(&self) -> Vec<DVec3> {
        let mut pts = Vec::new();
        for row in &self.surface.b_spline_surface.control_points {
            for pt in row {
                pts.push(*pt);
            }
        }
        pts
    }

    fn basis_function(&self, i: usize, degree: usize, t: f64, knots: &[f64]) -> f64 {
        if degree == 0 {
            if knots[i] <= t && t < knots[i + 1] {
                return 1.0;
            }
            return 0.0;
        }

        let mut term1 = 0.0;
        let denom1 = knots[i + degree] - knots[i];
        if denom1 != 0.0 {
            term1 = (t - knots[i]) / denom1 * self.basis_function(i, degree - 1, t, knots);
        }

        let mut term2 = 0.0;
        let denom2 = knots[i + degree + 1] - knots[i + 1];
        if denom2 != 0.0 {
            term2 = (knots[i + degree + 1] - t) / denom2 * self.basis_function(i + 1, degree - 1, t, knots);
        }

        term1 + term2
    }

    fn surface_point(&self, u: f64, v: f64) -> DVec3 {
        let p = self.surface.b_spline_surface.u_degree as usize;
        let q = self.surface.b_spline_surface.v_degree as usize;
        let mut numerator = DVec3::ZERO;
        let mut denom = 0.0;

        for i in 0..self.num_u {
            let bu = self.basis_function(i, p, u, &self.knots_u);
            for j in 0..self.num_v {
                let bv = self.basis_function(j, q, v, &self.knots_v);
                let weight = self.weights[i * self.num_v + j];
                let basis = bu * bv * weight;
                numerator += self.control_points[i * self.num_v + j] * basis;
                denom += basis;
            }
        }

        if denom == 0.0 {
            numerator
        } else {
            numerator / denom
        }
    }
}

pub fn inverse_evaluation(_surface: &IfcSurface, _pt: DVec3) -> DVec2 {
    DVec2::ZERO
}

