//! File-backed stream for IFC tokenization.

use std::cell::RefCell;
use std::rc::Rc;

pub type DataSource = Rc<dyn Fn(&mut [u8], usize, usize) -> u32>;

#[derive(Debug, Clone)]
pub struct IfcFileStream {
    data_source: DataSource,
    size: usize,
    pointer: usize,
    current_size: usize,
    start_ref: usize,
    prev: u8,
    buffer: Option<Vec<u8>>,
}

impl IfcFileStream {
    pub fn new(source: DataSource, size: u32) -> Self {
        let mut stream = Self {
            data_source: source,
            size: size as usize,
            pointer: 0,
            current_size: 0,
            start_ref: 0,
            prev: 0,
            buffer: None,
        };
        stream.load();
        stream
    }

    pub fn go(&mut self, reference: u32) {
        self.start_ref = reference as usize;
        self.load();
    }

    pub fn forward(&mut self) {
        self.pointer += 1;
        if self.pointer == self.current_size && self.current_size != 0 {
            self.start_ref += self.current_size;
            self.load();
        }
    }

    pub fn back(&mut self) {
        if self.pointer == 0 {
            if self.start_ref > 0 {
                self.start_ref -= 1;
                self.load();
                self.pointer = 0;
            }
        } else {
            self.pointer -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.buffer = None;
        self.current_size = 0;
    }

    pub fn prev(&self) -> u8 {
        if self.pointer == 0 {
            self.prev
        } else {
            self.buffer
                .as_ref()
                .map(|b| b[self.pointer - 1])
                .unwrap_or(0)
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.pointer == self.current_size && self.current_size == 0
    }

    pub fn get_ref(&self) -> usize {
        self.start_ref + self.pointer
    }

    pub fn get(&self) -> u8 {
        self.buffer.as_ref().map(|b| b[self.pointer]).unwrap_or(0)
    }

    pub fn clone_stream(&self) -> Self {
        Self::new(self.data_source.clone(), self.size as u32)
    }

    fn load(&mut self) {
        if self.buffer.is_none() {
            self.buffer = Some(vec![0u8; self.size]);
        } else if self.current_size > 0 {
            if let Some(buffer) = self.buffer.as_ref() {
                self.prev = buffer[self.current_size - 1];
            }
        }

        if let Some(buffer) = self.buffer.as_mut() {
            self.current_size = (self.data_source)(buffer, self.start_ref, self.size) as usize;
        }
        self.pointer = 0;
    }
}

pub type SharedIfcFileStream = Rc<RefCell<IfcFileStream>>;
