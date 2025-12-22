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

include!(concat!(env!("OUT_DIR"), "/ifc_schema_generated.rs"));
