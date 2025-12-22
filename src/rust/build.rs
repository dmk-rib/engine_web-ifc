use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let ts_path = manifest_dir.join("../ts/ifc-schema.ts");
    let content = fs::read_to_string(&ts_path).expect("read ifc-schema.ts");

    let mut output = String::new();
    let mut seen_consts = HashSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("export const ") {
            continue;
        }
        let rest = trimmed.trim_start_matches("export const ");
        let Some((name, value_part)) = rest.split_once('=') else {
            continue;
        };
        let name = name.trim();
        let value_str = value_part.trim().trim_end_matches(';').trim();
        if name.is_empty() {
            continue;
        }
        if let Ok(value) = value_str.parse::<u64>() {
            if seen_consts.insert(name.to_string()) {
                output.push_str(&format!("pub const {name}: u32 = {value}u32;\n"));
            }
        }
    }

    let mut seen_types = HashSet::new();
    let mut rest = content.as_str();
    while let Some(idx) = rest.find("export class ") {
        rest = &rest[idx + "export class ".len()..];
        let mut name = String::new();
        for ch in rest.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
            } else {
                break;
            }
        }
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        if matches!(name, "Handle" | "NumberHandle" | "IfcLineObject") {
            continue;
        }
        if !seen_types.insert(name.to_string()) {
            rest = &rest[name.len()..];
            continue;
        }
        let after_name = rest[name.len()..].trim_start();
        let base = after_name.strip_prefix("extends").and_then(|after| {
            let after = after.trim_start();
            let base_name: String = after
                .chars()
                .take_while(|ch| ch.is_alphanumeric() || *ch == '_')
                .collect();
            if base_name.is_empty() {
                None
            } else {
                Some(base_name)
            }
        });
        let target = match base {
            Some(base_name) if base_name == "NumberHandle" => "NumberHandle".to_string(),
            Some(base_name) if base_name == "Handle" => "Handle<()>".to_string(),
            Some(base_name) => base_name,
            None => "IfcEntity".to_string(),
        };
        output.push_str(&format!("pub type {name} = {target};\n"));
        rest = &rest[name.len()..];
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("ifc_schema_generated.rs"), output).expect("write ifc schema");

    println!("cargo:rerun-if-changed={}", ts_path.display());
}
