pub mod aliases;
pub mod crc;
pub mod generator;
pub mod model;
pub mod parser;

use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct GenerationResult {
    pub output_path: std::path::PathBuf,
    pub contents: String,
}

pub fn generate(schema_dir: &Path, out_dir: &Path) -> Result<GenerationResult> {
    let schemas = parser::load_schemas(schema_dir)?;
    let contents = generator::generate_rust(&schemas)?;
    std::fs::create_dir_all(out_dir)?;
    let output_path = out_dir.join("ifc_schema.rs");
    std::fs::write(&output_path, &contents)?;
    Ok(GenerationResult {
        output_path,
        contents,
    })
}

pub fn generate_to_string(schema_dir: &Path) -> Result<String> {
    let schemas = parser::load_schemas(schema_dir)?;
    generator::generate_rust(&schemas)
}
