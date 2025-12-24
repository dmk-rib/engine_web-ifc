//! Token stream for IFC parsing.

use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use super::ifc_file_stream::{DataSource, IfcFileStream, SharedIfcFileStream};
use super::ifc_token_chunk::IfcTokenChunk;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i8)]
pub enum IfcTokenType {
    Unknown = 0,
    String,
    Label,
    Enum,
    Real,
    Ref,
    Empty,
    SetBegin,
    SetEnd,
    LineEnd,
    Integer,
}

#[derive(Debug, Clone)]
pub struct IfcTokenStream {
    read_ptr: usize,
    current_chunk: usize,
    active_chunks: usize,
    chunk_size: usize,
    max_chunks: u64,
    chunks: Vec<IfcTokenChunk>,
    current_chunk_ref: Option<usize>,
    file_stream: Option<SharedIfcFileStream>,
}

impl IfcTokenStream {
    pub fn new(chunk_size: usize, max_chunks: u64) -> Self {
        Self {
            read_ptr: 0,
            current_chunk: 0,
            active_chunks: 0,
            chunk_size,
            max_chunks,
            chunks: Vec::new(),
            current_chunk_ref: None,
            file_stream: None,
        }
    }

    pub fn set_token_source(&mut self, request_data: DataSource) {
        let file_stream = Rc::new(RefCell::new(IfcFileStream::new(
            request_data,
            self.chunk_size as u32,
        )));
        self.file_stream = Some(file_stream.clone());
        let mut token_offset = 0usize;
        loop {
            if file_stream.borrow().is_at_end() {
                break;
            }
            self.check_memory();
            let chunk = IfcTokenChunk::new(
                self.chunk_size,
                token_offset,
                file_stream.borrow().get_ref(),
                Some(file_stream.clone()),
            );
            let c_size = chunk.token_size();
            token_offset += c_size;
            if c_size > self.chunk_size {
                self.chunk_size = c_size;
            }
            self.chunks.push(chunk);
            self.active_chunks += 1;
        }
        self.current_chunk = 0;
        self.current_chunk_ref = if self.chunks.is_empty() {
            None
        } else {
            Some(0)
        };
        if let Some(fs) = self.file_stream.as_ref() {
            fs.borrow_mut().clear();
        }
    }

