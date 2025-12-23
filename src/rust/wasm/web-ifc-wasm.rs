//! Rust port of `wasm/web-ifc-wasm.cpp`.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::api::helpers::log::LogLevel;
use crate::api::value::{Map, Value};
use crate::api::web_ifc_api::{
    Alignment, Arc, BooleanOperator, CircularSweep, Clothoid, CrossSection, CylindricalRevolve,
    Extrusion, FlatMesh, IfcAPI, IfcGeometry, LoaderSettings, ModelLoadCallback, ModelSaveCallback,
    NewIfcModel, Parabola, ProfileSection, Revolution, Sweep, AABB,
};
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum IfcTokenType {
    Unknown = 0,
    String = 1,
    Label = 2,
    Enum = 3,
    Real = 4,
    Ref = 5,
    Empty = 6,
    SetBegin = 7,
    SetEnd = 8,
    LineEnd = 9,
    Integer = 10,
}

#[derive(Clone, Debug)]
struct HeaderLine {
    type_code: u32,
    arguments: Value,
}

#[derive(Clone, Debug)]
struct LineRecord {
    type_code: u32,
    arguments: Value,
}

#[derive(Default)]
struct WasmState {
    api: IfcAPI,
    header_lines: HashMap<u32, Vec<HeaderLine>>,
    lines: HashMap<u32, HashMap<u32, LineRecord>>,
    current_args: HashMap<u32, Vec<Value>>,
    current_indices: HashMap<u32, usize>,
}

static STATE: OnceLock<Mutex<WasmState>> = OnceLock::new();

fn state() -> &'static Mutex<WasmState> {
    STATE.get_or_init(|| Mutex::new(WasmState::default()))
}

fn normalize_bool_like(value: &Value) -> Option<String> {
    match value {
        Value::Bool(true) => Some("T".to_string()),
        Value::Bool(false) => Some("F".to_string()),
        _ => None,
    }
}

fn normalize_argument(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            if let Some(Value::Number(type_code)) = map.get("type") {
                let token_type = *type_code as u32;
                if token_type == IfcTokenType::Label as u32 {
                    let label = map.get("label").cloned().unwrap_or(Value::Null);
                    let value_type = map.get("valueType").cloned().unwrap_or(Value::Null);
                    let value = map
                        .get("value")
                        .cloned()
                        .or_else(|| map.get("internalValue").cloned())
                        .unwrap_or(Value::Null);
                    let mut obj = Map::new();
                    obj.insert("type".to_string(), Value::Number(token_type as i64));
                    obj.insert("label".to_string(), label);
                    obj.insert("valueType".to_string(), value_type);
                    obj.insert("value".to_string(), value);
                    return Value::Object(obj);
                }
                if let Some(value) = map.get("value").or_else(|| map.get("internalValue")) {
                    let mut obj = Map::new();
                    obj.insert("type".to_string(), Value::Number(token_type as i64));
                    obj.insert("value".to_string(), value.clone());
                    return Value::Object(obj);
                }
            }
            Value::Object(map.clone())
        }
        Value::Array(values) => Value::Array(values.iter().map(normalize_argument).collect()),
        Value::String(text) => {
            if text == "true" {
                Value::String("T".to_string())
            } else if text == "false" {
                Value::String("F".to_string())
            } else {
                Value::String(text.clone())
            }
        }
        _ => value.clone(),
    }
}

/// C++ overload mapping: WriteValue validates input and maps it to IFC token semantics.
pub fn WriteValue(_model_id: u32, token: IfcTokenType, value: Value) -> bool {
    match token {
        IfcTokenType::String | IfcTokenType::Enum => match value {
            Value::Null => true,
            Value::Bool(_) => true,
            Value::String(_) => true,
            Value::Number(_) => true,
            _ => false,
        },
        IfcTokenType::Ref | IfcTokenType::Integer | IfcTokenType::Real => {
            matches!(value, Value::Number(_) | Value::String(_))
        }
        _ => false,
    }
}

