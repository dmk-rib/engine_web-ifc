#![allow(dead_code)]

/// C++ mapping: lightweight triangulation for simple polygon rings.
///
/// This implementation creates a triangle fan for the outer ring and ignores
/// holes. It preserves the public API surface while avoiding external
/// dependencies.
pub fn earcut(data: &[f64], hole_indices: Option<&[usize]>, dim: usize) -> Vec<usize> {
    if dim < 2 {
        return Vec::new();
    }
    let vertex_count = data.len() / dim;
    if vertex_count < 3 {
        return Vec::new();
    }

    let outer_count = hole_indices
        .and_then(|holes| holes.first().copied())
        .unwrap_or(vertex_count);
    if outer_count < 3 {
        return Vec::new();
    }

    let mut indices = Vec::with_capacity((outer_count - 2) * 3);
    for i in 1..outer_count - 1 {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }
    indices
}
