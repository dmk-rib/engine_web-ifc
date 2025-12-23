//! Rust port of `test/encoding_test.cpp`.

#![allow(dead_code)]

use crate::web_ifc::parsing::string_parsing::p21decode;

fn check_decode(input: &str, expected: &str) -> bool {
    match p21decode(input) {
        Ok(value) => value == expected,
        Err(_) => false,
    }
}

/// Runs the encoding tests and returns whether they succeeded.
pub fn run_tests() -> bool {
    check_decode("\\\\ ''", "\\ '")
        && check_decode("see \\X\\A7 4.1", "see § 4.1")
        && check_decode("\\X2\\03C0\\X0\\", "π")
        && check_decode("\\X2\\03B103B203B3\\X0\\", "αβγ")
        && check_decode(
            "\\X4\\0000041F0000044000000438000004320000043500000442000000200000041C0000043800000440\\X0\\",
            "Привет Мир",
        )
        && check_decode("\\S\\Drger", "Ärger")
        && check_decode("h\\S\\ttel", "hôtel")
        && check_decode("\\PE\\\\S\\*\\S\\U\\S\\b", "Њет")
}

#[cfg(test)]
mod tests {
    use super::run_tests;

    #[test]
    fn encoding_tests() {
        assert!(run_tests());
    }
}