/// C++ overload mapping: WriteSet handles nested argument lists.
pub fn WriteSet(model_id: u32, value: &Value) -> bool {
    let values = match value {
        Value::Array(values) => values,
        _ => return false,
    };

    let mut ok = true;
    for child in values {
        match child {
            Value::Null => {}
            Value::Array(_) => {
                if !WriteSet(model_id, child) {
                    ok = false;
                }
            }
            Value::Object(map) => {
                if let Some(Value::Number(token)) = map.get("type") {
                    let token_type = token_type_from_u32(*token as u32);
                    let value = map
                        .get("value")
                        .or_else(|| map.get("internalValue"))
                        .cloned()
                        .unwrap_or(Value::Null);
                    ok &= WriteValue(model_id, token_type, value);
                } else {
                    ok = false;
                }
            }
            _ => {
                let token_type = if matches!(child, Value::String(_)) {
                    IfcTokenType::String
                } else {
                    IfcTokenType::Real
                };
                ok &= WriteValue(model_id, token_type, child.clone());
            }
        }
    }

    ok
}

pub fn GetNameFromTypeCode(type_code: i32) -> String {
    let state = state().lock().expect("wasm state");
    state.api.GetNameFromTypeCode(type_code)
}

pub fn GetTypeCodeFromName(type_name: &str) -> i32 {
    let state = state().lock().expect("wasm state");
    state.api.GetTypeCodeFromName(type_name)
}

pub fn IsIfcElement(type_code: i32) -> bool {
    let state = state().lock().expect("wasm state");
    state.api.IsIfcElement(type_code)
}

pub fn CreateModel(_settings: LoaderSettings) -> i32 {
    let mut state = state().lock().expect("wasm state");
    let new_model = NewIfcModel {
        schema: "IFC4".to_string(),
        ..NewIfcModel::default()
    };
    let model_id = state.api.CreateModel(new_model).unwrap_or_default();
    state.lines.entry(model_id as u32).or_default();
    state.header_lines.entry(model_id as u32).or_default();
    model_id
}

pub fn CloseAllModels() {
    let mut state = state().lock().expect("wasm state");
    state.api.Dispose();
    state.lines.clear();
    state.header_lines.clear();
}

pub fn OpenModel(settings: LoaderSettings, callback: ModelLoadCallback, size: usize) -> i32 {
    let mut state = state().lock().expect("wasm state");
    let model_id = state
        .api
        .OpenModelFromCallback(callback, size, Some(settings))
        .unwrap_or_default();
    state.lines.entry(model_id as u32).or_default();
    state.header_lines.entry(model_id as u32).or_default();
    model_id
}

pub fn SaveModel(model_id: i32, callback: ModelSaveCallback) {
    let state = state().lock().expect("wasm state");
    let _ = state.api.SaveModelToCallback(model_id, callback);
}

pub fn GetModelSize(model_id: i32) -> usize {
    let state = state().lock().expect("wasm state");
    state
        .api
        .SaveModel(model_id)
        .map(|data| data.len())
        .unwrap_or(0)
}

pub fn CloseModel(model_id: i32) {
    let mut state = state().lock().expect("wasm state");
    let _ = state.api.CloseModel(model_id);
    state.lines.remove(&(model_id as u32));
    state.header_lines.remove(&(model_id as u32));
}

pub fn GetFlatMesh(model_id: i32, express_id: i32) -> FlatMesh {
    let state = state().lock().expect("wasm state");
    state.api.GetFlatMesh(model_id, express_id)
}

pub fn StreamMeshes<F>(model_id: i32, express_ids: &[i32], mut callback: F)
where
    F: FnMut(FlatMesh, i32, i32),
{
    let state = state().lock().expect("wasm state");
    let total = express_ids.len() as i32;
    for (index, express_id) in express_ids.iter().enumerate() {
        let mesh = state.api.GetFlatMesh(model_id, *express_id);
        if !mesh.geometries.0.is_empty() {
            callback(mesh, index as i32, total);
        }
    }
}

