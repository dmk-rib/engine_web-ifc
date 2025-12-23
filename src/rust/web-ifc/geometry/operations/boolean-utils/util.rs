//! Rust port of `web-ifc/geometry/operations/boolean-utils/util.h`.

#[cfg(windows)]
pub fn write_file(filename: &str, data: &str) {
    use std::fs;
    use std::path::PathBuf;

    let mut path = PathBuf::from("debug_output");
    let _ = fs::create_dir_all(&path);
    path.push(filename);
    if let Ok(mut file) = fs::File::create(path) {
        use std::io::Write;
        let _ = file.write_all(data.as_bytes());
    }
}

#[cfg(not(windows))]
pub fn write_file(_filename: &str, _data: &str) {}