    pub fn set_token_source_reader<R: Read>(&mut self, mut reader: R) {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).ok();
        let source: DataSource = Rc::new(
            move |dest: &mut [u8], source_offset: usize, dest_size: usize| {
                if source_offset >= buffer.len() {
                    return 0;
                }
                let end = (source_offset + dest_size).min(buffer.len());
                let size = end - source_offset;
                dest[..size].copy_from_slice(&buffer[source_offset..end]);
                size as u32
            },
        );
        self.set_token_source(source);
    }

    pub fn read<T: Copy>(&mut self) -> T {
        if let Some(idx) = self.current_chunk_ref {
            let chunk = &mut self.chunks[idx];
            if !chunk.is_loaded() {
                self.check_memory();
                self.active_chunks += 1;
            }
            let v = chunk.read::<T>(self.read_ptr);
            self.forward(std::mem::size_of::<T>());
            v
        } else {
            panic!("no chunk loaded");
        }
    }

    pub fn push<T: Copy>(&mut self, input: T) {
        // Safety: we only read the bytes of a Copy POD value for serialization;
        // the slice is valid for size_of::<T>() and does not outlive `input`.
        self.push_bytes(unsafe {
            std::slice::from_raw_parts((&input as *const T) as *const u8, std::mem::size_of::<T>())
        });
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        if self.chunks.is_empty() {
            self.chunks.push(IfcTokenChunk::new(
                self.chunk_size,
                0,
                0,
                self.file_stream.clone(),
            ));
            self.active_chunks += 1;
        }
        let last_size = self.chunks.last().unwrap().token_size();
        let max_size = self.chunks.last().unwrap().get_max_size();
        if last_size + bytes.len() > max_size {
            self.check_memory();
            let fs_ref = self
                .file_stream
                .as_ref()
                .map(|fs| fs.borrow().get_ref())
                .unwrap_or(0);
            let start_ref = self.chunks.last().unwrap().get_token_ref() + last_size;
            self.chunks.push(IfcTokenChunk::new(
                self.chunk_size,
                start_ref,
                fs_ref,
                self.file_stream.clone(),
            ));
            self.active_chunks += 1;
        }
        self.chunks.last_mut().unwrap().push_bytes(bytes);
    }

    // C++ mapping: Push(void*, size).
    pub fn push_raw(&mut self, bytes: &[u8]) {
        self.push_bytes(bytes);
    }

    pub fn read_string(&mut self) -> &str {
        if let Some(idx) = self.current_chunk_ref {
            let chunk = &mut self.chunks[idx];
            if !chunk.is_loaded() {
                self.check_memory();
                self.active_chunks += 1;
            }
            let length = chunk.read::<u16>(self.read_ptr) as usize;
            self.forward(2);
            if length > 0 {
                let str_view = chunk.read_string(self.read_ptr, length);
                self.forward(length);
                str_view
            } else {
                ""
            }
        } else {
            ""
        }
    }

    pub fn forward(&mut self, size: usize) {
        self.read_ptr += size;
        loop {
            let Some(idx) = self.current_chunk_ref else {
                break;
            };
            let token_size = self.chunks[idx].token_size();
            if self.read_ptr < token_size {
                break;
            }
            if self.current_chunk >= self.chunks.len() - 1 {
                self.read_ptr = self.chunks.last().map(|c| c.token_size()).unwrap_or(0);
                break;
            }
            self.read_ptr -= token_size;
            self.current_chunk += 1;
            self.current_chunk_ref = Some(self.current_chunk);
        }
    }

    pub fn move_to(&mut self, pos: usize) {
        if self.chunks.is_empty() {
            return;
        }
        for i in (0..self.chunks.len()).rev() {
            if self.chunks[i].get_token_ref() <= pos {
                self.current_chunk = i;
                self.current_chunk_ref = Some(i);
                self.read_ptr = pos - self.chunks[i].get_token_ref();
                break;
            }
        }
    }

    pub fn back(&mut self) {
        if self.read_ptr == 0 {
            if self.current_chunk > 0 {
                self.current_chunk -= 1;
                self.current_chunk_ref = Some(self.current_chunk);
                self.read_ptr = self.chunks[self.current_chunk].token_size() - 1;
                return;
            }
        }
        self.read_ptr = self.read_ptr.saturating_sub(1);
    }

    pub fn is_at_end(&self) -> bool {
        if self.chunks.is_empty() {
            return true;
        }
        self.current_chunk >= self.chunks.len() - 1
            && self.read_ptr >= self.chunks.last().unwrap().token_size()
    }

    pub fn get_read_offset(&self) -> usize {
        if let Some(idx) = self.current_chunk_ref {
            self.chunks[idx].get_token_ref() + self.read_ptr
        } else {
            0
        }
    }

    pub fn get_total_size(&self) -> usize {
        if self.chunks.is_empty() {
            0
        } else {
            self.chunks.last().unwrap().token_size() + self.chunks.last().unwrap().get_token_ref()
        }
    }

    pub fn clone_stream(&self) -> Self {
        Self {
            read_ptr: self.read_ptr,
            current_chunk: self.current_chunk,
            active_chunks: self.active_chunks,
            chunk_size: self.chunk_size,
            max_chunks: self.max_chunks,
            chunks: self.chunks.clone(),
            current_chunk_ref: self.current_chunk_ref,
            file_stream: self
                .file_stream
                .as_ref()
                .map(|fs| Rc::new(RefCell::new(fs.borrow().clone_stream()))),
        }
    }

    fn check_memory(&mut self) {
        if self.max_chunks != 0 && self.active_chunks as u64 == self.max_chunks {
            for chunk in &mut self.chunks {
                if chunk.is_loaded() {
                    if chunk.clear_without_force() {
                        self.active_chunks = self.active_chunks.saturating_sub(1);
                        break;
                    }
                }
            }
        }
    }
}
