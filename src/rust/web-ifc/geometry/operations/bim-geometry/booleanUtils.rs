//! Boolean utilities bridging to fuzzy bools.

use super::geometry::Geometry;

/// Convert and execute boolean operations.
///
/// NOTE: The full fuzzy-bools port is pending. This implementation preserves
/// API shape and provides a deterministic fallback.
pub fn bool_process(mut first: Geometry, second_geoms: &mut [Geometry], op: &str) -> Geometry {
    for second in second_geoms.iter() {
        let mut doit = true;
        if second.num_faces == 0 {
            doit = false;
        }
        if first.num_faces == 0 && op != "UNION" {
            break;
        }
        if doit {
            match op {
                "UNION" => {
                    first.add_geometry(second.clone());
                }
                "DIFFERENCE" => {
                    // TODO: implement fuzzy bools subtraction.
                }
                _ => {}
            }
        }
    }

    let mut final_result = Geometry::default();
    final_result.add_geometry(first);
    final_result
}
