//! Rust port of IfcLoader public API.

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::Arc;

use super::ifc_file_stream::DataSource;
use super::ifc_token_stream::{IfcTokenStream, IfcTokenType};
use super::string_parsing::{p21decode, p21encode};
use super::uuid_utils::{compress_ifc_guid, expand_ifc_guid, generate_string_uuid};
use crate::api::helpers::log::Log;
use crate::web_ifc::schema::ifc_schema;
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
    token_stream: UnsafeCell<IfcTokenStream>,
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
        let max_chunks = if memory_limit > 0 {
            memory_limit / tape_size as u64
        } else {
            0
        };
        Self {
            max_express_id: 0,
            line_writer_buffer,
            schema_manager,
            token_stream: UnsafeCell::new(IfcTokenStream::new(tape_size as usize, max_chunks)),
            lines: HashMap::new(),
            header_lines: Vec::new(),
            ifc_type_to_express_id: HashMap::new(),
        }
    }

    pub fn get_header_lines_with_type(&self, type_code: u32) -> Vec<u32> {
        let mut ret = Vec::new();
        for (index, line) in self.header_lines.iter().enumerate() {
            if line.ifc_type == type_code {
                ret.push(index as u32);
            }
        }
        ret
    }

    pub fn load_file_with_callback(
        &mut self,
        request_data: Arc<dyn Fn(&mut [u8], usize, usize) -> u32 + Send + Sync>,
    ) {
        let source: DataSource = Rc::new(move |dest: &mut [u8], offset: usize, size: usize| {
            (request_data)(dest, offset, size)
        });
        self.token_stream_mut().set_token_source(source);
        self.parse_lines();
    }

    pub fn load_file<R: Read>(&mut self, request_data: R) {
        self.token_stream_mut()
            .set_token_source_reader(request_data);
        self.parse_lines();
    }

    pub fn save_file_with_callback(
        &self,
        output_data: Arc<dyn Fn(&mut [u8], usize) + Send + Sync>,
        order_lines_by_express_id: bool,
    ) {
        let mut output = String::new();
        let _ = writeln!(output, "ISO-10303-21;");
        let _ = writeln!(output, "HEADER;");
        let _ = writeln!(
            output,
            "/******************************************************"
        );
        let _ = writeln!(
            output,
            "* STEP Physical File produced by: That Open Engine WebIfc {}",
            crate::version::WEB_IFC_VERSION_NUMBER
        );
        let _ = writeln!(output, "* Module: web-ifc/IfcLoader");
        let _ = writeln!(
            output,
            "* Version: {}",
            crate::version::WEB_IFC_VERSION_NUMBER
        );
        let _ = writeln!(
            output,
            "* Source: https://github.com/ThatOpen/engine_web-ifc"
        );
        let _ = writeln!(
            output,
            "* Issues: https://github.com/ThatOpen/engine_web-ifc/issues"
        );
        let _ = writeln!(
            output,
            "******************************************************/"
        );

        let mut lines_written: u32 = 0;
        for z in 0..2u8 {
            let mut current_lines: Vec<&IfcLine> = if z == 0 {
                self.header_lines.iter().collect()
            } else {
                self.lines.values().collect()
            };

            if order_lines_by_express_id {
                current_lines.sort_by_key(|line| line.tape_offset);
            }

            for line in current_lines {
                if line.ifc_type == 0 {
                    continue;
                }

                let token_stream = self.token_stream_mut();
                token_stream.move_to(line.tape_offset as usize);
                let mut new_line = true;
                let mut inside_set = false;
                let mut prev = IfcTokenType::Empty;

                while !token_stream.is_at_end() {
                    let t = token_type_from(token_stream.read::<u8>());

                    if t != IfcTokenType::SetEnd && t != IfcTokenType::LineEnd {
                        if inside_set
                            && prev != IfcTokenType::SetBegin
                            && prev != IfcTokenType::Label
                            && prev != IfcTokenType::LineEnd
                        {
                            output.push(',');
                        }
                    }

                    if t == IfcTokenType::LineEnd {
                        output.push_str(";\n");
                        break;
                    }

                    match t {
                        IfcTokenType::Unknown => {
                            output.push('*');
                        }
                        IfcTokenType::Empty => {
                            output.push('$');
                        }
                        IfcTokenType::SetBegin => {
                            output.push('(');
                            inside_set = true;
                        }
                        IfcTokenType::SetEnd => {
                            output.push(')');
                        }
                        IfcTokenType::String => {
                            output.push('\'');
                            let raw = token_stream.read_string();
                            let encoded = p21encode(raw).unwrap_or_default();
                            output.push_str(&encoded);
                            output.push('\'');
                        }
                        IfcTokenType::Enum => {
                            output.push('.');
                            output.push_str(token_stream.read_string());
                            output.push('.');
                        }
                        IfcTokenType::Ref => {
                            let value = token_stream.read::<u32>();
                            let _ = write!(output, "#{value}");
                            if new_line {
                                output.push('=');
                            }
                        }
                        IfcTokenType::Label | IfcTokenType::Real | IfcTokenType::Integer => {
                            output.push_str(token_stream.read_string());
                        }
                        _ => {}
                    }

                    if t == IfcTokenType::LineEnd {
                        new_line = true;
                        inside_set = false;
                    } else {
                        new_line = false;
                    }
                    prev = t;
                }

                lines_written += 1;
                if lines_written > self.line_writer_buffer {
                    let mut bytes = output.into_bytes();
                    (output_data)(&mut bytes, bytes.len());
                    output = String::new();
                    lines_written = 0;
                }
            }

            if z == 0 {
                output.push_str("ENDSEC;\nDATA;\n");
            }
        }

        output.push_str("ENDSEC;\nEND-ISO-10303-21;");
        let mut bytes = output.into_bytes();
        (output_data)(&mut bytes, bytes.len());
    }

    pub fn save_file<W: Write>(&self, output_data: W, order_lines_by_express_id: bool) {
        let mut output_data = output_data;
        self.save_file_with_callback(
            Arc::new(move |src: &mut [u8], src_size: usize| {
                let _ = output_data.write_all(&src[..src_size]);
            }),
            order_lines_by_express_id,
        );
    }

    pub fn get_express_ids_with_type(&self, type_code: u32) -> Vec<u32> {
        self.ifc_type_to_express_id
            .get(&type_code)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_max_express_id(&self) -> u32 {
        self.max_express_id
    }

    pub fn is_valid_express_id(&self, express_id: u32) -> bool {
        if express_id == 0
            || express_id > self.max_express_id
            || !self.lines.contains_key(&express_id)
        {
            false
        } else {
            true
        }
    }

    pub fn get_line_type(&self, express_id: u32) -> u32 {
        if express_id == 0 || express_id > self.max_express_id {
            let expr = express_id.to_string();
            Log::error(
                "[GetLineType()] Attempt to Access Invalid ExpressID",
                &[expr.as_str()],
            );
            return 0;
        }

        match self.lines.get(&express_id) {
            Some(line) => line.ifc_type,
            None => {
                let expr = express_id.to_string();
                Log::error(
                    "[GetLineType()] Attempt to Access Invalid ExpressID",
                    &[expr.as_str()],
                );
                0
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.token_stream_ref().is_at_end()
    }

    pub fn move_to_line_argument(&self, express_id: u32, argument_index: u32) {
        let Some(line) = self.lines.get(&express_id) else {
            return;
        };
        self.token_stream_mut().move_to(line.tape_offset as usize);
        self.argument_offset(argument_index);
    }

    pub fn move_to_header_line_argument(&self, line_id: u32, argument_index: u32) {
        let line = &self.header_lines[line_id as usize];
        self.token_stream_mut().move_to(line.tape_offset as usize);
        self.argument_offset(argument_index);
    }

    pub fn get_string_argument(&self) -> &str {
        let token_stream = self.token_stream_mut();
        token_stream.read::<u8>();
        token_stream.read_string()
    }

    pub fn get_decoded_string_argument(&self) -> String {
        let str_value = self.get_string_argument();
        p21decode(str_value).unwrap_or_default()
    }

    pub fn get_expanded_uuid_argument(&self) -> String {
        expand_ifc_guid(self.get_string_argument())
    }

    pub fn get_double_argument(&self) -> f64 {
        let str_value = self.get_string_argument();
        str_value.parse::<f64>().unwrap_or(0.0)
    }

    pub fn get_int_argument(&self) -> i64 {
        let str_value = self.get_string_argument();
        str_value.parse::<i64>().unwrap_or(0)
    }

    // C++ overload mapping: GetIntArgument(tapeOffset).
    pub fn get_int_argument_at(&self, tape_offset: u32) -> i64 {
        self.token_stream_mut().move_to(tape_offset as usize);
        self.get_int_argument()
    }

    // C++ overload mapping: GetDoubleArgument(tapeOffset).
    pub fn get_double_argument_at(&self, tape_offset: u32) -> f64 {
        self.token_stream_mut().move_to(tape_offset as usize);
        self.get_double_argument()
    }

    pub fn get_double_argument_as_string(&self) -> &str {
        self.get_string_argument()
    }

    pub fn get_optional_double_param(&self, default_value: f64) -> f64 {
        let token = self.get_token_type();
        if token == IfcTokenType::Real {
            self.step_back();
            return self.get_double_argument();
        }
        if token == IfcTokenType::Integer {
            self.step_back();
            return self.get_int_argument() as f64;
        }
        default_value
    }

    pub fn get_ref_argument(&self) -> u32 {
        if token_type_from(self.token_stream_mut().read::<u8>()) != IfcTokenType::Ref {
            let expr = self.get_current_line_express_id().to_string();
            Log::error(
                "[GetRefArgument()] unexpected token type, expected REF",
                &[expr.as_str()],
            );
            return 0;
        }
        self.token_stream_mut().read::<u32>()
    }

    // C++ overload mapping: GetRefArgument(tapeOffset).
    pub fn get_ref_argument_at(&self, tape_offset: u32) -> u32 {
        self.token_stream_mut().move_to(tape_offset as usize);
        self.get_ref_argument()
    }

    pub fn get_optional_ref_argument(&self) -> u32 {
        let token = self.get_token_type();
        if token == IfcTokenType::Empty {
            return 0;
        }
        if token == IfcTokenType::Ref {
            return self.token_stream_mut().read::<u32>();
        }
        let expr = self.get_current_line_express_id().to_string();
        Log::error(
            "[GetOptionalRefArgument()] unexpected token type, expected REF or EMPTY",
            &[expr.as_str()],
        );
        0
    }

    pub fn get_token_type(&self) -> IfcTokenType {
        token_type_from(self.token_stream_mut().read::<u8>())
    }

    // C++ overload mapping: GetTokenType(tapeOffset).
    pub fn get_token_type_at(&self, tape_offset: u32) -> IfcTokenType {
        self.token_stream_mut().move_to(tape_offset as usize);
        self.get_token_type()
    }

    pub fn get_set_argument(&self) -> Vec<u32> {
        let token_stream = self.token_stream_mut();
        let mut tape_offsets: Vec<u32> = Vec::with_capacity(4);
        token_stream.read::<u8>();
        let mut depth = 1i32;
        while depth > 0 {
            let offset = token_stream.get_read_offset() as u32;
            let t = token_type_from(token_stream.read::<u8>());

            match t {
                IfcTokenType::SetBegin => depth += 1,
                IfcTokenType::SetEnd => depth -= 1,
                IfcTokenType::Ref => {
                    tape_offsets.push(offset);
                    token_stream.read::<u32>();
                }
                IfcTokenType::String
                | IfcTokenType::Integer
                | IfcTokenType::Real
                | IfcTokenType::Label
                | IfcTokenType::Enum => {
                    tape_offsets.push(offset);
                    let length = token_stream.read::<u16>() as usize;
                    token_stream.forward(length);
                }
                _ => {
                    let expr = self.get_current_line_express_id().to_string();
                    Log::error("[GetSetArgument[]) unexpected token", &[expr.as_str()]);
                }
            }
        }
        tape_offsets
    }

    pub fn get_all_lines(&self) -> Vec<u32> {
        self.lines.keys().copied().collect()
    }

    pub fn get_set_list_argument(&self) -> Vec<Vec<u32>> {
        let token_stream = self.token_stream_mut();
        let mut tape_offsets: Vec<Vec<u32>> = Vec::new();
        token_stream.read::<u8>();
        let mut depth = 1i32;
        let mut temp_set: Vec<u32> = Vec::new();

        loop {
            let offset = token_stream.get_read_offset() as u32;
            let t = token_type_from(token_stream.read::<u8>());

            if t == IfcTokenType::SetBegin {
                temp_set = Vec::new();
                depth += 1;
            } else if t == IfcTokenType::SetEnd {
                if !temp_set.is_empty() {
                    tape_offsets.push(temp_set);
                    temp_set = Vec::new();
                }
                depth -= 1;
            } else {
                temp_set.push(offset);
                match t {
                    IfcTokenType::Ref => {
                        token_stream.read::<u32>();
                    }
                    IfcTokenType::String
                    | IfcTokenType::Integer
                    | IfcTokenType::Real
                    | IfcTokenType::Label
                    | IfcTokenType::Enum => {
                        let length = token_stream.read::<u16>() as usize;
                        token_stream.forward(length);
                    }
                    _ => {
                        let expr = self.get_current_line_express_id().to_string();
                        Log::error("[GetSetListArgument()] unexpected token", &[expr.as_str()]);
                    }
                }
            }

            if depth == 0 {
                break;
            }
        }

        tape_offsets
    }

    pub fn move_to_argument_offset(&self, express_id: u32, argument_index: u32) {
        let Some(line) = self.lines.get(&express_id) else {
            return;
        };
        self.token_stream_mut().move_to(line.tape_offset as usize);
        self.argument_offset(argument_index);
    }

    pub fn get_no_line_arguments(&self, express_id: u32) -> u32 {
        let Some(line) = self.lines.get(&express_id) else {
            return 0;
        };
        let token_stream = self.token_stream_mut();
        token_stream.move_to(line.tape_offset as usize);
        token_stream.read::<u8>();
        token_stream.read::<u32>();
        token_stream.read::<u8>();
        let length = token_stream.read::<u16>() as usize;
        token_stream.forward(length);
        token_stream.read::<u8>();
        let mut no_arguments = 0u32;

        loop {
            let t = token_type_from(token_stream.read::<u8>());
            if t == IfcTokenType::SetEnd || t == IfcTokenType::LineEnd {
                return no_arguments;
            }
            if t == IfcTokenType::Unknown || t == IfcTokenType::Empty {
                no_arguments += 1;
                continue;
            }
            if t == IfcTokenType::SetBegin {
                self.step_back();
                self.get_set_argument();
                no_arguments += 1;
                continue;
            }
            if matches!(
                t,
                IfcTokenType::String
                    | IfcTokenType::Integer
                    | IfcTokenType::Real
                    | IfcTokenType::Label
                    | IfcTokenType::Enum
            ) {
                let length = token_stream.read::<u16>() as usize;
                token_stream.forward(length);
                no_arguments += 1;
                if t == IfcTokenType::Label {
                    self.get_set_argument();
                }
                continue;
            }
            if t == IfcTokenType::Ref {
                token_stream.read::<u32>();
                no_arguments += 1;
                continue;
            }
        }
    }

    pub fn step_back(&self) {
        self.token_stream_mut().back();
    }

    pub fn get_schema(&self) -> IFC_SCHEMA {
        let header_lines = self.get_header_lines_with_type(ifc_schema::FILE_SCHEMA);
        let Some(line) = header_lines.first().copied() else {
            return IFC_SCHEMA::IFC2X3;
        };
        self.move_to_header_line_argument(line, 0);
        let schemas = self.schema_manager.get_available_schemas();
        let token_stream = self.token_stream_mut();
        while !token_stream.is_at_end() {
            let t = token_type_from(token_stream.read::<u8>());
            if t == IfcTokenType::LineEnd {
                break;
            }
            if t == IfcTokenType::Label {
                let schema_name = token_stream.read_string();
                for schema in &schemas {
                    if self.schema_manager.get_schema_name(*schema) == schema_name {
                        return *schema;
                    }
                }
            }
        }
        IFC_SCHEMA::IFC2X3
    }

    pub fn push_bytes(&mut self, data: &[u8]) {
        self.token_stream_mut().push_bytes(data);
    }

    // C++ mapping: Push(void*, size).
    pub fn push_raw(&mut self, data: &[u8]) {
        self.push_bytes(data);
    }

    // C++ template mapping: Push(T input).
    pub fn push<T: Copy>(&mut self, input: T) {
        self.token_stream_mut().push(input);
    }

    pub fn get_total_size(&self) -> u64 {
        self.token_stream_ref().get_total_size() as u64
    }

    pub fn update_line_tape(&mut self, express_id: u32, type_code: u32, start: u32) {
        if let Some(line) = self.lines.get_mut(&express_id) {
            line.tape_offset = start;
            return;
        }

        let line = IfcLine {
            ifc_type: type_code,
            tape_offset: start,
        };
        self.lines.insert(express_id, line);
        self.ifc_type_to_express_id
            .entry(type_code)
            .or_default()
            .push(express_id);
        self.max_express_id = self.max_express_id.max(express_id);
    }

    pub fn add_header_line_tape(&mut self, type_code: u32, start: u32) {
        self.header_lines.push(IfcLine {
            ifc_type: type_code,
            tape_offset: start,
        });
    }

    pub fn get_current_line_express_id(&self) -> u32 {
        if self.lines.is_empty() {
            return 0;
        }
        let pos = self.token_stream_ref().get_read_offset() as u32;
        let mut prev_line = 0u32;
        let mut prev_offset = 0u32;
        for (key, value) in &self.lines {
            if value.tape_offset <= pos && value.tape_offset >= prev_offset {
                prev_offset = value.tape_offset;
                prev_line = *key;
            }
        }
        prev_line
    }

    pub fn remove_line(&mut self, express_id: u32) {
        self.lines.remove(&express_id);
    }

    pub fn push_double(&mut self, input: f64) {
        let mut number_string = format!("{input}");
        if let Some(pos) = number_string.find('e') {
            number_string.replace_range(pos..=pos, "E");
        } else if input.fract() == 0.0 {
            number_string.push('.');
        }
        let length = number_string.len() as u16;
        let token_stream = self.token_stream_mut();
        token_stream.push::<u16>(length);
        token_stream.push_bytes(number_string.as_bytes());
    }

    pub fn push_int(&mut self, input: i32) {
        let number_string = input.to_string();
        let length = number_string.len() as u16;
        let token_stream = self.token_stream_mut();
        token_stream.push::<u16>(length);
        token_stream.push_bytes(number_string.as_bytes());
    }

    pub fn generate_uuid(&self) -> String {
        compress_ifc_guid(&generate_string_uuid())
    }

    pub fn clone_loader(&self) -> Self {
        Self {
            max_express_id: self.max_express_id,
            line_writer_buffer: self.line_writer_buffer,
            schema_manager: Arc::clone(&self.schema_manager),
            token_stream: UnsafeCell::new(self.token_stream_ref().clone_stream()),
            lines: self.lines.clone(),
            header_lines: self.header_lines.clone(),
            ifc_type_to_express_id: self.ifc_type_to_express_id.clone(),
        }
    }

    pub fn get_next_express_id(&self, express_id: u32) -> u32 {
        let mut current_id = express_id + 1;
        while !self.lines.contains_key(&current_id) {
            current_id += 1;
        }
        current_id
    }

    fn parse_lines(&mut self) {
        let mut current_ifc_type = 0u32;
        let mut current_express_id = 0u32;
        let mut current_tape_offset = 0u32;
        let token_stream = self.token_stream_mut();

        while !token_stream.is_at_end() {
            let t = token_type_from(token_stream.read::<u8>());
            match t {
                IfcTokenType::LineEnd => {
                    if current_ifc_type != 0 {
                        let line = IfcLine {
                            ifc_type: current_ifc_type,
                            tape_offset: current_tape_offset,
                        };
                        if current_ifc_type == ifc_schema::FILE_DESCRIPTION
                            || current_ifc_type == ifc_schema::FILE_NAME
                            || current_ifc_type == ifc_schema::FILE_SCHEMA
                        {
                            self.header_lines.push(line);
                        } else if current_express_id != 0 {
                            self.ifc_type_to_express_id
                                .entry(current_ifc_type)
                                .or_default()
                                .push(current_express_id);
                            self.max_express_id = self.max_express_id.max(current_express_id);
                            self.lines.insert(current_express_id, line);
                            current_express_id = 0;
                        }
                        current_ifc_type = 0;
                    }
                    current_tape_offset = token_stream.get_read_offset() as u32;
                }
                IfcTokenType::Unknown
                | IfcTokenType::Empty
                | IfcTokenType::SetBegin
                | IfcTokenType::SetEnd => {}
                IfcTokenType::String
                | IfcTokenType::Real
                | IfcTokenType::Integer
                | IfcTokenType::Enum => {
                    let size = token_stream.read::<u16>() as usize;
                    token_stream.forward(size);
                }
                IfcTokenType::Label => {
                    let s = token_stream.read_string();
                    if current_ifc_type == 0 {
                        current_ifc_type = self.schema_manager.ifc_type_to_type_code(s);
                    }
                }
                IfcTokenType::Ref => {
                    let reference = token_stream.read::<u32>();
                    if current_express_id == 0 {
                        current_express_id = reference;
                    }
                }
            }
        }
    }

    fn argument_offset(&self, argument_index: u32) {
        let token_stream = self.token_stream_mut();
        let mut moved_over = 0u32;
        let mut set_depth = 0u32;
        loop {
            if set_depth == 1 {
                moved_over += 1;
                if moved_over - 1 == argument_index {
                    return;
                }
            }

            let t = token_type_from(token_stream.read::<u8>());
            match t {
                IfcTokenType::LineEnd => {
                    let expr = self.get_current_line_express_id().to_string();
                    Log::error("[ArgumentOffset()] unexpected line end", &[expr.as_str()]);
                    break;
                }
                IfcTokenType::Unknown | IfcTokenType::Empty => {}
                IfcTokenType::SetBegin => {
                    set_depth += 1;
                }
                IfcTokenType::SetEnd => {
                    if set_depth > 0 {
                        set_depth -= 1;
                    }
                    if set_depth == 0 {
                        return;
                    }
                }
                IfcTokenType::String
                | IfcTokenType::Enum
                | IfcTokenType::Label
                | IfcTokenType::Integer
                | IfcTokenType::Real => {
                    let length = token_stream.read::<u16>() as usize;
                    token_stream.forward(length);
                }
                IfcTokenType::Ref => {
                    token_stream.read::<u32>();
                }
            }
        }
    }

    fn token_stream_mut(&self) -> &mut IfcTokenStream {
        // SAFETY: IfcLoader is !Sync and token_stream is only accessed through &self methods
        // that preserve single-threaded access. We rely on external callers to avoid aliasing
        // mutable access across concurrent threads; this matches the C++ loader semantics.
        unsafe { &mut *self.token_stream.get() }
    }

    fn token_stream_ref(&self) -> &IfcTokenStream {
        // SAFETY: Shared access is read-only and must not overlap with mutable access in
        // concurrent threads. The loader is not Sync, so callers cannot race this reference.
        unsafe { &*self.token_stream.get() }
    }
}

fn token_type_from(value: u8) -> IfcTokenType {
    match value {
        x if x == IfcTokenType::Unknown as u8 => IfcTokenType::Unknown,
        x if x == IfcTokenType::String as u8 => IfcTokenType::String,
        x if x == IfcTokenType::Label as u8 => IfcTokenType::Label,
        x if x == IfcTokenType::Enum as u8 => IfcTokenType::Enum,
        x if x == IfcTokenType::Real as u8 => IfcTokenType::Real,
        x if x == IfcTokenType::Ref as u8 => IfcTokenType::Ref,
        x if x == IfcTokenType::Empty as u8 => IfcTokenType::Empty,
        x if x == IfcTokenType::SetBegin as u8 => IfcTokenType::SetBegin,
        x if x == IfcTokenType::SetEnd as u8 => IfcTokenType::SetEnd,
        x if x == IfcTokenType::LineEnd as u8 => IfcTokenType::LineEnd,
        x if x == IfcTokenType::Integer as u8 => IfcTokenType::Integer,
        _ => IfcTokenType::Unknown,
    }
}
