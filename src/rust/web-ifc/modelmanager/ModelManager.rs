//! Rust port of ModelManager public API.

use std::collections::HashMap;
use std::sync::Arc;

use crate::web_ifc::geometry::ifc_geometry_processor::IfcGeometryProcessor;
use crate::web_ifc::parsing::ifc_loader::IfcLoader;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;

#[derive(Debug, Clone)]
pub struct LoaderSettings {
    pub coordinate_to_origin: bool,
    pub circle_segments: u16,
    pub tape_size: u32,
    pub memory_limit: u32,
    pub linewriter_buffer: u16,
    pub tolerance_plane_intersection: f64,
    pub tolerance_plane_deviation: f64,
    pub tolerance_back_deviation_distance: f64,
    pub tolerance_inside_outside_perimeter: f64,
    pub tolerance_scalar_equality: f64,
    pub plane_refit_iterations: u16,
    pub boolean_union_threshold: u16,
}

impl Default for LoaderSettings {
    fn default() -> Self {
        Self {
            coordinate_to_origin: false,
            circle_segments: 12,
            tape_size: 67_108_864,
            memory_limit: 2_147_483_648,
            linewriter_buffer: 10_000,
            tolerance_plane_intersection: 1.0E-04,
            tolerance_plane_deviation: 1.0E-04,
            tolerance_back_deviation_distance: 1.0E-04,
            tolerance_inside_outside_perimeter: 1.0E-10,
            tolerance_scalar_equality: 1.0E-04,
            plane_refit_iterations: 1,
            boolean_union_threshold: 150,
        }
    }
}

#[derive(Debug)]
pub struct ModelManager {
    schema_manager: Arc<IfcSchemaManager>,
    loaders: Vec<IfcLoader>,
    settings: Vec<LoaderSettings>,
    geometry_processors: HashMap<u32, IfcGeometryProcessor>,
    header_shown: bool,
    mt_enabled: bool,
}

impl ModelManager {
    pub fn new(mt_enabled: bool) -> Self {
        Self {
            schema_manager: Arc::new(IfcSchemaManager::new()),
            loaders: Vec::new(),
            settings: Vec::new(),
            geometry_processors: HashMap::new(),
            header_shown: false,
            mt_enabled,
        }
    }

    pub fn get_geometry_processor(&self, _model_id: u32) -> Option<&IfcGeometryProcessor> {
        self.geometry_processors.get(&_model_id)
    }

    pub fn get_settings(&self, model_id: u32) -> Option<&LoaderSettings> {
        self.settings.get(model_id as usize)
    }

    pub fn get_ifc_loader(&self, model_id: u32) -> Option<&IfcLoader> {
        self.loaders.get(model_id as usize)
    }

    pub fn get_schema_manager(&self) -> &IfcSchemaManager {
        &self.schema_manager
    }

    pub fn is_model_open(&self, model_id: u32) -> bool {
        model_id as usize  < self.loaders.len()
    }

    pub fn close_model(&mut self, _model_id: u32) {
        unimplemented!("close_model not yet implemented");
    }

    pub fn create_model(&mut self, settings: LoaderSettings) -> u32 {
        let model_id = self.loaders.len() as u32;
        let loader = IfcLoader::new(
            settings.tape_size,
            settings.memory_limit as u64,
            settings.linewriter_buffer as u32,
            Arc::clone(&self.schema_manager),
        );
        self.loaders.push(loader);
        self.settings.push(settings);
        model_id
    }

    pub fn set_log_level(&mut self, _level: u8) {
        unimplemented!("set_log_level not yet implemented");
    }

    pub fn close_all_models(&mut self) {
        self.loaders.clear();
        self.settings.clear();
        self.geometry_processors.clear();
    }
}