pub fn StreamAllMeshesWithTypes<F>(model_id: i32, types: &[i32], mut callback: F)
where
    F: FnMut(FlatMesh, i32, i32),
{
    let state = state().lock().expect("wasm state");
    let mut meshes: Vec<FlatMesh> = Vec::new();
    for type_code in types {
        if let Ok(ids) = state.api.GetLineIDsWithType(model_id, *type_code) {
            for id in ids {
                let mesh = state.api.GetFlatMesh(model_id, id);
                if !mesh.geometries.0.is_empty() {
                    meshes.push(mesh);
                }
            }
        }
    }
    let total = meshes.len() as i32;
    let mut index = 0i32;
    for mesh in meshes {
        callback(mesh, index, total);
        index += 1;
    }
}

pub fn StreamMeshesWithExpressID<F>(model_id: i32, express_ids: &[i32], callback: F)
where
    F: FnMut(FlatMesh, i32, i32),
{
    StreamMeshes(model_id, express_ids, callback);
}

pub fn StreamAllMeshesWithTypesVal<F>(model_id: i32, types: &[i32], callback: F)
where
    F: FnMut(FlatMesh, i32, i32),
{
    StreamAllMeshesWithTypes(model_id, types, callback);
}

pub fn StreamAllMeshes(model_id: i32) -> Vec<FlatMesh> {
    let state = state().lock().expect("wasm state");
    state.api.StreamAllMeshes(model_id)
}

pub fn LoadAllGeometry(model_id: i32) -> Vec<FlatMesh> {
    let state = state().lock().expect("wasm state");
    state.api.LoadAllGeometry(model_id)
}

pub fn GetGeometry(model_id: i32, express_id: i32) -> IfcGeometry {
    let state = state().lock().expect("wasm state");
    state
        .api
        .GetGeometry(model_id, express_id)
        .unwrap_or_default()
}

pub fn GetAllCrossSections(model_id: i32, dimensions: u8) -> Vec<CrossSection> {
    let state = state().lock().expect("wasm state");
    if dimensions == 2 {
        state.api.GetAllCrossSections2D(model_id)
    } else {
        state.api.GetAllCrossSections3D(model_id)
    }
}

pub fn GetAllAlignments(model_id: i32) -> Vec<Alignment> {
    let state = state().lock().expect("wasm state");
    state.api.GetAllAlignments(model_id)
}

pub fn SetGeometryTransformation(model_id: i32, transformation: &[f64]) {
    let state = state().lock().expect("wasm state");
    state
        .api
        .SetGeometryTransformation(model_id, transformation);
}

pub fn GetCoordinationMatrix(model_id: i32) -> Vec<f64> {
    let state = state().lock().expect("wasm state");
    state.api.GetCoordinationMatrix(model_id)
}

pub fn GetWorldTransformMatrix(model_id: i32, _placement_express_id: u32) -> Vec<f64> {
    let state = state().lock().expect("wasm state");
    state.api.GetWorldTransformMatrix(model_id)
}

pub fn GetLineIDsWithType(model_id: i32, type_codes: &[i32]) -> Vec<i32> {
    let state = state().lock().expect("wasm state");
    let Some(lines) = state.lines.get(&(model_id as u32)) else {
        return Vec::new();
    };
    let mut ids: Vec<i32> = lines
        .iter()
        .filter_map(|(express_id, record)| {
            if type_codes.contains(&(record.type_code as i32)) {
                Some(*express_id as i32)
            } else {
                None
            }
        })
        .collect();
    ids.sort_unstable();
    ids
}

pub fn GetInversePropertyForItem(
    model_id: u32,
    express_id: u32,
    target_types: &[i32],
    position: u32,
    set: bool,
) -> Vec<u32> {
    let state = state().lock().expect("wasm state");
    let mut inverse_ids = Vec::new();
    let lines = match state.lines.get(&model_id) {
        Some(lines) => lines,
        None => return inverse_ids,
    };

    for (found_id, line) in lines {
        if !target_types.is_empty() && !target_types.contains(&(line.type_code as i32)) {
            continue;
        }

        if let Some(arg) = argument_at(&line.arguments, position as usize) {
            if contains_ref(arg, express_id) {
                inverse_ids.push(*found_id);
                if !set {
                    return inverse_ids;
                }
            }
        }
    }

    inverse_ids
}

