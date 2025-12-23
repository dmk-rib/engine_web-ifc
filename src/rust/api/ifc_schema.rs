//! Web-IFC IFC Schema Representation

use std::marker::PhantomData;
use std::sync::OnceLock;

use crate::api::value::{Map, Value};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IfcEntity {
    pub attributes: Map,
}

impl IfcEntity {
    pub fn new() -> Self {
        Self {
            attributes: Map::new(),
        }
    }

    pub fn with_attribute(mut self, key: &str, value: Value) -> Self {
        self.attributes.insert(key.to_string(), value);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handle<T> {
    pub value: i64,
    pub schema: u8,
    pub tape_item: Option<Value>,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(value: i64, schema: u8, tape_item: Option<Value>) -> Self {
        Self {
            value,
            schema,
            tape_item,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberHandle {
    pub value: f64,
    pub type_code: u32,
}

impl NumberHandle {
    pub fn new(value: f64, type_code: u32) -> Self {
        Self { value, type_code }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum logical {
    FALSE,
    TRUE,
    UNKNOWN,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IfcLineObject {
    pub express_id: i64,
    pub attributes: Map,
}

impl IfcLineObject {
    pub fn new(express_id: i64) -> Self {
        Self {
            express_id,
            attributes: Map::new(),
        }
    }

    pub fn with_attribute(mut self, key: &str, value: Value) -> Self {
        self.attributes.insert(key.to_string(), value);
        self
    }
}

pub static FromRawLineData: OnceLock<Value> = OnceLock::new();
pub static InversePropertyDef: OnceLock<Value> = OnceLock::new();
pub static InheritanceDef: OnceLock<Value> = OnceLock::new();
pub static Constructors: OnceLock<Value> = OnceLock::new();
pub static ToRawLineData: OnceLock<Value> = OnceLock::new();
pub static TypeInitialisers: OnceLock<Value> = OnceLock::new();
pub static SchemaNames: OnceLock<Vec<Vec<String>>> = OnceLock::new();

pub fn schema_names() -> &'static Vec<Vec<String>> {
    SchemaNames.get_or_init(|| {
        IFC_SCHEMA_NAMES
            .iter()
            .map(|names| names.iter().map(|name| name.to_string()).collect())
            .collect()
    })
}

pub fn lookup_schema_id(schema_name: &str) -> Option<usize> {
    IFC_SCHEMA_NAMES
        .iter()
        .enumerate()
        .find(|(_, names)| {
            names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(schema_name))
        })
        .map(|(index, _)| index)
}

pub fn type_name_from_code(type_code: i32) -> Option<&'static str> {
    IFC_SCHEMA_CONSTANTS
        .iter()
        .find(|(_, value)| *value as i32 == type_code)
        .map(|(name, _)| *name)
}

pub fn type_code_from_name(type_name: &str) -> Option<i32> {
    IFC_SCHEMA_CONSTANTS
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(type_name))
        .map(|(_, value)| *value as i32)
}

pub fn is_ifc_element(type_code: i32) -> bool {
    type_name_from_code(type_code).is_some()
}

include!(concat!(env!("OUT_DIR"), "/ifc_schema_generated.rs"));
