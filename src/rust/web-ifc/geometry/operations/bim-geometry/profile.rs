//! Profile generation.

use glam::DMat4;

use super::buffers::Buffers;
use super::curve::Curve;
use super::utils::{
    get_c_shaped_curve, get_i_shaped_curve, get_l_shaped_curve, get_t_shaped_curve, get_u_shaped_curve,
    get_z_shaped_curve,
};

#[derive(Clone, Debug, Default)]
pub struct Profile {
    pub p_type: u16,
    pub width: f64,
    pub depth: f64,
    pub thickness: f64,
    pub flange_thickness: f64,
    pub has_fillet: bool,
    pub fillet_radius: f64,
    pub radius: f64,
    pub slope: f64,
    pub num_segments: u16,
    pub placement: Vec<f64>,
    pub profile: Curve,
}

impl Profile {
    pub fn set_values(
        &mut self,
        p_type: u16,
        width: f64,
        depth: f64,
        web_thickness: f64,
        flange_thickness: f64,
        has_fillet: bool,
        fillet_radius: f64,
        radius: f64,
        slope: f64,
        num_segments: u16,
        placement: Vec<f64>,
    ) {
        self.p_type = p_type;
        self.width = width;
        self.depth = depth;
        self.thickness = web_thickness;
        self.flange_thickness = flange_thickness;
        self.has_fillet = has_fillet;
        self.fillet_radius = fillet_radius;
        self.radius = radius;
        self.slope = slope;
        self.num_segments = num_segments;
        self.placement = placement;
    }

    pub fn get_buffers(&mut self) -> Buffers {
        let mut buffers = Buffers::default();
        let placement = if self.placement.len() == 16 {
            DMat4::from_cols_array(&[
                self.placement[0], self.placement[1], self.placement[2], self.placement[3],
                self.placement[4], self.placement[5], self.placement[6], self.placement[7],
                self.placement[8], self.placement[9], self.placement[10], self.placement[11],
                self.placement[12], self.placement[13], self.placement[14], self.placement[15],
            ])
        } else {
            DMat4::IDENTITY
        };

        self.profile = match self.p_type {
            0 => get_i_shaped_curve(
                self.width,
                self.depth,
                self.thickness,
                self.flange_thickness,
                self.has_fillet,
                self.fillet_radius,
                placement,
            ),
            1 => get_c_shaped_curve(
                self.width,
                self.depth,
                self.thickness,
                self.flange_thickness,
                self.has_fillet,
                self.fillet_radius,
                placement,
            ),
            2 => get_z_shaped_curve(
                self.width,
                self.depth,
                self.thickness,
                self.flange_thickness,
                self.has_fillet,
                self.fillet_radius,
                placement,
            ),
            3 => get_t_shaped_curve(
                self.width,
                self.depth,
                self.thickness,
                self.has_fillet,
                self.fillet_radius,
                self.radius,
                self.slope,
                placement,
            ),
            4 => get_l_shaped_curve(
                self.width,
                self.depth,
                self.thickness,
                self.has_fillet,
                self.fillet_radius,
                self.radius,
                self.slope,
                self.num_segments,
                placement,
            ),
            5 => get_u_shaped_curve(
                self.width,
                self.depth,
                self.thickness,
                self.flange_thickness,
                self.fillet_radius,
                self.radius,
                self.slope,
                placement,
            ),
            _ => Curve::default(),
        };

        for point in &self.profile.points {
            buffers.add_point(*point);
        }
        buffers
    }
}