pub fn ValidateExpressID(model_id: i32, express_id: i32) -> bool {
    let state = state().lock().expect("wasm state");
    let Some(lines) = state.lines.get(&(model_id as u32)) else {
        return false;
    };
    lines.contains_key(&(express_id as u32))
}

pub fn GetNextExpressID(model_id: i32, _express_id: i32) -> i32 {
    let express_id = _express_id;
    let state = state().lock().expect("wasm state");
    let Some(lines) = state.lines.get(&(model_id as u32)) else {
        return 0;
    };
    let mut ids: Vec<i32> = lines.keys().map(|id| *id as i32).collect();
    ids.sort_unstable();
    ids.into_iter().find(|id| *id > express_id).unwrap_or(0)
}

pub fn GetAllLines(model_id: i32) -> Vec<i32> {
    let state = state().lock().expect("wasm state");
    let Some(lines) = state.lines.get(&(model_id as u32)) else {
        return Vec::new();
    };
    let mut ids: Vec<i32> = lines.keys().map(|id| *id as i32).collect();
    ids.sort_unstable();
    ids
}

pub fn ReadValue(model_id: u32, token: IfcTokenType) -> Value {
    let mut state = state().lock().expect("wasm state");
    let args = state
        .current_args
        .get(&model_id)
        .cloned()
        .unwrap_or_default();
    let index = state.current_indices.entry(model_id).or_insert(0);
    if *index >= args.len() {
        return Value::Null;
    }
    let value = args[*index].clone();
    *index += 1;
    match token {
        IfcTokenType::String | IfcTokenType::Enum => match extract_value(&value) {
            Value::String(text) => {
                if text == "T" {
                    Value::Bool(true)
                } else if text == "F" {
                    Value::Bool(false)
                } else if text == "U" {
                    Value::Null
                } else {
                    Value::String(text)
                }
            }
            Value::Bool(flag) => Value::Bool(flag),
            Value::Null => Value::Null,
            other => other,
        },
        IfcTokenType::Real => match extract_value(&value) {
            Value::Number(num) => Value::String(num.to_string()),
            Value::String(text) => Value::String(text),
            other => other,
        },
        IfcTokenType::Integer | IfcTokenType::Ref => match extract_value(&value) {
            Value::Number(num) => Value::Number(num),
            Value::String(text) => text
                .parse::<i64>()
                .map(Value::Number)
                .unwrap_or(Value::Null),
            other => other,
        },
        _ => Value::Null,
    }
}

pub fn GetArgs(model_id: u32, in_object: bool, in_list: bool) -> Value {
    let state = state().lock().expect("wasm state");
    let args = state
        .current_args
        .get(&model_id)
        .cloned()
        .unwrap_or_default();
    if args.is_empty() && !in_list {
        return Value::Null;
    }
    let values: Vec<Value> = args
        .into_iter()
        .map(|arg| normalize_argument(&arg))
        .collect();
    if values.len() == 1 && in_object {
        return values[0].clone();
    }
    Value::Array(values)
}

pub fn WriteHeaderLine(model_id: u32, type_code: u32, parameters: Value) -> bool {
    let mut state = state().lock().expect("wasm state");
    if !state.api.IsModelOpen(model_id as i32) {
        return false;
    }
    let args = normalize_argument(&parameters);
    let record = HeaderLine {
        type_code,
        arguments: args,
    };
    state.header_lines.entry(model_id).or_default().push(record);
    true
}

pub fn RemoveLine(model_id: u32, express_id: u32) {
    let mut state = state().lock().expect("wasm state");
    if let Some(lines) = state.lines.get_mut(&model_id) {
        lines.remove(&express_id);
    }
}

