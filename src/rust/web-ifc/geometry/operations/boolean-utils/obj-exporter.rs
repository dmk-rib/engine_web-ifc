//! Rust port of `web-ifc/geometry/operations/boolean-utils/obj-exporter.h`.

use glam::DMat4;

use super::geometry::Geometry;
use super::util::write_file;

pub fn to_obj(geom: &Geometry, offset: &mut usize, transform: DMat4, input_scale: f64) -> String {
    let mut obj = String::new();

    let scale = input_scale;
    for i in 0..geom.num_points as usize {
        let t = transform * geom.get_point(i).extend(1.0);
        obj.push_str(&format!(
            "v {} {} {}\n",
            t.x * scale,
            t.y * scale,
            t.z * scale
        ));
    }

    for i in 0..geom.num_faces as usize {
        let f = geom.get_face(i);
        obj.push_str(&format!(
            "f {}// {}// {}//\n",
            f.i0 as usize + 1 + *offset,
            f.i1 as usize + 1 + *offset,
            f.i2 as usize + 1 + *offset
        ));
    }

    *offset += geom.num_points as usize;
    obj
}

pub fn dump_geometry(geom: &Geometry, filename: &str) {
    let mut offset = 0;
    write_file(filename, &to_obj(geom, &mut offset, DMat4::IDENTITY, 1.0));
}
