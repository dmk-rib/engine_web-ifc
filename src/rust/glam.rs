#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}

impl DVec2 {
    pub const ZERO: DVec2 = DVec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: DVec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> DVec2 {
        let len = self.length();
        if len == 0.0 {
            Self::ZERO
        } else {
            self / len
        }
    }
}

impl Add for DVec2 {
    type Output = DVec2;

    fn add(self, rhs: DVec2) -> Self::Output {
        DVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for DVec2 {
    type Output = DVec2;

    fn sub(self, rhs: DVec2) -> Self::Output {
        DVec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for DVec2 {
    type Output = DVec2;

    fn mul(self, rhs: f64) -> Self::Output {
        DVec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for DVec2 {
    type Output = DVec2;

    fn div(self, rhs: f64) -> Self::Output {
        DVec2::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3 {
    pub const ZERO: DVec3 = DVec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: DVec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: DVec3) -> DVec3 {
        DVec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> DVec3 {
        let len = self.length();
        if len == 0.0 {
            Self::ZERO
        } else {
            self / len
        }
    }
}

impl Add for DVec3 {
    type Output = DVec3;

    fn add(self, rhs: DVec3) -> Self::Output {
        DVec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for DVec3 {
    type Output = DVec3;

    fn sub(self, rhs: DVec3) -> Self::Output {
        DVec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for DVec3 {
    type Output = DVec3;

    fn mul(self, rhs: f64) -> Self::Output {
        DVec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for DVec3 {
    type Output = DVec3;

    fn div(self, rhs: f64) -> Self::Output {
        DVec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl DVec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn truncate(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DMat3 {
    pub x_axis: DVec3,
    pub y_axis: DVec3,
    pub z_axis: DVec3,
}

impl DMat3 {
    pub const IDENTITY: DMat3 = DMat3 {
        x_axis: DVec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        y_axis: DVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        z_axis: DVec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    pub fn from_cols(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    pub fn from_cols_array(values: &[f64; 9]) -> Self {
        Self {
            x_axis: DVec3::new(values[0], values[1], values[2]),
            y_axis: DVec3::new(values[3], values[4], values[5]),
            z_axis: DVec3::new(values[6], values[7], values[8]),
        }
    }
}

impl Mul<DVec3> for DMat3 {
    type Output = DVec3;

    fn mul(self, rhs: DVec3) -> Self::Output {
        DVec3::new(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.z_axis.x * rhs.z,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.z_axis.y * rhs.z,
            self.x_axis.z * rhs.x + self.y_axis.z * rhs.y + self.z_axis.z * rhs.z,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DMat4 {
    pub x_axis: DVec4,
    pub y_axis: DVec4,
    pub z_axis: DVec4,
    pub w_axis: DVec4,
}

impl DMat4 {
    pub const IDENTITY: DMat4 = DMat4 {
        x_axis: DVec4 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        y_axis: DVec4 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        },
        z_axis: DVec4 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        },
        w_axis: DVec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
    };

    pub fn from_cols(x_axis: DVec4, y_axis: DVec4, z_axis: DVec4, w_axis: DVec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    pub fn from_cols_array(values: &[f64; 16]) -> Self {
        Self {
            x_axis: DVec4::new(values[0], values[1], values[2], values[3]),
            y_axis: DVec4::new(values[4], values[5], values[6], values[7]),
            z_axis: DVec4::new(values[8], values[9], values[10], values[11]),
            w_axis: DVec4::new(values[12], values[13], values[14], values[15]),
        }
    }
}
