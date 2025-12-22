//! Rust port of IfcLoader public API.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;

use super::ifc_token_stream::{IfcTokenStream, IfcTokenType};
use crate::web_ifc::schema::ifc_schema::IFC_SCHEMA;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;

#[derive(Debug, Clone, Copy)]
struct IfcLine {
    ifc_type: u32,
    tape_offset: u32,
}

#[derive(Debug)]
pub struct IfcLoader {
    max_express_id: u32,
    line_writer_buffer: u32,
    schema_manager: Arc<IfcSchemaManager>,
    token_stream: IfcTokenStream,
    lines: HashMap<u32, IfcLine>,
    header_lines: Vec<IfcLine>,
    ifc_type_to_express_id: HashMap<u32, Vec<u32>>, 
}

impl IfcLoader {
    pub fn new(
        tape_size: u32,
        memory_limit: u64,
        line_writer_buffer: u32,
        schema_manager: Arc<IfcSchemaManager>,
    ) -> Self {
        let _ = memory_limit;
        Self {
            max_express_id: 0,
            line_writer_buffer,
            schema_manager,
            token_stream: IfcTokenStream::new(tape_size as usize, memory_limit),
            lines: HashMap::new(),
            header_lines: Vec::new(),
            ifc_type_to_express_id: HashMap::new(),
        }
    }

    pub fn get_header_lines_with_type(&self, _type_code: u32) -> Vec<u32> {
        unimplemented!("get_header_lines_with_type not yet implemented");
    }

    pub fn load_file_with_callback(
        &mut self,
        _request_data: Arc<dyn Fn(&mut [u8], usize, usize) -> u32 + Send + Sync>,
    ) {
        unimplemented!("load_file_with_callback not yet implemented");
    }

    pub fn load_file<R: Read>(&mut self, _request_data: R) {
        unimplemented!("load_file not yet implemented");
    }

    pub fn save_file_with_callback(
        &self,
        _output_data: Arc<dyn Fn(&mut [u8], usize) + Send + Sync>,
        _order_lines_by_express_id: bool,
    ) {
        unimplemented!("save_file_with_callback not yet implemented");
    }

    pub fn save_file<W: Write>(&self, _output_data: W, _order_lines_by_express_id: bool) {
        unimplemented!("save_file not yet implemented");
    }

    pub fn get_express_ids_with_type(&self, _type_code: u32) -> Vec<u32> {
        unimplemented!("get_express_ids_with_type not yet implemented");
    }

    pub fn get_max_express_id(&self) -> u32 {
        self.max_express_id
    }

    pub fn is_valid_express_id(&self, _express_id: u32) -> bool {
        unimplemented!("is_valid_express_id not yet implemented");
    }

    pub fn get_line_type(&self, _express_id: u32) -> u32 {
        unimplemented!("get_line_type not yet implemented");
    }

    pub fn is_at_end(&self) -> bool {
        self.token_stream.is_at_end()
    }

    pub fn move_to_line_argument(&self, _express_id: u32, _argument_index: u32) {
        unimplemented!("move_to_line_argument not yet implemented");
    }

    pub fn move_to_header_line_argument(&self, _line_id: u32, _argument_index: u32) {
        unimplemented!("move_to_header_line_argument not yet implemented");
    }

    pub fn get_string_argument(&self) -> &str {
        unimplemented!("get_string_argument not yet implemented");
    }

    pub fn get_decoded_string_argument(&self) -> String {
        unimplemented!("get_decoded_string_argument not yet implemented");
    }

    pub fn get_expanded_uuid_argument(&self) -> String {
        unimplemented!("get_expanded_uuid_argument not yet implemented");
    }

    pub fn get_double_argument(&self) -> f64 {
        unimplemented!("get_double_argument not yet implemented");
    }

    pub fn get_int_argument(&self) -> i64 {
        unimplemented!("get_int_argument not yet implemented");
    }

