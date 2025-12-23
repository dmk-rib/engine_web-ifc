//! Web-IFC Main API Class

use std::collections::{HashMap, HashSet};
use std::sync::{Arc as SyncArc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::api::helpers::log::{Log, LogLevel};
use crate::api::helpers::properties::Properties;
use crate::api::value::{Map, Value};

pub use crate::api::ifc_schema::*;

pub const UNKNOWN: u32 = 0;
pub const STRING: u32 = 1;
pub const LABEL: u32 = 2;
pub const ENUM: u32 = 3;
pub const REAL: u32 = 4;
pub const REF: u32 = 5;
pub const EMPTY: u32 = 6;
pub const SET_BEGIN: u32 = 7;
pub const SET_END: u32 = 8;
pub const LINE_END: u32 = 9;
pub const INTEGER: u32 = 10;

#[derive(Clone, Debug, Default)]
pub struct LoaderSettings {
    pub coordinate_to_origin: Option<bool>,
    pub circle_segments: Option<u32>,
    pub memory_limit: Option<usize>,
    pub tape_size: Option<usize>,
    pub linewriter_buffer: Option<usize>,
    pub tolerance_plane_intersection: Option<f64>,
    pub tolerance_plane_deviation: Option<f64>,
    pub tolerance_back_deviation_distance: Option<f64>,
    pub tolerance_inside_outside_perimeter: Option<f64>,
    pub tolerance_scalar_equality: Option<f64>,
    pub plane_refit_iterations: Option<u32>,
    pub boolean_union_threshold: Option<u32>,
}

pub trait Vector<T> {
    fn get(&self, index: usize) -> Option<&T>;
    fn size(&self) -> usize;
}

#[derive(Clone, Debug, Default)]
pub struct IfcVector<T>(pub Vec<T>);

impl<T> IfcVector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self(data)
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }
}

impl<T> Vector<T> for IfcVector<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}

