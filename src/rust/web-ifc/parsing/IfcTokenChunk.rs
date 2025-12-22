//! Rust port of IfcTokenChunk public API.

use std::sync::Arc;

use super::ifc_file_stream::IfcFileStream;

#[derive(Debug)]
pub struct IfcTokenChunk {
    chunk_size: usize,
    start_ref: usize,
    file_start_ref: usize,
    loaded: bool,
    current_size: usize,
    file_stream: Option<Arc<IfcFileStream>>,
}

impl IfcTokenChunk {
    pub fn new(chunk_size: usize, start_ref: usize, file_start_ref: usize, file_stream: Arc<IfcFileStream>) -> Self {
        Self {
            chunk_size,
            start_ref,
            file_start_ref,
            loaded: false,
            current_size: 0,
            file_stream: Some(file_stream),
        }
    }

    pub fn clear(&mut self, _force: bool) -> bool {
        unimplemented!("clear not yet implemented");
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn token_size(&self) -> usize {
        self.current_size
    }

    pub fn get_token_ref(&self) -> usize {
        self.start_ref
    }

    pub fn push_bytes(&mut self, _bytes: &[u8]) {
        unimplemented!("push_bytes not yet implemented");
    }

    pub fn get_max_size(&self) -> usize {
        self.chunk_size
    }

    pub fn read_string(&self, _ptr: usize, _size: usize) -> &str {
        unimplemented!("read_string not yet implemented");
    }

    pub fn read<T: Copy>(&mut self, _ptr: usize) -> T {
        unimplemented!("read not yet implemented");
    }

    pub fn push<T: Copy>(&mut self, _input: T) {
        unimplemented!("push not yet implemented");
    }
}
