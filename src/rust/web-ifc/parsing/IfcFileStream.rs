//! Rust port of IfcFileStream public API.

use std::sync::Arc;

#[derive(Debug)]
pub struct IfcFileStream {
    size: usize,
    pointer: usize,
    data_source: Option<Arc<dyn Fn(&mut [u8], usize, usize) -> u32 + Send + Sync>>,
}

impl IfcFileStream {
    pub fn new(source: Arc<dyn Fn(&mut [u8], usize, usize) -> u32 + Send + Sync>, size: u32) -> Self {
        Self {
            size: size as usize,
            pointer: 0,
            data_source: Some(source),
        }
    }

    pub fn go(&mut self, _reference: u32) {
        unimplemented!("go not yet implemented");
    }

    pub fn forward(&mut self) {
        unimplemented!("forward not yet implemented");
    }

    pub fn back(&mut self) {
        unimplemented!("back not yet implemented");
    }

    pub fn get_ref(&self) -> usize {
        self.pointer
    }

    pub fn next(&mut self) -> u8 {
        unimplemented!("next not yet implemented");
    }

    pub fn prev(&mut self) -> u8 {
        unimplemented!("prev not yet implemented");
    }

    pub fn is_at_end(&self) -> bool {
        self.pointer >= self.size
    }

    pub fn get(&self) -> u8 {
        unimplemented!("get not yet implemented");
    }

    pub fn clear(&mut self) {
        unimplemented!("clear not yet implemented");
    }

    pub fn clone_stream(&self) -> Self {
        Self {
            size: self.size,
            pointer: self.pointer,
            data_source: self.data_source.clone(),
        }
    }
}
