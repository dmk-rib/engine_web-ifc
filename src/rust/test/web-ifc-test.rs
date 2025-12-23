//! Rust port of `test/web-ifc-test.cpp`.

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use glam::{DMat4, DVec2};

use crate::test::io_helpers::{
    dump_alignment, dump_cross_sections, dump_flat_mesh, dump_mesh, Point,
};
use crate::web_ifc::geometry::ifc_geometry_processor::IfcGeometryProcessor;
use crate::web_ifc::geometry::operations::bim_geometry::utils::convert_2d_alignments_to_3d;
use crate::web_ifc::geometry::representation::geometry::{
    IfcAlignment, IfcCrossSections, IfcFlatMesh, SweptDiskSolid,
};
use crate::web_ifc::geometry::representation::ifc_curve::IfcCurve;
use crate::web_ifc::parsing::ifc_loader::IfcLoader;
use crate::web_ifc::parsing::ifc_token_stream::IfcTokenType;
use crate::web_ifc::schema::ifc_schema;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;

static SHOULD_WRITE_FILES: OnceLock<bool> = OnceLock::new();
static RNG_STATE: AtomicU64 = AtomicU64::new(0);

pub fn ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

pub fn should_write_files() -> bool {
    *SHOULD_WRITE_FILES.get_or_init(|| {
        let Ok(value) = env::var("WEB_IFC_TEST_WRITE_FILES") else {
            return false;
        };
        let value = value.trim();
        if value == "1" {
            return true;
        }
        let value = value.to_ascii_lowercase();
        matches!(value.as_str(), "true" | "yes")
    })
}

pub fn srand(seed: u64) {
    RNG_STATE.store(seed, Ordering::Relaxed);
}

fn next_rand_u32() -> u32 {
    let mut state = RNG_STATE.load(Ordering::Relaxed);
    if state == 0 {
        state = 0x853c_49e6_748f_ea9b;
    }
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    RNG_STATE.store(state, Ordering::Relaxed);
    (state >> 32) as u32
}

pub fn random_double(lo: f64, hi: f64) -> f64 {
    let unit = next_rand_u32() as f64 / u32::MAX as f64;
    lo + unit * (hi - lo)
}

pub fn read_file(filename: &str) -> String {
    let data = fs::read(filename).unwrap_or_else(|_| panic!("Could not open file: {filename}"));
    String::from_utf8_lossy(&data).to_string()
}

pub fn specific_load_test(
    loader: &IfcLoader,
    geometry_loader: &mut IfcGeometryProcessor,
    num: u32,
) {
    let _walls = loader.get_express_ids_with_type(ifc_schema::IFCSLAB);
    let mesh = geometry_loader.get_mesh(num);

    if should_write_files() {
        dump_mesh(&mesh, geometry_loader, "TEST.obj");
    }
}

pub fn get_alignments(
    loader: &IfcLoader,
    geometry_loader: &mut IfcGeometryProcessor,
) -> Vec<IfcAlignment> {
    let mut alignments = Vec::new();
    let elements = loader.get_express_ids_with_type(ifc_schema::IFCALIGNMENT);

    for express_id in elements {
        let mut alignment = geometry_loader.get_loader().get_alignment(
            express_id,
            IfcAlignment::default(),
            DMat4::IDENTITY,
            u32::MAX,
        );
        alignment.transform(geometry_loader.get_coordination_matrix());
        alignments.push(alignment);
    }

    if should_write_files() {
        dump_alignment(alignments.clone(), "V_ALIGN.obj", "H_ALIGN.obj");
    }

    for alignment in &mut alignments {
        let mut points_h = Vec::new();
        let mut points_v = Vec::new();
        for curve in &alignment.horizontal.curves {
            for point in &curve.base.points {
                points_h.push(*point);
            }
        }
        for curve in &alignment.vertical.curves {
            for point in &curve.base.points {
                points_v.push(*point);
            }
        }
        let mut curve = IfcCurve::default();
        curve.base.points = convert_2d_alignments_to_3d(&points_h, &points_v);
        alignment.absolute.curves.push(curve);
    }

    alignments
}

pub fn get_cross_sections_3d(
    loader: &IfcLoader,
    geometry_loader: &mut IfcGeometryProcessor,
) -> Vec<IfcCrossSections> {
    let mut cross_sections = Vec::new();
    let type_list = [
        ifc_schema::IFCSECTIONEDSOLID,
        ifc_schema::IFCSECTIONEDSURFACE,
        ifc_schema::IFCSECTIONEDSOLIDHORIZONTAL,
    ];

    for type_code in type_list {
        let elements = loader.get_express_ids_with_type(type_code);
        for express_id in elements {
            let cross_section = geometry_loader.get_loader().get_cross_sections_3d(
                express_id,
                false,
                DMat4::IDENTITY,
            );
            cross_sections.push(cross_section);
        }
    }

    if should_write_files() {
        dump_cross_sections(cross_sections.clone(), "CrossSection.obj");
    }

    cross_sections
}