pub fn WriteLine(model_id: u32, express_id: u32, type_code: u32, parameters: Value) -> bool {
    let mut state = state().lock().expect("wasm state");
    if !state.api.IsModelOpen(model_id as i32) {
        return false;
    }
    let args = normalize_argument(&parameters);
    let record = LineRecord {
        type_code,
        arguments: args.clone(),
    };
    state
        .lines
        .entry(model_id)
        .or_default()
        .insert(express_id, record);

    let mut line_map = Map::new();
    line_map.insert("expressID".to_string(), Value::Number(express_id as i64));
    line_map.insert("type".to_string(), Value::Number(type_code as i64));
    line_map.insert("arguments".to_string(), args);
    let _ = state
        .api
        .WriteLine(model_id as i32, Value::Object(line_map));
    true
}

pub fn GetHeaderLine(model_id: u32, header_type: u32) -> Value {
    let mut state = state().lock().expect("wasm state");
    let lines = state.header_lines.get(&model_id).cloned();
    let Some(lines) = lines else {
        return Value::Null;
    };
    let Some((index, line)) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.type_code == header_type)
    else {
        return Value::Null;
    };

    state.current_args.insert(
        model_id,
        match &line.arguments {
            Value::Array(values) => values.clone(),
            value => vec![value.clone()],
        },
    );
    state.current_indices.insert(model_id, 0);
    let arguments = GetArgs(model_id, false, false);

    let mut obj = Map::new();
    obj.insert("ID".to_string(), Value::Number(index as i64));
    obj.insert(
        "type".to_string(),
        Value::String(state.api.GetNameFromTypeCode(header_type as i32)),
    );
    obj.insert("arguments".to_string(), arguments);
    Value::Object(obj)
}

pub fn GetLine(model_id: u32, express_id: u32) -> Value {
    let mut state = state().lock().expect("wasm state");
    let Some(lines) = state.lines.get(&model_id).cloned() else {
        return Value::Object(Map::new());
    };
    let Some(line) = lines.get(&express_id) else {
        return Value::Object(Map::new());
    };
    state.current_args.insert(
        model_id,
        match &line.arguments {
            Value::Array(values) => values.clone(),
            value => vec![value.clone()],
        },
    );
    state.current_indices.insert(model_id, 0);
    let arguments = GetArgs(model_id, false, false);

    let mut obj = Map::new();
    obj.insert("ID".to_string(), Value::Number(express_id as i64));
    obj.insert("type".to_string(), Value::Number(line.type_code as i64));
    obj.insert("arguments".to_string(), arguments);
    Value::Object(obj)
}

pub fn GetLines(model_id: u32, express_ids: &[u32]) -> Vec<Value> {
    express_ids
        .iter()
        .map(|id| GetLine(model_id, *id))
        .collect()
}

pub fn GetLineType(model_id: i32, express_id: i32) -> i32 {
    let state = state().lock().expect("wasm state");
    state
        .api
        .GetLineType(model_id, express_id)
        .unwrap_or_default()
}

pub fn EncodeText(text: &str) -> String {
    let state = state().lock().expect("wasm state");
    state.api.EncodeText(text)
}

pub fn DecodeText(text: &str) -> String {
    let state = state().lock().expect("wasm state");
    state.api.DecodeText(text)
}

pub fn GetVersion() -> String {
    let state = state().lock().expect("wasm state");
    state.api.GetVersion()
}

pub fn GenerateGuid(model_id: i32) -> String {
    let state = state().lock().expect("wasm state");
    let value = state.api.CreateIFCGloballyUniqueId(model_id);
    value
        .get("value")
        .and_then(|entry| entry.as_str())
        .unwrap_or_default()
        .to_string()
}

pub fn GetMaxExpressID(model_id: i32) -> i32 {
    let state = state().lock().expect("wasm state");
    state.api.GetMaxExpressID(model_id).unwrap_or_default()
}

pub fn IsModelOpen(model_id: i32) -> bool {
    let state = state().lock().expect("wasm state");
    state.api.IsModelOpen(model_id)
}

pub fn SetLogLevel(level: u8) {
    let api_level = match level {
        0 | 1 => LogLevel::LOG_LEVEL_DEBUG,
        2 | 3 => LogLevel::LOG_LEVEL_WARN,
        4 | 5 => LogLevel::LOG_LEVEL_ERROR,
        _ => LogLevel::LOG_LEVEL_OFF,
    };
    let state = state().lock().expect("wasm state");
    state.api.SetLogLevel(api_level);
}

