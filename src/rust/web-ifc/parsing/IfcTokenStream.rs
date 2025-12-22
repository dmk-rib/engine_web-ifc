//! Rust port of IfcTokenStream public API.

use std::io::Read;
use std::sync::Arc;

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

/// Token stream over IFC data.
#[derive(Debug)]
pub struct IfcTokenStream {
    chunk_size: usize,
    max_chunks: u64,
    read_ptr: usize,
    total_size: usize,
}

impl IfcTokenStream {
    pub fn new(chunk_size: usize, max_chunks: u64) -> Self {
        Self {
            chunk_size,
            max_chunks,
            read_ptr: 0,
            total_size: 0,
        }
    }

    pub fn set_token_source(&mut self, _source: Arc<dyn Fn(&mut [u8], usize, usize) -> u32 + Send + Sync>) {
        unimplemented!("set_token_source callback-based path is pending Rust port");
    }

    pub fn set_token_source_reader<R: Read>(&mut self, _reader: R) {
        unimplemented!("set_token_source reader-based path is pending Rust port");
    }

    #[inline]
    pub fn read<T: Copy>(&mut self) -> T {
        unimplemented!("read token not yet implemented");
    }

    #[inline]
    pub fn push<T: Copy>(&mut self, _input: T) {
        unimplemented!("push token not yet implemented");
    }

    pub fn push_bytes(&mut self, _bytes: &[u8]) {
        unimplemented!("push bytes not yet implemented");
    }

    pub fn forward(&mut self, _size: usize) {
        unimplemented!("forward not yet implemented");
    }

    pub fn read_string(&mut self) -> &str {
        unimplemented!("read_string not yet implemented");
    }

    pub fn back(&mut self) {
        unimplemented!("back not yet implemented");
    }

    pub fn is_at_end(&self) -> bool {
        self.read_ptr >= self.total_size
    }

    pub fn move_to(&mut self, _pos: usize) {
        unimplemented!("move_to not yet implemented");
    }

    pub fn get_read_offset(&self) -> usize {
        self.read_ptr
    }

    pub fn get_total_size(&self) -> usize {
        self.total_size
    }

    pub fn clone_stream(&self) -> Self {
        Self {
            chunk_size: self.chunk_size,
            max_chunks: self.max_chunks,
            read_ptr: self.read_ptr,
            total_size: self.total_size,
        }
    }
}