pub fn read_value(loader: &IfcLoader, token: IfcTokenType) -> String {
    match token {
        IfcTokenType::String => loader.get_decoded_string_argument(),
        IfcTokenType::Enum => loader.get_string_argument().to_string(),
        IfcTokenType::Real => loader.get_double_argument_as_string().to_string(),
        IfcTokenType::Integer => loader.get_int_argument().to_string(),
        IfcTokenType::Ref => loader.get_ref_argument().to_string(),
        _ => String::new(),
    }
}

pub fn get_args(loader: &IfcLoader, in_object: bool, _in_list: bool) -> String {
    let mut arguments = String::new();
    let mut end_of_line = false;

    while !loader.is_at_end() && !end_of_line {
        let token = loader.get_token_type();
        match token {
            IfcTokenType::LineEnd => {
                end_of_line = true;
            }
            IfcTokenType::Empty => {
                arguments.push_str(" Empty ");
            }
            IfcTokenType::SetBegin => {
                arguments.push_str(&get_args(loader, false, true));
            }
            IfcTokenType::SetEnd => {
                end_of_line = true;
            }
            IfcTokenType::Label => {
                let mut obj = String::from(" type: LABEL ");
                loader.step_back();
                let _ = loader.get_string_argument();
                let _ = loader.get_token_type();
                obj.push_str(" value ");
                obj.push_str(&get_args(loader, true, false));
                obj.push(' ');
                arguments.push_str(&obj);
            }
            IfcTokenType::String
            | IfcTokenType::Enum
            | IfcTokenType::Real
            | IfcTokenType::Integer
            | IfcTokenType::Ref => {
                loader.step_back();
                let obj = if in_object {
                    read_value(loader, token)
                } else {
                    let mut obj = String::from(" type REF ");
                    obj.push_str(&read_value(loader, token));
                    obj.push(' ');
                    obj
                };
                arguments.push_str(&obj);
            }
            _ => {}
        }
    }

    arguments
}

pub fn get_line(loader: &IfcLoader, express_id: u32) -> String {
    if !loader.is_valid_express_id(express_id) {
        return String::new();
    }
    let line_type = loader.get_line_type(express_id);
    if line_type == 0 {
        return String::new();
    }

    loader.move_to_argument_offset(express_id, 0);
    let arguments = get_args(loader, false, false);

    format!("\"ID\": {express_id}, \"type\": {line_type}, \"arguments\": {arguments}}")
}

pub fn load_all_test(
    loader: &IfcLoader,
    geometry_loader: &mut IfcGeometryProcessor,
    id_to_export: u32,
) -> Vec<IfcFlatMesh> {
    let mut meshes = Vec::new();
    let schema = IfcSchemaManager::new();

    for &type_code in schema.get_ifc_element_list() {
        let elements = loader.get_express_ids_with_type(type_code);
        for express_id in elements {
            let mesh = geometry_loader.get_flat_mesh(express_id, true);

            if mesh.express_id == id_to_export && should_write_files() {
                dump_flat_mesh(&mesh, geometry_loader, "TEST_GEOM.obj");
            }

            for geom in &mesh.geometries {
                let _ = geometry_loader.get_geometry(geom.geometry_express_id);
            }

            meshes.push(mesh);
        }
    }

    meshes
}

pub fn get_all_rebars(
    loader: &IfcLoader,
    geometry_loader: &mut IfcGeometryProcessor,
) -> Vec<SweptDiskSolid> {
    let mut reinforcing_bars = Vec::new();
    let mut reinforcing_bars_transform: Vec<DMat4> = Vec::new();

    let elements = loader.get_express_ids_with_type(ifc_schema::IFCREINFORCINGBAR);
    for express_id in elements {
        let mesh = geometry_loader.get_flat_mesh(express_id, true);

        for geom in &mesh.geometries {
            let flat_geom = geometry_loader.get_geometry(geom.geometry_express_id);
            reinforcing_bars.push(flat_geom.get_swept_disk_solid());
            reinforcing_bars_transform.push(geom.transformation);
        }
    }

    reinforcing_bars
}