    pub fn get_int_argument_at(&self, _tape_offset: u32) -> i64 {
        unimplemented!("get_int_argument_at not yet implemented");
    }

    pub fn get_double_argument_at(&self, _tape_offset: u32) -> f64 {
        unimplemented!("get_double_argument_at not yet implemented");
    }

    pub fn get_double_argument_as_string(&self) -> &str {
        unimplemented!("get_double_argument_as_string not yet implemented");
    }

    pub fn get_optional_double_param(&self, _default_value: f64) -> f64 {
        unimplemented!("get_optional_double_param not yet implemented");
    }

    pub fn get_ref_argument(&self) -> u32 {
        unimplemented!("get_ref_argument not yet implemented");
    }

    pub fn get_ref_argument_at(&self, _tape_offset: u32) -> u32 {
        unimplemented!("get_ref_argument_at not yet implemented");
    }

    pub fn get_optional_ref_argument(&self) -> u32 {
        unimplemented!("get_optional_ref_argument not yet implemented");
    }

    pub fn get_token_type(&self) -> IfcTokenType {
        unimplemented!("get_token_type not yet implemented");
    }

    pub fn get_token_type_at(&self, _tape_offset: u32) -> IfcTokenType {
        unimplemented!("get_token_type_at not yet implemented");
    }

    pub fn get_set_argument(&self) -> Vec<u32> {
        unimplemented!("get_set_argument not yet implemented");
    }

    pub fn get_all_lines(&self) -> Vec<u32> {
        unimplemented!("get_all_lines not yet implemented");
    }

    pub fn get_set_list_argument(&self) -> Vec<Vec<u32>> {
        unimplemented!("get_set_list_argument not yet implemented");
    }

    pub fn move_to_argument_offset(&self, _express_id: u32, _argument_index: u32) {
        unimplemented!("move_to_argument_offset not yet implemented");
    }

    pub fn get_no_line_arguments(&self, _express_id: u32) -> u32 {
        unimplemented!("get_no_line_arguments not yet implemented");
    }

    pub fn step_back(&self) {
        unimplemented!("step_back not yet implemented");
    }

    pub fn get_schema(&self) -> IFC_SCHEMA {
        unimplemented!("get_schema not yet implemented");
    }

    pub fn push_bytes(&mut self, _data: &[u8]) {
        unimplemented!("push_bytes not yet implemented");
    }

    pub fn get_total_size(&self) -> u64 {
        self.token_stream.get_total_size() as u64
    }

    pub fn update_line_tape(&mut self, _express_id: u32, _type_code: u32, _start: u32) {
        unimplemented!("update_line_tape not yet implemented");
    }

    pub fn add_header_line_tape(&mut self, _type_code: u32, _start: u32) {
        unimplemented!("add_header_line_tape not yet implemented");
    }

    pub fn get_current_line_express_id(&self) -> u32 {
        unimplemented!("get_current_line_express_id not yet implemented");
    }

    pub fn remove_line(&mut self, _express_id: u32) {
        unimplemented!("remove_line not yet implemented");
    }

    pub fn push_double(&mut self, _input: f64) {
        unimplemented!("push_double not yet implemented");
    }

    pub fn push_int(&mut self, _input: i32) {
        unimplemented!("push_int not yet implemented");
    }

    pub fn generate_uuid(&self) -> String {
        unimplemented!("generate_uuid not yet implemented");
    }

    pub fn clone_loader(&self) -> Self {
        Self {
            max_express_id: self.max_express_id,
            line_writer_buffer: self.line_writer_buffer,
            schema_manager: Arc::clone(&self.schema_manager),
            token_stream: self.token_stream.clone_stream(),
            lines: self.lines.clone(),
            header_lines: self.header_lines.clone(),
            ifc_type_to_express_id: self.ifc_type_to_express_id.clone(),
        }
    }

    pub fn get_next_express_id(&self, _express_id: u32) -> u32 {
        unimplemented!("get_next_express_id not yet implemented");
    }
}