impl<T> IntoIterator for IfcVector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Color {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RawLineData {
    pub id: i32,
    pub type_code: i32,
    pub arguments: Vec<Value>,
}

#[derive(Clone, Debug, Default)]
pub struct PlacedGeometry {
    pub color: Color,
    pub geometry_express_id: i32,
    pub flat_transformation: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct FlatMesh {
    pub geometries: IfcVector<PlacedGeometry>,
    pub express_id: i32,
}

impl FlatMesh {
    pub fn delete(&mut self) {
        self.geometries.0.clear();
    }
}

#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct Curve {
    pub points: Vec<Point>,
    pub user_data: Vec<String>,
    pub arc_segments: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct SweptDiskSolid {
    pub profile: Profile,
    pub axis: Vec<Curve>,
    pub profile_radius: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Profile {
    pub curve: Curve,
    pub holes: Vec<Curve>,
    pub profiles: Vec<Profile>,
    pub is_convex: bool,
    pub is_composite: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CrossSection {
    pub curves: Vec<Curve>,
    pub express_id: Vec<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct AlignmentSegment {
    pub curves: Vec<Curve>,
}

#[derive(Clone, Debug, Default)]
pub struct Alignment {
    pub flat_coordination_matrix: Vec<f64>,
    pub horizontal: AlignmentSegment,
    pub vertical: AlignmentSegment,
    pub absolute: AlignmentSegment,
}

#[derive(Clone, Debug, Default)]
pub struct IfcGeometry {
    pub vertex_data: Vec<f32>,
    pub index_data: Vec<u32>,
    pub swept_disk_solid: Option<SweptDiskSolid>,
}

impl IfcGeometry {
    pub fn GetVertexData(&self) -> u32 {
        if self.vertex_data.is_empty() {
            return 0;
        }
        let ptr = self.vertex_data.as_ptr() as usize;
        u32::try_from(ptr).unwrap_or(0)
    }

    pub fn GetVertexDataSize(&self) -> u32 {
        self.vertex_data.len() as u32
    }

    pub fn GetIndexData(&self) -> u32 {
        if self.index_data.is_empty() {
            return 0;
        }
        let ptr = self.index_data.as_ptr() as usize;
        u32::try_from(ptr).unwrap_or(0)
    }

    pub fn GetIndexDataSize(&self) -> u32 {
        self.index_data.len() as u32
    }

    pub fn GetSweptDiskSolid(&self) -> SweptDiskSolid {
        self.swept_disk_solid.clone().unwrap_or_default()
    }

    pub fn delete(&mut self) {
        self.vertex_data.clear();
        self.index_data.clear();
        self.swept_disk_solid = None;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Buffers {
    pub fvertex_data: Vec<f64>,
    pub index_data: Vec<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct AABB {
    pub buffers: Buffers,
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl AABB {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) {
        self.min = [min_x, min_y, min_z];
        self.max = [max_x, max_y, max_z];
    }
}

#[derive(Clone, Debug, Default)]
pub struct Extrusion {
    pub buffers: Buffers,
    pub profile: Vec<f64>,
    pub direction: Vec<f64>,
    pub length: f64,
    pub cutting_plane_normal: Vec<f64>,
    pub cutting_plane_pos: Vec<f64>,
    pub cap: bool,
    pub holes: Vec<f64>,
}

impl Extrusion {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        profile: Vec<f64>,
        direction: Vec<f64>,
        length: f64,
        cutting_plane_normal: Vec<f64>,
        cutting_plane_pos: Vec<f64>,
        cap: bool,
    ) {
        self.profile = profile;
        self.direction = direction;
        self.length = length;
        self.cutting_plane_normal = cutting_plane_normal;
        self.cutting_plane_pos = cutting_plane_pos;
        self.cap = cap;
    }

    pub fn SetHoles(&mut self, profile: Vec<f64>) {
        self.holes = profile;
    }

    pub fn ClearHoles(&mut self) {
        self.holes.clear();
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sweep {
    pub buffers: Buffers,
    pub scaling: f64,
    pub closed: bool,
    pub profile: Vec<f64>,
    pub directrix: Vec<f64>,
    pub initial_normal: Option<Vec<f64>>,
    pub rotate90: Option<bool>,
    pub optimize: Option<bool>,
}

impl Sweep {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        closed: bool,
        profile: Vec<f64>,
        directrix: Vec<f64>,
        initial_normal: Option<Vec<f64>>,
        rotate90: Option<bool>,
        optimize: Option<bool>,
    ) {
        self.scaling = scaling;
        self.closed = closed;
        self.profile = profile;
        self.directrix = directrix;
        self.initial_normal = initial_normal;
        self.rotate90 = rotate90;
        self.optimize = optimize;
    }
}

#[derive(Clone, Debug, Default)]
pub struct CircularSweep {
    pub buffers: Buffers,
    pub scaling: f64,
    pub closed: bool,
    pub profile: Vec<f64>,
    pub radius: f64,
    pub directrix: Vec<f64>,
    pub initial_normal: Option<Vec<f64>>,
    pub rotate90: Option<bool>,
    pub optimize: Option<bool>,
}

impl CircularSweep {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        closed: bool,
        profile: Vec<f64>,
        radius: f64,
        directrix: Vec<f64>,
        initial_normal: Option<Vec<f64>>,
        rotate90: Option<bool>,
        optimize: Option<bool>,
    ) {
        self.scaling = scaling;
        self.closed = closed;
        self.profile = profile;
        self.radius = radius;
        self.directrix = directrix;
        self.initial_normal = initial_normal;
        self.rotate90 = rotate90;
        self.optimize = optimize;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Revolution {
    pub buffers: Buffers,
    pub scaling: f64,
    pub profile: Vec<f64>,
    pub axis: Vec<f64>,
    pub angle: f64,
    pub optimize: Option<bool>,
}

impl Revolution {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        profile: Vec<f64>,
        axis: Vec<f64>,
        angle: f64,
        optimize: Option<bool>,
    ) {
        self.scaling = scaling;
        self.profile = profile;
        self.axis = axis;
        self.angle = angle;
        self.optimize = optimize;
    }
}

#[derive(Clone, Debug, Default)]
pub struct CylindricalRevolve {
    pub buffers: Buffers,
    pub scaling: f64,
    pub profile: Vec<f64>,
    pub axis: Vec<f64>,
    pub radius: f64,
    pub angle: f64,
    pub optimize: Option<bool>,
}

impl CylindricalRevolve {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        profile: Vec<f64>,
        axis: Vec<f64>,
        radius: f64,
        angle: f64,
        optimize: Option<bool>,
    ) {
        self.scaling = scaling;
        self.profile = profile;
        self.axis = axis;
        self.radius = radius;
        self.angle = angle;
        self.optimize = optimize;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Parabola {
    pub buffers: Buffers,
    pub scaling: f64,
    pub profile: Vec<f64>,
    pub height: f64,
    pub width: f64,
    pub segments: u32,
}

impl Parabola {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        profile: Vec<f64>,
        height: f64,
        width: f64,
        segments: u32,
    ) {
        self.scaling = scaling;
        self.profile = profile;
        self.height = height;
        self.width = width;
        self.segments = segments;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Clothoid {
    pub buffers: Buffers,
    pub scaling: f64,
    pub profile: Vec<f64>,
    pub length: f64,
    pub radius_start: f64,
    pub radius_end: f64,
    pub segments: u32,
}

impl Clothoid {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        profile: Vec<f64>,
        length: f64,
        radius_start: f64,
        radius_end: f64,
        segments: u32,
    ) {
        self.scaling = scaling;
        self.profile = profile;
        self.length = length;
        self.radius_start = radius_start;
        self.radius_end = radius_end;
        self.segments = segments;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Arc {
    pub buffers: Buffers,
    pub scaling: f64,
    pub profile: Vec<f64>,
    pub radius: f64,
    pub angle: f64,
    pub segments: u32,
}

impl Arc {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(
        &mut self,
        scaling: f64,
        profile: Vec<f64>,
        radius: f64,
        angle: f64,
        segments: u32,
    ) {
        self.scaling = scaling;
        self.profile = profile;
        self.radius = radius;
        self.angle = angle;
        self.segments = segments;
    }
}

#[derive(Clone, Debug, Default)]
pub struct BooleanOperator {
    pub buffers: Buffers,
    pub first: Vec<f64>,
    pub second: Vec<f64>,
    pub operation: String,
}

impl BooleanOperator {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    pub fn SetValues(&mut self, triangles: Vec<f64>, operation: &str) {
        self.first = triangles;
        self.operation = operation.to_string();
    }

    pub fn SetSecond(&mut self, triangles: Vec<f64>) {
        self.second = triangles;
    }

    pub fn clear(&mut self) {
        self.first.clear();
        self.second.clear();
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProfileSection {
    pub buffers: Buffers,
    pub values: Vec<f64>,
}

impl ProfileSection {
    pub fn GetBuffers(&self) -> Buffers {
        self.buffers.clone()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn SetValues(
        &mut self,
        p_type: f64,
        width: f64,
        depth: f64,
        web_thickness: f64,
        flange_thickness: f64,
        has_fillet: bool,
        fillet_radius: f64,
        radius: f64,
        slope: f64,
        circle_segments: f64,
        placement: Vec<f64>,
    ) {
        self.values = vec![
            p_type,
            width,
            depth,
            web_thickness,
            flange_thickness,
            if has_fillet { 1.0 } else { 0.0 },
            fillet_radius,
            radius,
            slope,
            circle_segments,
        ];
        self.values.extend(placement);
    }
}

#[derive(Clone, Debug, Default)]
pub struct IfcType {
    pub type_id: i32,
    pub type_name: String,
}

#[derive(Clone, Debug, Default)]
pub struct NewIfcModel {
    pub schema: String,
    pub name: Option<String>,
    pub description: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub organizations: Option<Vec<String>>,
    pub authorization: Option<String>,
}

pub type ModelLoadCallback = Box<dyn Fn(usize, usize) -> Vec<u8> + Send + Sync>;
pub type ModelSaveCallback = Box<dyn Fn(&[u8]) + Send + Sync>;
pub type LocateFileHandlerFn = Box<dyn Fn(&str, &str) -> String + Send + Sync>;

pub fn ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default()
}

#[derive(Debug)]
pub enum IfcApiError {
    ModelNotFound(i32),
    LineNotFound(i32),
    InvalidInput(&'static str),
}

impl std::fmt::Display for IfcApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IfcApiError::ModelNotFound(id) => write!(f, "model not found: {id}"),
            IfcApiError::LineNotFound(id) => write!(f, "line not found: {id}"),
            IfcApiError::InvalidInput(field) => write!(f, "invalid input: {field}"),
        }
    }
}

impl std::error::Error for IfcApiError {}

#[derive(Default)]
pub(crate) struct Model {
    pub(crate) schema: String,
    pub(crate) lines: HashMap<i32, Value>,
    pub(crate) line_types: HashMap<i32, i32>,
    pub(crate) max_express_id: i32,
}

#[derive(Default)]
pub(crate) struct IfcAPIState {
    pub(crate) next_model_id: i32,
    pub(crate) models: HashMap<i32, Model>,
    pub(crate) deleted_lines: HashMap<i32, HashSet<i32>>,
    pub(crate) ifc_guid_map: HashMap<i32, HashMap<String, i32>>,
}

#[derive(Clone)]
pub struct IfcAPI {
    state: SyncArc<Mutex<IfcAPIState>>,
    wasm_path: SyncArc<Mutex<String>>,
    is_wasm_path_absolute: SyncArc<Mutex<bool>>,
    model_schema_list: SyncArc<Mutex<Vec<usize>>>,
    model_schema_name_list: SyncArc<Mutex<Vec<String>>>,
    pub properties: Properties,
}

impl Default for IfcAPI {
    fn default() -> Self {
        let state = SyncArc::new(Mutex::new(IfcAPIState::default()));
        Self {
            state: state.clone(),
            wasm_path: SyncArc::new(Mutex::new(String::new())),
            is_wasm_path_absolute: SyncArc::new(Mutex::new(false)),
            model_schema_list: SyncArc::new(Mutex::new(Vec::new())),
            model_schema_name_list: SyncArc::new(Mutex::new(Vec::new())),
            properties: Properties::new(state),
        }
    }
}

impl IfcAPI {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn Init(
        &self,
        _custom_locate_file_handler: Option<LocateFileHandlerFn>,
        _force_single_thread: bool,
    ) -> Result<(), IfcApiError> {
        Ok(())
    }

    pub fn OpenModels(&self, data: &[Vec<u8>]) -> Result<Vec<i32>, IfcApiError> {
        data.iter()
            .map(|buffer| self.OpenModel(buffer, None))
            .collect()
    }

    pub fn OpenModel(
        &self,
        _data: &[u8],
        _settings: Option<LoaderSettings>,
    ) -> Result<i32, IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        state.next_model_id += 1;
        let model_id = state.next_model_id;
        let schema_name = "IFC4".to_string();
        let schema_id =
            lookup_schema_id(&schema_name).ok_or(IfcApiError::InvalidInput("schema"))?;
        state.models.insert(
            model_id,
            Model {
                schema: schema_name.clone(),
                ..Model::default()
            },
        );
        if let Ok(mut list) = self.model_schema_list.lock() {
            if list.len() <= model_id as usize {
                list.resize(model_id as usize + 1, 0);
            }
            list[model_id as usize] = schema_id;
        }
        if let Ok(mut list) = self.model_schema_name_list.lock() {
            if list.len() <= model_id as usize {
                list.resize(model_id as usize + 1, String::new());
            }
            list[model_id as usize] = schema_name;
        }
        Ok(model_id)
    }

    pub fn OpenModelFromCallback(
        &self,
        callback: ModelLoadCallback,
        size: usize,
        settings: Option<LoaderSettings>,
    ) -> Result<i32, IfcApiError> {
        let data = callback(0, size);
        self.OpenModel(&data, settings)
    }

    pub fn GetModelSchema(&self, model_id: i32) -> Result<String, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        Ok(model.schema.clone())
    }

    pub fn CreateModel(&self, new_model: NewIfcModel) -> Result<i32, IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        state.next_model_id += 1;
        let model_id = state.next_model_id;
        let schema_id =
            lookup_schema_id(&new_model.schema).ok_or(IfcApiError::InvalidInput("schema"))?;
        state.models.insert(
            model_id,
            Model {
                schema: new_model.schema.clone(),
                ..Model::default()
            },
        );
        if let Ok(mut list) = self.model_schema_list.lock() {
            if list.len() <= model_id as usize {
                list.resize(model_id as usize + 1, 0);
            }
            list[model_id as usize] = schema_id;
        }
        if let Ok(mut list) = self.model_schema_name_list.lock() {
            if list.len() <= model_id as usize {
                list.resize(model_id as usize + 1, String::new());
            }
            list[model_id as usize] = new_model.schema;
        }
        Ok(model_id)
    }

    pub fn SaveModel(&self, _model_id: i32) -> Result<Vec<u8>, IfcApiError> {
        Ok(Vec::new())
    }

    pub fn SaveModelToCallback(
        &self,
        model_id: i32,
        callback: ModelSaveCallback,
    ) -> Result<(), IfcApiError> {
        let data = self.SaveModel(model_id)?;
        callback(&data);
        Ok(())
    }

    pub fn GetGeometry(
        &self,
        _model_id: i32,
        _express_id: i32,
    ) -> Result<IfcGeometry, IfcApiError> {
        Ok(IfcGeometry::default())
    }

    pub fn CreateAABB(&self) -> AABB {
        AABB::default()
    }

    pub fn CreateExtrusion(&self) -> Extrusion {
        Extrusion::default()
    }

    pub fn CreateSweep(&self) -> Sweep {
        Sweep::default()
    }

    pub fn CreateCircularSweep(&self) -> CircularSweep {
        CircularSweep::default()
    }

    pub fn CreateRevolution(&self) -> Revolution {
        Revolution::default()
    }

    pub fn CreateCylindricalRevolution(&self) -> CylindricalRevolve {
        CylindricalRevolve::default()
    }

    pub fn CreateParabola(&self) -> Parabola {
        Parabola::default()
    }

    pub fn CreateClothoid(&self) -> Clothoid {
        Clothoid::default()
    }

    pub fn CreateArc(&self) -> Arc {
        Arc::default()
    }

    pub fn CreateAlignment(&self) -> Alignment {
        Alignment::default()
    }

    pub fn CreateBooleanOperator(&self) -> BooleanOperator {
        BooleanOperator::default()
    }

    pub fn CreateProfile(&self) -> ProfileSection {
        ProfileSection::default()
    }

    pub fn GetHeaderLine(&self, _model_id: i32, _header_type: i32) -> RawLineData {
        RawLineData::default()
    }

    pub fn GetAllTypesOfModel(&self, model_id: i32) -> Result<Vec<IfcType>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        let mut type_ids: Vec<i32> = model.line_types.values().copied().collect();
        type_ids.sort_unstable();
        type_ids.dedup();
        Ok(type_ids
            .into_iter()
            .map(|type_id| IfcType {
                type_id,
                type_name: self.GetNameFromTypeCode(type_id),
            })
            .collect())
    }

    pub fn GetLine(
        &self,
        model_id: i32,
        express_id: i32,
        _flatten: bool,
        _inverse: bool,
        _inverse_prop_key: Option<&str>,
    ) -> Result<Value, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model
            .lines
            .get(&express_id)
            .cloned()
            .ok_or(IfcApiError::LineNotFound(express_id))
    }

    pub fn GetLines(
        &self,
        model_id: i32,
        express_ids: &[i32],
        flatten: bool,
        inverse: bool,
        inverse_prop_key: Option<&str>,
    ) -> Result<Vec<Value>, IfcApiError> {
        express_ids
            .iter()
            .map(|id| self.GetLine(model_id, *id, flatten, inverse, inverse_prop_key))
            .collect()
    }

    pub fn GetNextExpressID(&self, model_id: i32) -> Result<i32, IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get_mut(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model.max_express_id += 1;
        Ok(model.max_express_id)
    }

    /// C++ overload mapping: CreateIfcEntity(model_id, type_code, ...args)
    pub fn CreateIfcEntity(&self, _model_id: i32, type_code: i32, args: Vec<Value>) -> Value {
        let mut map = Map::new();
        map.insert("expressID".to_string(), Value::Number(0));
        map.insert("type".to_string(), Value::Number(type_code.into()));
        map.insert("arguments".to_string(), Value::Array(args));
        Value::Object(map)
    }

    pub fn CreateIFCGloballyUniqueId(&self, _model_id: i32) -> Value {
        let guid = crate::uuid::Uuid::new_v4().to_string();
        let mut map = Map::new();
        map.insert(
            "type".to_string(),
            Value::Number(IFCGLOBALLYUNIQUEID as i64),
        );
        map.insert("value".to_string(), Value::String(guid));
        Value::Object(map)
    }

    pub fn CreateIfcType(&self, _model_id: i32, type_code: i32, value: Value) -> Value {
        let mut map = Map::new();
        map.insert("type".to_string(), Value::Number(type_code.into()));
        map.insert("value".to_string(), value);
        Value::Object(map)
    }

    pub fn GetNameFromTypeCode(&self, type_code: i32) -> String {
        type_name_from_code(type_code)
            .map(ToString::to_string)
            .unwrap_or_else(|| format!("{type_code}"))
    }

    pub fn GetTypeCodeFromName(&self, type_name: &str) -> i32 {
        type_code_from_name(type_name).unwrap_or_default()
    }

    pub fn IsIfcElement(&self, type_code: i32) -> bool {
        is_ifc_element(type_code)
    }

    pub fn GetIfcEntityList(&self, model_id: i32) -> Result<Vec<i32>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        let mut ids: Vec<i32> = model.line_types.values().copied().collect();
        ids.sort_unstable();
        ids.dedup();
        Ok(ids)
    }

    pub fn DeleteLine(&self, model_id: i32, express_id: i32) -> Result<(), IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get_mut(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model.lines.remove(&express_id);
        model.line_types.remove(&express_id);
        state
            .deleted_lines
            .entry(model_id)
            .or_default()
            .insert(express_id);
        Ok(())
    }

    pub fn FlattenLine(&self, _model_id: i32, _line: &mut Value) {}

    pub fn GetRawLinesData(
        &self,
        model_id: i32,
        express_ids: &[i32],
    ) -> Result<Vec<RawLineData>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        Ok(express_ids
            .iter()
            .filter_map(|id| {
                model.lines.get(id).map(|line| RawLineData {
                    id: *id,
                    type_code: model.line_types.get(id).copied().unwrap_or_default(),
                    arguments: vec![line.clone()],
                })
            })
            .collect())
    }

    pub fn GetRawLineData(
        &self,
        model_id: i32,
        express_id: i32,
    ) -> Result<RawLineData, IfcApiError> {
        let data = self.GetRawLinesData(model_id, &[express_id])?;
        data.into_iter()
            .next()
            .ok_or(IfcApiError::LineNotFound(express_id))
    }

    pub fn WriteRawLineData(&self, model_id: i32, line: RawLineData) -> Result<(), IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get_mut(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model.lines.insert(line.id, Value::Array(line.arguments));
        model.line_types.insert(line.id, line.type_code);
        Ok(())
    }

    pub fn WriteRawLinesData(
        &self,
        model_id: i32,
        lines: Vec<RawLineData>,
    ) -> Result<(), IfcApiError> {
        for line in lines {
            self.WriteRawLineData(model_id, line)?;
        }
        Ok(())
    }

    pub fn GetLineIDsWithType(
        &self,
        model_id: i32,
        type_code: i32,
    ) -> Result<IfcVector<i32>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        let mut ids: Vec<i32> = model
            .line_types
            .iter()
            .filter_map(|(id, t)| if *t == type_code { Some(*id) } else { None })
            .collect();
        ids.sort_unstable();
        Ok(IfcVector::new(ids))
    }

    pub fn GetAllLines(&self, model_id: i32) -> Result<Vec<Value>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        Ok(model.lines.values().cloned().collect())
    }

    pub fn GetAllCrossSections2D(&self, _model_id: i32) -> Vec<CrossSection> {
        Vec::new()
    }

    pub fn GetAllCrossSections3D(&self, _model_id: i32) -> Vec<CrossSection> {
        Vec::new()
    }

    pub fn GetAllAlignments(&self, _model_id: i32) -> Vec<Alignment> {
        Vec::new()
    }

    pub fn SetGeometryTransformation(&self, _model_id: i32, _transformation: &[f64]) {}

    pub fn GetCoordinationMatrix(&self, _model_id: i32) -> Vec<f64> {
        vec![
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]
    }

    pub fn GetWorldTransformMatrix(&self, _model_id: i32) -> Vec<f64> {
        self.GetCoordinationMatrix(_model_id)
    }

    pub fn GetVertexArray(&self, _geometry: &IfcGeometry) -> Vec<f32> {
        _geometry.vertex_data.clone()
    }

    pub fn GetIndexArray(&self, _geometry: &IfcGeometry) -> Vec<u32> {
        _geometry.index_data.clone()
    }

    pub fn getSubArray(&self, data: &[u8], start: usize, size: usize) -> Vec<u8> {
        let end = start.saturating_add(size).min(data.len());
        data[start..end].to_vec()
    }

    pub fn CloseModel(&self, model_id: i32) -> Result<(), IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        state.models.remove(&model_id);
        Ok(())
    }

