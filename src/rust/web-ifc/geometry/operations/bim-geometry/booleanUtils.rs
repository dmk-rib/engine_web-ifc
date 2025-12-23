//! Boolean utilities bridging to fuzzy bools.

use crate::web_ifc::geometry::operations::boolean_utils::{fuzzy_bools, geometry as bool_geometry};

use super::geometry::Geometry;
use super::plane::Plane;

fn convert_to_engine(geom: Geometry) -> bool_geometry::Geometry {
    let mut new_geom = bool_geometry::Geometry::default();
    new_geom.fvertex_data = geom.fvertex_data;
    new_geom.vertex_data = geom.vertex_data;
    new_geom.index_data = geom.index_data;
    new_geom.plane_data = geom.plane_data;
    new_geom.num_points = geom.num_points;
    new_geom.num_faces = geom.num_faces;
    for plane in geom.planes {
        new_geom.planes.push(bool_geometry::SimplePlane {
            distance: plane.distance,
            normal: plane.normal,
        });
    }
    new_geom.has_planes = geom.has_planes;
    new_geom
}

fn convert_to_bim(geom: bool_geometry::Geometry) -> Geometry {
    let mut new_geom = Geometry::default();
    new_geom.fvertex_data = geom.fvertex_data;
    new_geom.vertex_data = geom.vertex_data;
    new_geom.index_data = geom.index_data;
    new_geom.plane_data = geom.plane_data;
    new_geom.num_points = geom.num_points;
    new_geom.num_faces = geom.num_faces;
    for (id, plane) in geom.planes.into_iter().enumerate() {
        new_geom.planes.push(Plane {
            id,
            distance: plane.distance,
            normal: plane.normal,
        });
    }
    new_geom.has_planes = geom.has_planes;
    new_geom
}

fn union(first: Geometry, second: Geometry) -> Geometry {
    let first_engine = convert_to_engine(first);
    let second_engine = convert_to_engine(second);
    convert_to_bim(fuzzy_bools::union(&first_engine, &second_engine))
}

fn subtract(first: Geometry, second: Geometry) -> Geometry {
    let first_engine = convert_to_engine(first);
    let second_engine = convert_to_engine(second);
    convert_to_bim(fuzzy_bools::subtract(&first_engine, &second_engine))
}

/// Convert and execute boolean operations.
pub fn bool_process(mut first: Geometry, second_geoms: &[Geometry], op: &str) -> Geometry {
    for second in second_geoms.iter() {
        let mut doit = true;
        if second.num_faces == 0 {
            doit = false;
        }
        if first.num_faces == 0 && op != "UNION" {
            break;
        }
        if doit {
            let mut second_operator = second.clone();
            first.build_planes();
            second_operator.build_planes();

            if op == "DIFFERENCE" {
                first = subtract(first, second_operator);
            } else if op == "UNION" {
                first = union(first, second_operator);
            }
        }
    }

    let mut final_result = Geometry::default();
    final_result.add_geometry(first);
    final_result
}
