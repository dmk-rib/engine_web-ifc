//! Rust port of IfcGeometryLoader public API.

use std::collections::HashMap;

use glam::{DMat3, DMat4, DVec2, DVec3, DVec4};

use crate::web_ifc::geometry::representation::geometry::{IfcAlignment, IfcBound3D, IfcCrossSections, IfcProfile, IfcTrimmingSelect, TrimSense};
use crate::web_ifc::geometry::representation::ifc_curve::IfcCurve;
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;
use crate::web_ifc::parsing::ifc_loader::IfcLoader;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;

#[derive(Debug)]
pub struct IfcGeometryLoader<'a> {
    loader: &'a IfcLoader,
    schema_manager: &'a IfcSchemaManager,
    circle_segments: u16,
    linear_scaling_factor: f64,
    angle_units: String,
    rel_voids: HashMap<u32, Vec<u32>>,
    styled_items: HashMap<u32, Vec<(u32, u32)>>,
    rel_materials: HashMap<u32, Vec<(u32, u32)>>,
    material_definitions: HashMap<u32, Vec<(u32, u32)>>,
}

impl<'a> IfcGeometryLoader<'a> {
    pub fn new(
        loader: &'a IfcLoader,
        schema_manager: &'a IfcSchemaManager,
        circle_segments: u16,
        _tolerance_plane_intersection: f64,
        _tolerance_plane_deviation: f64,
        _tolerance_back_deviation_distance: f64,
        _tolerance_inside_outside_perimeter: f64,
        _tolerance_scalar_equality: f64,
        _unused: f64,
        _boolean_union_threshold: f64,
    ) -> Self {
        Self {
            loader,
            schema_manager,
            circle_segments,
            linear_scaling_factor: 1.0,
            angle_units: String::new(),
            rel_voids: HashMap::new(),
            styled_items: HashMap::new(),
            rel_materials: HashMap::new(),
            material_definitions: HashMap::new(),
        }
    }

    pub fn reset_cache(&mut self) {
        unimplemented!("reset_cache not yet implemented");
    }

    pub fn get_axis1_placement(&self, _express_id: u32) -> [DVec3; 2] {
        unimplemented!("get_axis1_placement not yet implemented");
    }

    pub fn get_axis2_placement_2d(&self, _express_id: u32) -> DMat3 {
        unimplemented!("get_axis2_placement_2d not yet implemented");
    }

    pub fn get_local_placement(&self, _express_id: u32, _vector: DVec3) -> DMat4 {
        unimplemented!("get_local_placement not yet implemented");
    }

    pub fn get_cartesian_point_3d(&self, _express_id: u32) -> DVec3 {
        unimplemented!("get_cartesian_point_3d not yet implemented");
    }

    pub fn get_cartesian_point_2d(&self, _express_id: u32) -> DVec2 {
        unimplemented!("get_cartesian_point_2d not yet implemented");
    }

    pub fn get_vector(&self, _express_id: u32) -> DVec3 {
        unimplemented!("get_vector not yet implemented");
    }

    pub fn get_profile(&self, _express_id: u32) -> IfcProfile {
        unimplemented!("get_profile not yet implemented");
    }

    pub fn get_profile_3d(&self, _express_id: u32) -> IfcProfile {
        unimplemented!("get_profile_3d not yet implemented");
    }

    pub fn get_local_curve(&self, _express_id: u32) -> IfcCurve {
        unimplemented!("get_local_curve not yet implemented");
    }

    pub fn get_curve(&self, _express_id: u32, _dimensions: u8, _edge: bool) -> IfcCurve {
        unimplemented!("get_curve not yet implemented");
    }

    pub fn compute_curve_length(&self, _curve: &IfcCurve) -> f64 {
        unimplemented!("compute_curve_length not yet implemented");
    }

    pub fn compute_length_to_point(&self, _curve: &IfcCurve, _target_point: &DVec3) -> f64 {
        unimplemented!("compute_length_to_point not yet implemented");
    }

