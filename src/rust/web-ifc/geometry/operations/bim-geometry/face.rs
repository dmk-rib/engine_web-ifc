//! Triangle face indices.

#[derive(Copy, Clone, Debug, Default)]
pub struct Face {
    pub i0: u32,
    pub i1: u32,
    pub i2: u32,
    pub p_id: u32,
}
