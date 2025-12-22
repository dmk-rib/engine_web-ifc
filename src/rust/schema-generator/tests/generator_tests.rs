use std::path::PathBuf;

use schema_generator::generate_to_string;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[test]
fn generates_expected_output() {
    let output = generate_to_string(&fixture_dir()).expect("generate schema");
    let golden_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/ifc_schema.rs");
    let expected = std::fs::read_to_string(golden_path).expect("read golden file");
    assert_eq!(output, expected);
}

#[test]
fn generation_is_deterministic() {
    let first = generate_to_string(&fixture_dir()).expect("generate schema");
    let second = generate_to_string(&fixture_dir()).expect("generate schema");
    assert_eq!(first, second);
}