    pub fn get_parameter_for_point(&self, _curve: &IfcCurve, _total_length: f64, _point: &DVec3) -> f64 {
        unimplemented!("get_parameter_for_point not yet implemented");
    }

    pub fn read_ifc_cartesian_point_list(&self, _express_id: u32) -> bool {
        unimplemented!("read_ifc_cartesian_point_list not yet implemented");
    }

    pub fn read_ifc_cartesian_point_list_3d(&self, _express_id: u32) -> Vec<DVec3> {
        unimplemented!("read_ifc_cartesian_point_list_3d not yet implemented");
    }

    pub fn read_ifc_cartesian_point_list_2d(&self, _express_id: u32) -> Vec<DVec2> {
        unimplemented!("read_ifc_cartesian_point_list_2d not yet implemented");
    }

    pub fn get_oriented_edge(&self, _express_id: u32) -> IfcCurve {
        unimplemented!("get_oriented_edge not yet implemented");
    }

    pub fn get_edge(&self, _express_id: u32) -> IfcCurve {
        unimplemented!("get_edge not yet implemented");
    }

    pub fn get_bound(&self, _express_id: u32) -> IfcBound3D {
        unimplemented!("get_bound not yet implemented");
    }

    pub fn get_loop(&self, _express_id: u32) -> IfcCurve {
        unimplemented!("get_loop not yet implemented");
    }

    pub fn get_color(&self, _express_id: u32) -> Option<DVec4> {
        unimplemented!("get_color not yet implemented");
    }

    pub fn get_cross_sections_2d(&self, _express_id: u32) -> IfcCrossSections {
        unimplemented!("get_cross_sections_2d not yet implemented");
    }

    pub fn get_cross_sections_3d(&self, _express_id: u32, _scaled: bool, _coordination: DMat4) -> IfcCrossSections {
        unimplemented!("get_cross_sections_3d not yet implemented");
    }

    pub fn get_placements_on_curve_points(&self, _curve_id: u32, _placements: &mut HashMap<f64, DMat4>) {
        unimplemented!("get_placements_on_curve_points not yet implemented");
    }

    pub fn get_alignment(&self, _express_id: u32, _alignment: IfcAlignment, _transform: DMat4, _source_express_id: u32) -> IfcAlignment {
        unimplemented!("get_alignment not yet implemented");
    }

    pub fn get_color_into(&self, _express_id: u32, _output_color: &DVec4) -> bool {
        unimplemented!("get_color_into not yet implemented");
    }

    pub fn get_rel_voids(&self) -> &HashMap<u32, Vec<u32>> {
        &self.rel_voids
    }

    pub fn get_styled_items(&self) -> &HashMap<u32, Vec<(u32, u32)>> {
        &self.styled_items
    }

    pub fn get_rel_materials(&self) -> &HashMap<u32, Vec<(u32, u32)>> {
        &self.rel_materials
    }

    pub fn get_material_definitions(&self) -> &HashMap<u32, Vec<(u32, u32)>> {
        &self.material_definitions
    }

    pub fn get_linear_scaling_factor(&self) -> f64 {
        self.linear_scaling_factor
    }

    pub fn get_angle_units(&self) -> &str {
        &self.angle_units
    }

    pub fn clear(&self) {
        unimplemented!("clear not yet implemented");
    }

    pub fn clone_loader(&self, loader: &'a IfcLoader) -> Self {
        Self::new(
            loader,
            self.schema_manager,
            self.circle_segments,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct ComputeCurveParams {
    pub dimensions: u8,
    pub ignore_placement: bool,
    pub edge: bool,
    pub same_sense: i32,
    pub has_trim: bool,
    pub trim_start: IfcTrimmingSelect,
    pub trim_end: IfcTrimmingSelect,
    pub trim_sense: TrimSense,
}
