#![allow(dead_code, unused_imports)]

#[cfg(feature = "web_ifc")]
#[path = "web-ifc/mod.rs"]
pub mod web_ifc;

#[path = "glam.rs"]
pub mod glam;

#[path = "earcutr.rs"]
pub mod earcutr;

#[path = "uuid.rs"]
pub mod uuid;

#[path = "api/mod.rs"]
pub mod api;

#[path = "test/mod.rs"]
pub mod test;

#[path = "wasm/mod.rs"]
pub mod wasm;

#[path = "version.rs"]
pub mod version;

#[path = "CMakeLists.rs"]
pub mod cmake_lists;
