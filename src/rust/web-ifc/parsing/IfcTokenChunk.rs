//! Token chunk for IFC parsing.

use std::cell::RefCell;
use std::mem::{size_of, MaybeUninit};
use std::rc::Rc;

use super::ifc_file_stream::SharedIfcFileStream;
use super::ifc_token_stream::IfcTokenType;

#[derive(Debug, Clone)]
pub struct IfcTokenChunk {
    start_ref: usize,
    file_start_ref: usize,
    chunk_size: usize,
    loaded: bool,
    current_size: usize,
    chunk_data: Option<Vec<u8>>,
    file_stream: Option<SharedIfcFileStream>,
}

impl IfcTokenChunk {
    pub fn new(
        chunk_size: usize,
        start_ref: usize,
        file_start_ref: usize,
        file_stream: Option<SharedIfcFileStream>,
    ) -> Self {
        let mut chunk = Self {
            start_ref,
            file_start_ref,
            chunk_size,
            loaded: true,
            current_size: 0,
            chunk_data: None,
            file_stream,
        };
        if chunk.file_stream.is_some() {
            chunk.load();
        }
        chunk
    }

    pub fn clear(&mut self, force: bool) -> bool {
        if self.file_stream.is_none() && !force {
            return false;
        }
        self.chunk_data = None;
        self.loaded = false;
        true
    }

    pub fn clear_without_force(&mut self) -> bool {
        self.clear(false)
    }

    pub fn get_token_ref(&self) -> usize {
        self.start_ref
    }

    pub fn token_size(&self) -> usize {
        self.current_size
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_max_size(&self) -> usize {
        self.chunk_size
    }

    pub fn read_string(&mut self, ptr: usize, size: usize) -> &str {
        if !self.loaded {
            self.load();
        }
        let data = self.chunk_data.as_ref().unwrap();
        let slice = &data[ptr..ptr + size];
        unsafe {
            // SAFETY: IFC strings are ASCII/UTF-8 byte sequences. We preserve raw bytes.
            std::str::from_utf8_unchecked(slice)
        }
    }

    pub fn read<T: Copy>(&mut self, ptr: usize) -> T {
        if !self.loaded {
            self.load();
        }
        let data = self.chunk_data.as_ref().unwrap();
        let slice = &data[ptr..ptr + size_of::<T>()];
        unsafe {
            // SAFETY: We read a plain-old-data value from an unaligned byte slice.
            // The slice length is guaranteed by caller to match T size.
            let mut value = MaybeUninit::<T>::uninit();
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                value.as_mut_ptr() as *mut u8,
                size_of::<T>(),
            );
            value.assume_init()
        }
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        if self.chunk_data.is_none() {
            self.chunk_data = Some(Vec::with_capacity(self.chunk_size));
        }
        let data = self.chunk_data.as_mut().unwrap();
        data.extend_from_slice(bytes);
        self.current_size += bytes.len();
        if self.current_size > self.chunk_size {
            self.chunk_size = self.current_size;
        }
    }

    pub fn push<T: Copy>(&mut self, input: T) {
        let bytes = unsafe {
            // SAFETY: We are serializing a POD value into bytes.
            std::slice::from_raw_parts((&input as *const T) as *const u8, size_of::<T>())
        };
        self.push_bytes(bytes);
    }

    fn load(&mut self) {
        let Some(file_stream) = self.file_stream.as_ref() else {
            return;
        };
        let mut file_stream = file_stream.borrow_mut();
        if file_stream.get_ref() != self.file_start_ref {
            file_stream.go(self.file_start_ref as u32);
        }

        self.chunk_data = Some(vec![0u8; 0]);
        self.loaded = true;
        self.current_size = 0;
        let mut temp: Vec<u8> = Vec::with_capacity(50);

        while !file_stream.is_at_end() && self.current_size < self.chunk_size {
            let c = file_stream.get();
            if c == b' ' || c == b'\n' || c == b'\r' || c == b'\t' {
                file_stream.forward();
                continue;
            }

            if c == b'\'' {
                file_stream.forward();
                temp.clear();
                loop {
                    temp.push(file_stream.get());
                    if file_stream.get() == b'\'' {
                        file_stream.forward();
                        if file_stream.get() == b'\'' {
                            temp.push(file_stream.get());
                        } else {
                            file_stream.back();
                            temp.pop();
                            break;
                        }
                    }
                    file_stream.forward();
                }
                self.push(IfcTokenType::String as u8);
                self.push::<u16>(temp.len() as u16);
                if !temp.is_empty() {
                    self.push_bytes(&temp);
                }
            } else if c == b'#' {
                file_stream.forward();
                let mut num: u32 = 0;
                let mut c2 = file_stream.get();
                while (b'0'..=b'9').contains(&c2) {
                    num = num * 10 + (c2 - b'0') as u32;
                    file_stream.forward();
                    c2 = file_stream.get();
                }
                self.push(IfcTokenType::Ref as u8);
                self.push::<u32>(num);
                continue;
            } else if c == b'$' {
                self.push(IfcTokenType::Empty as u8);
            } else if c == b'*' {
                if file_stream.prev() == b'/' {
                    file_stream.forward();
                    while !(file_stream.prev() == b'*' && file_stream.get() == b'/') {
                        file_stream.forward();
                    }
                } else {
                    self.push(IfcTokenType::Unknown as u8);
                }
            } else if c == b'(' {
                self.push(IfcTokenType::SetBegin as u8);
            } else if (b'0'..=b'9').contains(&c) {
                temp.clear();
                if file_stream.prev() == b'-' {
                    temp.push(b'-');
                }
                let mut c2 = file_stream.get();
                let mut is_frac = false;
                while (b'0'..=b'9').contains(&c2)
                    || c2 == b'.'
                    || c2 == b'e'
                    || c2 == b'E'
                    || c2 == b'-'
                    || c2 == b'+'
                {
                    temp.push(c2);
                    if c2 == b'.' || c2 == b'E' {
                        is_frac = true;
                    }
                    file_stream.forward();
                    c2 = file_stream.get();
                }
                if is_frac {
                    self.push(IfcTokenType::Real as u8);
                } else {
                    self.push(IfcTokenType::Integer as u8);
                }
                self.push::<u16>(temp.len() as u16);
                self.push_bytes(&temp);
                continue;
            } else if c == b'.' {
                temp.clear();
                file_stream.forward();
                let mut c2 = file_stream.get();
                while c2 != b'.' {
                    temp.push(c2);
                    file_stream.forward();
                    c2 = file_stream.get();
                }
                self.push(IfcTokenType::Enum as u8);
                self.push::<u16>(temp.len() as u16);
                self.push_bytes(&temp);
            } else if (b'A'..=b'Z').contains(&c) || (b'a'..=b'z').contains(&c) {
                temp.clear();
                let mut c2 = file_stream.get();
                while (b'A'..=b'Z').contains(&c2)
                    || (b'a'..=b'z').contains(&c2)
                    || (b'0'..=b'9').contains(&c2)
                    || c2 == b'_'
                {
                    temp.push(c2);
                    file_stream.forward();
                    c2 = file_stream.get();
                }
                self.push(IfcTokenType::Label as u8);
                self.push::<u16>(temp.len() as u16);
                self.push_bytes(&temp);
                continue;
            } else if c == b')' {
                self.push(IfcTokenType::SetEnd as u8);
            } else if c == b';' {
                self.push(IfcTokenType::LineEnd as u8);
            }
            file_stream.forward();
        }
    }
}

pub type SharedIfcTokenChunk = Rc<RefCell<IfcTokenChunk>>;
