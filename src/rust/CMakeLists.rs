//! Rust mirror of the CMake configuration used by the C++ build.

#![allow(dead_code)]

/// The CMakeLists.txt content for the native C++ build.
pub const CMAKE_LISTS: &str = include_str!("../cpp/CMakeLists.txt");
