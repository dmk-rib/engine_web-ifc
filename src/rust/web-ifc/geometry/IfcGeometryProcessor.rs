//! Rust port of IfcGeometryProcessor public API.

use std::collections::HashMap;

use glam::{DMat4, DVec4};

use crate::web_ifc::geometry::representation::geometry::{IfcComposedMesh, IfcFlatMesh, IfcSurface};
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;
use crate::web_ifc::parsing::ifc_loader::IfcLoader;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;

#[derive(Clone, Debug)]
pub struct IfcGeometrySettings {
    pub coordinate_to_origin: bool,
    pub optimize_profiles: bool,
    pub export_polylines: bool,
    pub circle_segments: u16,
    pub tolerance_plane_intersection: f64,
    pub tolerance_plane_deviation: f64,
    pub tolerance_back_deviation_distance: f64,
    pub tolerance_inside_outside_perimeter: f64,
    pub tolerance_bounding_box: f64,
    pub boolean_union_threshold: u16,
}

impl Default for IfcGeometrySettings {
    fn default() -> Self {
        Self {
            coordinate_to_origin: false,
            optimize_profiles: true,
            export_polylines: false,
            circle_segments: 12,
            tolerance_plane_intersection: 1.0E-04,
            tolerance_plane_deviation: 1.0E-04,
            tolerance_back_deviation_distance: 1.0E-04,
            tolerance_inside_outside_perimeter: 1.0E-10,
            tolerance_bounding_box: 1.0E-02,
            boolean_union_threshold: 150,
        }
    }
}

#[derive(Debug, Default)]
pub struct BooleanManager;

impl BooleanManager {
    pub fn bool_process(&self, _first: &[IfcGeometry], _second: &mut [IfcGeometry], _op: &str, _settings: &IfcGeometrySettings) -> IfcGeometry {
        unimplemented!("BooleanManager::bool_process not yet implemented");
    }
}

#[derive(Debug)]
pub struct IfcGeometryProcessor<'a> {
    settings: IfcGeometrySettings,
    express_id_to_geometry: HashMap<u32, IfcGeometry>,
    loader: &'a IfcLoader,
    schema_manager: &'a IfcSchemaManager,
    transformation: DMat4,
    coordination_matrix: DMat4,
    is_coordinated: bool,
    bool_engine: BooleanManager,
    predefined_cylinder: IfcGeometry,
    predefined_cube: IfcGeometry,
}

impl<'a> IfcGeometryProcessor<'a> {
    pub fn new(
        loader: &'a IfcLoader,
        schema_manager: &'a IfcSchemaManager,
        circle_segments: u16,
        coordinate_to_origin: bool,
        tolerance_plane_intersection: f64,
        tolerance_plane_deviation: f64,
        tolerance_back_deviation_distance: f64,
        tolerance_inside_outside_perimeter: f64,
        tolerance_scalar_equality: f64,
        plane_refit_iterations: f64,
        boolean_union_threshold: f64,
    ) -> Self {
        let _ = (circle_segments, coordinate_to_origin, tolerance_plane_intersection, tolerance_plane_deviation, tolerance_back_deviation_distance, tolerance_inside_outside_perimeter, tolerance_scalar_equality, plane_refit_iterations, boolean_union_threshold);
        Self {
            settings: IfcGeometrySettings::default(),
            express_id_to_geometry: HashMap::new(),
            loader,
            schema_manager,
            transformation: DMat4::IDENTITY,
            coordination_matrix: DMat4::IDENTITY,
            is_coordinated: false,
            bool_engine: BooleanManager::default(),
            predefined_cylinder: IfcGeometry::default(),
            predefined_cube: IfcGeometry::default(),
        }
    }

    pub fn get_geometry(&mut self, _express_id: u32) -> &IfcGeometry {
        unimplemented!("IfcGeometryProcessor::get_geometry not yet implemented");
    }

    pub fn get_flat_mesh(&self, _express_id: u32, _apply_linear_scaling_factor: bool) -> IfcFlatMesh {
        unimplemented!("IfcGeometryProcessor::get_flat_mesh not yet implemented");
    }

    pub fn get_mesh(&self, _express_id: u32) -> IfcComposedMesh {
        unimplemented!("IfcGeometryProcessor::get_mesh not yet implemented");
    }

    pub fn set_transformation(&mut self, _val: [f64; 16]) {
        unimplemented!("IfcGeometryProcessor::set_transformation not yet implemented");
    }

    pub fn get_flat_coordination_matrix(&self) -> [f64; 16] {
        self.coordination_matrix.to_cols_array()
    }

    pub fn get_coordination_matrix(&self) -> DMat4 {
        self.coordination_matrix
    }

    pub fn clear(&mut self) {
        self.express_id_to_geometry.clear();
    }

    pub fn clone_processor(&self, loader: &'a IfcLoader) -> Self {
        let mut cloned = Self::new(
            loader,
            self.schema_manager,
            self.settings.circle_segments,
            self.settings.coordinate_to_origin,
            self.settings.tolerance_plane_intersection,
            self.settings.tolerance_plane_deviation,
            self.settings.tolerance_back_deviation_distance,
            self.settings.tolerance_inside_outside_perimeter,
            0.0,
            0.0,
            self.settings.boolean_union_threshold as f64,
        );
        cloned.express_id_to_geometry = self.express_id_to_geometry.clone();
        cloned
    }

    pub fn add_composed_mesh_to_flat_mesh(
        &self,
        _flat_mesh: &mut IfcFlatMesh,
        _composed_mesh: &IfcComposedMesh,
        _parent_matrix: DMat4,
        _color: DVec4,
        _has_color: bool,
    ) {
        unimplemented!("add_composed_mesh_to_flat_mesh not yet implemented");
    }

    pub fn read_2d_array_of_three_indices(&self) -> Vec<u32> {
        unimplemented!("read_2d_array_of_three_indices not yet implemented");
    }

    pub fn read_indexed_polygonal_face(&self, _express_id: u32, _bounds: &mut Vec<crate::web_ifc::geometry::representation::geometry::IfcBound3D>, _points: &[glam::DVec3]) {
        unimplemented!("read_indexed_polygonal_face not yet implemented");
    }

    pub fn get_surface(&self, _express_id: u32) -> IfcSurface {
        unimplemented!("get_surface not yet implemented");
    }
}