    pub fn Dispose(&self) {
        let mut state = self.state.lock().expect("state lock");
        state.models.clear();
    }

    pub fn StreamMeshes(&self, _model_id: i32, _types: &[i32]) -> Vec<FlatMesh> {
        Vec::new()
    }

    pub fn StreamAllMeshes(&self, _model_id: i32) -> Vec<FlatMesh> {
        Vec::new()
    }

    pub fn StreamAllMeshesWithTypes(&self, _model_id: i32, _types: &[i32]) -> Vec<FlatMesh> {
        Vec::new()
    }

    pub fn IsModelOpen(&self, model_id: i32) -> bool {
        let state = self.state.lock().expect("state lock");
        state.models.contains_key(&model_id)
    }

    pub fn LoadAllGeometry(&self, _model_id: i32) -> Vec<FlatMesh> {
        Vec::new()
    }

    pub fn GetFlatMesh(&self, _model_id: i32, express_id: i32) -> FlatMesh {
        FlatMesh {
            express_id,
            ..FlatMesh::default()
        }
    }

    pub fn GetMaxExpressID(&self, model_id: i32) -> Result<i32, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        Ok(model.max_express_id)
    }

    pub fn GetLineType(&self, model_id: i32, express_id: i32) -> Result<i32, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model
            .line_types
            .get(&express_id)
            .copied()
            .ok_or(IfcApiError::LineNotFound(express_id))
    }

    pub fn GetVersion(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    pub fn GetExpressIdFromGuid(
        &self,
        model_id: i32,
        guid: &str,
    ) -> Result<Option<i32>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let map = state.ifc_guid_map.get(&model_id);
        Ok(map.and_then(|inner| inner.get(guid).copied()))
    }

    pub fn GetGuidFromExpressId(
        &self,
        model_id: i32,
        express_id: i32,
    ) -> Result<Option<String>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let map = state.ifc_guid_map.get(&model_id);
        Ok(map.and_then(|inner| {
            inner
                .iter()
                .find(|(_, id)| **id == express_id)
                .map(|(k, _)| k.clone())
        }))
    }

    pub fn CreateIfcGuidToExpressIdMapping(&self, model_id: i32, data: HashMap<String, i32>) {
        let mut state = self.state.lock().expect("state lock");
        state.ifc_guid_map.insert(model_id, data);
    }

    pub fn SetWasmPath(&self, path: &str, is_absolute: bool) {
        if let Ok(mut wasm_path) = self.wasm_path.lock() {
            *wasm_path = path.to_string();
        }
        if let Ok(mut is_absolute_flag) = self.is_wasm_path_absolute.lock() {
            *is_absolute_flag = is_absolute;
        }
    }

    pub fn SetLogLevel(&self, level: LogLevel) {
        Log::set_log_level(level);
    }

    pub fn EncodeText(&self, text: &str) -> String {
        text.replace("'", "''")
    }

    pub fn DecodeText(&self, text: &str) -> String {
        text.replace("''", "'")
    }

    pub fn ResetCache(&self) {
        let mut state = self.state.lock().expect("state lock");
        state.deleted_lines.clear();
    }

    pub fn WriteLine(&self, model_id: i32, line: Value) -> Result<(), IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get_mut(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        let express_id = line
            .get("expressID")
            .and_then(|value| value.as_i64())
            .ok_or(IfcApiError::InvalidInput("expressID"))?;
        let type_code = line
            .get("type")
            .and_then(|value| value.as_i64())
            .ok_or(IfcApiError::InvalidInput("type"))?;
        model.lines.insert(express_id as i32, line);
        model.line_types.insert(express_id as i32, type_code as i32);
        if express_id as i32 > model.max_express_id {
            model.max_express_id = express_id as i32;
        }
        Ok(())
    }
}
