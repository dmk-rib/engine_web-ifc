//! Rust port of boolean-utils epsilons.

#![allow(dead_code)]

pub static mut _TOLERANCE_PLANE_INTERSECTION: f64 = 1.0E-04;
pub static mut _TOLERANCE_PLANE_DEVIATION: f64 = 1.0E-04;
pub static mut _TOLERANCE_BACK_DEVIATION_DISTANCE: f64 = 1.0E-04;
pub static mut _TOLERANCE_INSIDE_OUTSIDE_PERIMETER: f64 = 1.0E-10;
pub static mut _TOLERANCE_BOUNDING_BOX: f64 = 1.0E-02;

pub static mut _BOOLSTATUS: f64 = 1.0E-10;

pub const MESSAGES: bool = false;

pub const EPS_NONZERO: f64 = 1.0E-20;
pub const EPS_MINISCULE: f64 = 1.0E-12;
pub const EPS_TINY: f64 = 1.0E-04;
pub const EPS_SMALL: f64 = 1.0E-04;
pub const EPS_BIG: f64 = 1.0E-04;
pub const EPS_BIG2: f64 = 1.0E-03;
pub const SCALED_EPS_BIG: f64 = 1.0E-04;

pub const TOLERANCE_IS_INSIDE_CENTER_EXTENTS: f64 = 1.0E-10;

pub const TOLERANCE_VECTOR_EQUALITY: f64 = 1.0E-04;
pub const TOLERANCE_SCALAR_EQUALITY: f64 = 1.0E-04;
pub const TOLERANCE_COLLINEAR: f64 = 1.0E-04;
pub const TRIANGLE_EVALUATION_FACTOR: f64 = 0.90;

pub const TOLERANCE_POINT_ON_LINE: f64 = 1.0E-04;

pub const TOLERANCE_AABB: f64 = 1.0E-03;

pub const TOLERANCE_PARALLEL: f64 = 1.0E-04;
pub const TOLERANCE_PARALLEL_TIGHT: f64 = 1.0E-10;
pub const TOLERANCE_THIN_TRIANGLE: f64 = 1.0E-10;

pub const TOLERANCE_ADD_FACE: f64 = 1.0E-10;

pub const ROUNDING_ENABLE: i32 = 0;
pub const ROUNDING: f64 = 1.0E-04;
pub const ROUNDING_RECIPROCAL: f64 = 1.0E+04;