pub fn dump_refs(refs: &HashMap<u32, Vec<u32>>) {
    if let Ok(mut file) = File::create("refs.txt") {
        let mut prev: i32 = 0;
        for values in refs.values() {
            for value in values {
                let delta = *value as i32 - prev;
                let _ = write!(file, "{delta}");
                prev = *value as i32;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BenchMarkResult {
    pub file: String,
    pub time_ms: i64,
    pub size_bytes: u64,
}

pub fn benchmark() {
    let mut results = Vec::new();
    let path = Path::new("../../../benchmark/ifcfiles");

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("ifc") {
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        let file_path = path.to_string_lossy().to_string();
        let _content = read_file(&file_path);

        let start = ms();
        {
            // loader.load_file(content.as_bytes());
        }
        let time = ms() - start;

        results.push(BenchMarkResult {
            file: filename.clone(),
            time_ms: time,
            size_bytes: entry.metadata().map(|m| m.len()).unwrap_or(0),
        });

        println!("Reading {filename} took {time}ms");
    }

    println!("\n\nResults:");

    let mut avg_mb_sec = 0.0;
    for result in &results {
        let mb_sec = result.size_bytes as f64 / 1000.0 / result.time_ms as f64;
        avg_mb_sec += mb_sec;
        println!("{}: {} MB/sec", result.file, mb_sec);
    }

    if !results.is_empty() {
        avg_mb_sec /= results.len() as f64;
    }

    println!("\nAverage: {} MB/sec", avg_mb_sec);
    println!("\n\n");
}

pub fn test_triangle_decompose() {
    const NUM_TESTS: i32 = 100;
    const PTS_PER_TEST: u32 = 100;
    const EDGE_PTS_PER_TEST: u32 = 10;

    let scale_x = 650.0;
    let scale_y = 1.0;

    let a = DVec2::new(0.0, 0.0);
    let b = DVec2::new(scale_x, 0.0);
    let c = DVec2::new(0.0, scale_y);

    for i in 0..NUM_TESTS {
        srand(i as u64);

        let mut points = Vec::new();

        for _ in 0..PTS_PER_TEST {
            points.push(DVec2::new(
                random_double(0.0, scale_x),
                random_double(0.0, scale_y),
            ));
        }

        for _ in 0..EDGE_PTS_PER_TEST {
            let e1 = b - a;
            let e2 = c - a;
            let e3 = b - c;

            points.push(a + e1 * random_double(0.0, 1.0));
            points.push(a + e2 * random_double(0.0, 1.0));
            points.push(c + e3 * random_double(0.0, 1.0));
        }

        println!("Start test {i}");

        let _swapped = false;

        let mut pts = Vec::new();
        for point in points {
            pts.push(Point::new(point.x, point.y));
        }
    }
}

pub fn main() -> i32 {
    println!("Hello web IFC test!");

    #[derive(Clone, Debug)]
    struct LoaderSettings {
        coordinate_to_origin: bool,
        circle_segments: u16,
        tape_size: u32,
        memory_limit: u64,
        linewriter_buffer: u32,
        tolerance_plane_intersection: f64,
        tolerance_plane_deviation: f64,
        tolerance_back_deviation_distance: f64,
        tolerance_inside_outside_perimeter: f64,
        tolerance_scalar_equality: f64,
        plane_refit_iterations: u16,
        boolean_union_threshold: u16,
    }

    let mut settings = LoaderSettings {
        coordinate_to_origin: false,
        circle_segments: 12,
        tape_size: 67_108_864,
        memory_limit: 2_147_483_648,
        linewriter_buffer: 10_000,
        tolerance_plane_intersection: 1.0E-01,
        tolerance_plane_deviation: 3.0E-04,
        tolerance_back_deviation_distance: 3.0E-04,
        tolerance_inside_outside_perimeter: 1.0E-10,
        tolerance_scalar_equality: 1.0E-04,
        plane_refit_iterations: 10,
        boolean_union_threshold: 150,
    };

    settings.coordinate_to_origin = true;

    let schema_manager = Arc::new(IfcSchemaManager::new());
    let mut loader = IfcLoader::new(
        settings.tape_size,
        settings.memory_limit,
        settings.linewriter_buffer,
        schema_manager.clone(),
    );

    let content = read_file("C:/Users/qmoya/Desktop/MODELS/1092_A.ifc");
    let content_bytes = Arc::new(content.into_bytes());

    let start = ms();
    loader.load_file_with_callback(Arc::new({
        let content_bytes = Arc::clone(&content_bytes);
        move |dest: &mut [u8], source_offset: usize, dest_size: usize| {
            let length =
                std::cmp::min(content_bytes.len().saturating_sub(source_offset), dest_size);
            dest[..length].copy_from_slice(&content_bytes[source_offset..source_offset + length]);
            length as u32
        }
    }));
    let time = ms() - start;

    println!("Reading took {time}ms");

    let mut geometry_loader = IfcGeometryProcessor::new(
        &loader,
        schema_manager.as_ref(),
        settings.circle_segments,
        settings.coordinate_to_origin,
        settings.tolerance_plane_intersection,
        settings.tolerance_plane_deviation,
        settings.tolerance_back_deviation_distance,
        settings.tolerance_inside_outside_perimeter,
        settings.tolerance_scalar_equality,
        settings.plane_refit_iterations as f64,
        settings.boolean_union_threshold as f64,
    );

    let start = ms();
    specific_load_test(&loader, &mut geometry_loader, 85_583);
    let time = ms() - start;

    println!("Generating geometry took {time}ms");
    println!("Done");

    0
}