pub fn ResetCache(model_id: i32) {
    let state = state().lock().expect("wasm state");
    state.api.ResetCache();
    let _ = model_id;
}

pub fn CreateAABB() -> AABB {
    let state = state().lock().expect("wasm state");
    state.api.CreateAABB()
}

pub fn CreateExtrusion() -> Extrusion {
    let state = state().lock().expect("wasm state");
    state.api.CreateExtrusion()
}

pub fn CreateSweep() -> Sweep {
    let state = state().lock().expect("wasm state");
    state.api.CreateSweep()
}

pub fn CreateCircularSweep() -> CircularSweep {
    let state = state().lock().expect("wasm state");
    state.api.CreateCircularSweep()
}

pub fn CreateRevolution() -> Revolution {
    let state = state().lock().expect("wasm state");
    state.api.CreateRevolution()
}

pub fn CreateCylindricalRevolution() -> CylindricalRevolve {
    let state = state().lock().expect("wasm state");
    state.api.CreateCylindricalRevolution()
}

pub fn CreateParabola() -> Parabola {
    let state = state().lock().expect("wasm state");
    state.api.CreateParabola()
}

pub fn CreateClothoid() -> Clothoid {
    let state = state().lock().expect("wasm state");
    state.api.CreateClothoid()
}

pub fn CreateArc() -> Arc {
    let state = state().lock().expect("wasm state");
    state.api.CreateArc()
}

pub fn CreateAlignment() -> Alignment {
    let state = state().lock().expect("wasm state");
    state.api.CreateAlignment()
}

pub fn CreateBoolean() -> BooleanOperator {
    let state = state().lock().expect("wasm state");
    state.api.CreateBooleanOperator()
}

pub fn CreateProfile() -> ProfileSection {
    let state = state().lock().expect("wasm state");
    state.api.CreateProfile()
}

fn token_type_from_u32(value: u32) -> IfcTokenType {
    match value {
        x if x == IfcTokenType::Unknown as u32 => IfcTokenType::Unknown,
        x if x == IfcTokenType::String as u32 => IfcTokenType::String,
        x if x == IfcTokenType::Label as u32 => IfcTokenType::Label,
        x if x == IfcTokenType::Enum as u32 => IfcTokenType::Enum,
        x if x == IfcTokenType::Real as u32 => IfcTokenType::Real,
        x if x == IfcTokenType::Ref as u32 => IfcTokenType::Ref,
        x if x == IfcTokenType::Empty as u32 => IfcTokenType::Empty,
        x if x == IfcTokenType::SetBegin as u32 => IfcTokenType::SetBegin,
        x if x == IfcTokenType::SetEnd as u32 => IfcTokenType::SetEnd,
        x if x == IfcTokenType::LineEnd as u32 => IfcTokenType::LineEnd,
        x if x == IfcTokenType::Integer as u32 => IfcTokenType::Integer,
        _ => IfcTokenType::Unknown,
    }
}

fn extract_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => map
            .get("value")
            .or_else(|| map.get("internalValue"))
            .cloned()
            .unwrap_or(Value::Null),
        _ => value.clone(),
    }
}

fn argument_at(value: &Value, position: usize) -> Option<&Value> {
    match value {
        Value::Array(values) => values.get(position),
        _ => None,
    }
}

fn contains_ref(value: &Value, express_id: u32) -> bool {
    match value {
        Value::Number(num) => *num == express_id as i64,
        Value::Object(map) => {
            if let Some(Value::Number(token)) = map.get("type") {
                if *token as u32 == IfcTokenType::Ref as u32 {
                    if let Some(Value::Number(val)) = map.get("value") {
                        return *val == express_id as i64;
                    }
                    if let Some(Value::String(val)) = map.get("value") {
                        return val.parse::<i64>().ok() == Some(express_id as i64);
                    }
                }
            }
            map.values().any(|v| contains_ref(v, express_id))
        }
        Value::Array(values) => values.iter().any(|v| contains_ref(v, express_id)),
        _ => false,
    }
}
