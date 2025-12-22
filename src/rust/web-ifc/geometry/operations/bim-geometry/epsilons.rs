//! Epsilon constants and tunables for bim-geometry.

pub static mut TOLERANCE_SCALAR_EQUALITY: f64 = 1.0E-04;
pub static mut PLANE_REFIT_ITERATIONS: f64 = 1.0;
pub static mut BOOLEAN_UNION_THRESHOLD: f64 = 150.0;

pub const EPS_TINY_CURVE: f64 = 1.0E-09;
pub const EPS_NONZERO: f64 = 1.0E-20;
pub const EPS_MINISCULE: f64 = 1.0E-12;
pub const EPS_TINY: f64 = 1.0E-04;
pub const EPS_SMALL: f64 = 1.0E-04;
pub const EPS_BIG: f64 = 1.0E-04;
pub const EPS_BIG2: f64 = 1.0E-03;
pub const SCALED_EPS_BIG: f64 = 1.0E-04;

pub const TOLERANCE_ADD_FACE: f64 = 1.0E-10;
pub const TOLERANCE_VECTOR_EQUALITY: f64 = 1.0E-04;
pub const RECONSTRUCT_TOLERANCE: f64 = 1.0E-01;

#[inline]
pub fn set_epsilons(tol_scalar: f64, plane_refit: f64, boolean_union: f64) {
    unsafe {
        TOLERANCE_SCALAR_EQUALITY = tol_scalar;
        PLANE_REFIT_ITERATIONS = plane_refit;
        BOOLEAN_UNION_THRESHOLD = boolean_union;
    }
}
