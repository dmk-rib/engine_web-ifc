//! Rust port of `test/main.cpp`.

#![allow(dead_code)]

use crate::test::encoding_test;

/// Run the test suite and return an exit code.
pub fn run(argv: &[String]) -> i32 {
    let _ = argv;
    if encoding_test::run_tests() {
        0
    } else {
        1
    }
}
