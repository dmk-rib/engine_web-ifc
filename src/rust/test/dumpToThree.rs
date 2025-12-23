//! Rust port of `test/dumpToThree.h`.

#![allow(dead_code)]

use std::fmt::Write as _;

use glam::{DMat4, DVec2, DVec3, DVec4};

use crate::test::io_helpers::write_file;
use crate::web_ifc::geometry::ifc_geometry_processor::IfcGeometryProcessor;
use crate::web_ifc::geometry::representation::geometry::{
    normalize_ifc, IfcAlignment, IfcComposedMesh, IfcCrossSections, IfcFlatMesh,
};
use crate::web_ifc::geometry::representation::ifc_curve::IfcCurve;
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;

pub mod dump {
    use super::*;

    pub const THREE_JS_VIEWER_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DumpCurveToHtml</title>
    <style>
        body { margin: 0; overflow: hidden; }
        canvas { display: block; }
    </style>
</head>
<body>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r134/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.134.0/examples/js/controls/OrbitControls.js"></script>
    <script>
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0xcccccc);
        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        camera.position.set(50, 50, 50);
        camera.lookAt(0, 0, 0);
        camera.up.set(0, 0, 1); // Set Z as up direction
        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(window.innerWidth, window.innerHeight);
        document.body.appendChild(renderer.domElement);
        const controls = new THREE.OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.screenSpacePanning = true; // Allow panning in XY plane
        controls.minDistance = 1;
        controls.maxDistance = 500;
        controls.mouseButtons = {
            LEFT: THREE.MOUSE.ROTATE,
            MIDDLE: THREE.MOUSE.PAN,
            RIGHT: THREE.MOUSE.DOLLY
        };
        controls.enablePan = true;
        controls.panSpeed = 0.5;
        controls.up = new THREE.Vector3(0, 0, 1); // Ensure Z is up for rotations
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
        scene.add(ambientLight);
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
        directionalLight.position.set(1, 1, 1);
        scene.add(directionalLight);

        const objData = "{}";
        const blackMaterial = new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 2 });
        const grayMaterial = new THREE.LineBasicMaterial({ color: 0x888888, linewidth: 2 });

        // Parse OBJ data manually to create individual line segments
        function parseOBJ(objText) {
            const vertices = [];
            const lines = [];
            const linesArray = objText.split('\n');
            linesArray.forEach(line => {
                line = line.trim();
                if (line.startsWith('v ')) {
                    const parts = line.split(/\s+/).slice(1).map(parseFloat);
                    vertices.push(new THREE.Vector3(parts[0], parts[1], parts[2]));
                } else if (line.startsWith('l ')) {
                    const indices = line.split(/\s+/).slice(1).map(i => parseInt(i) - 1);
                    lines.push(indices);
                }
            });
            return { vertices, lines };
        }

        // Create individual Line objects for each segment
        const { vertices, lines } = parseOBJ(objData);
        lines.forEach((lineIndices, index) => {
            const geometry = new THREE.BufferGeometry().setFromPoints([
                vertices[lineIndices[0]],
                vertices[lineIndices[1]]
            ]);
            const material = (index % 2 === 0) ? blackMaterial : grayMaterial;
            const line = new THREE.Line(geometry, material);
            scene.add(line);
        });

        // Compute bounding box from vertices
        const box = new THREE.Box3();
        vertices.forEach(vertex => box.expandByPoint(vertex));
        const minPoint = box.min;
        const center = box.getCenter(new THREE.Vector3());

        // Add XY grid (10x10 lines, 10m spacing, 100x100m total) in XY plane at Z=0, positioned at minPoint
        const grid = new THREE.GridHelper(100, 10, 0x888888, 0x888888);
        grid.rotation.x = -Math.PI / 2; // Rotate grid to lie in XY plane with Z up
        grid.position.set(minPoint.x, minPoint.y, minPoint.z); // Position at min point
        scene.add(grid);

        // Add coordinate axes (X: red, Y: green, Z: blue, 200m length), positioned at minPoint
        const axesGroup = new THREE.Group();
        axesGroup.position.set(minPoint.x, minPoint.y, minPoint.z);
        const axesMaterialX = new THREE.LineBasicMaterial({ color: 0xff0000 });
        const axesMaterialY = new THREE.LineBasicMaterial({ color: 0x00ff00 });
        const axesMaterialZ = new THREE.LineBasicMaterial({ color: 0x0000ff });
        const axesGeometryX = new THREE.BufferGeometry().setFromPoints([
            new THREE.Vector3(0, 0, 0), new THREE.Vector3(200, 0, 0)
        ]);
        const axesGeometryY = new THREE.BufferGeometry().setFromPoints([
            new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 200, 0)
        ]);
        const axesGeometryZ = new THREE.BufferGeometry().setFromPoints([
            new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 0, 200)
        ]);
        const xAxis = new THREE.Line(axesGeometryX, axesMaterialX);
        const yAxis = new THREE.Line(axesGeometryY, axesMaterialY);
        const zAxis = new THREE.Line(axesGeometryZ, axesMaterialZ);
        axesGroup.add(xAxis);
        axesGroup.add(yAxis);
        axesGroup.add(zAxis);
        scene.add(axesGroup);

        // Zoom to the bounding box
        const size = box.getSize(new THREE.Vector3());
        const maxDim = Math.max(size.x, size.y, size.z);
        const fov = camera.fov * (Math.PI / 180);
        let cameraDistance = maxDim / Math.tan(fov / 2);
        cameraDistance *= 1.1; // Padding
        camera.position.set(center.x, center.y, center.z + cameraDistance);
        camera.lookAt(center);
        controls.target.copy(center);

        function animate() {
            requestAnimationFrame(animate);
            controls.update();
            renderer.render(scene, camera);
        }
        animate();
        window.addEventListener('resize', () => {
            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(window.innerWidth, window.innerHeight);
        });
    </script>
</body>
</html>
"#;

    fn escape_obj_data(obj_data: &str) -> String {
        let mut escaped = String::new();
        for c in obj_data.chars() {
            match c {
                '\n' => escaped.push_str("\\n"),
                '"' => escaped.push_str("\\\""),
                _ => escaped.push(c),
            }
        }
        escaped
    }

    pub fn make_three_js_viewer(input: Vec<DVec3>, indices: Vec<u32>) -> String {
        let mut obj = String::new();
        if !input.is_empty() {
            for pt in &input {
                let _ = writeln!(obj, "v {} {} {}", pt.x, pt.y, pt.z);
            }

            if !indices.is_empty() {
                for pair in indices.chunks(2) {
                    if pair.len() == 2 {
                        let _ = writeln!(obj, "l {} {}", pair[0] + 1, pair[1] + 1);
                    }
                }
            } else {
                for i in 0..input.len().saturating_sub(1) {
                    let _ = writeln!(obj, "l {} {}", i + 1, i + 2);
                }
            }
        }

        let escaped = escape_obj_data(&obj);
        THREE_JS_VIEWER_TEMPLATE.replace("{}", &escaped)
    }

    pub fn make_three_js_viewer_lines(lines: Vec<Vec<DVec2>>) -> String {
        let mut obj = String::new();
        let mut vertex_offset = 0usize;
        for line in &lines {
            for pt in line {
                let _ = writeln!(obj, "v {} {} 0", pt.x, pt.y);
            }
            for i in 0..line.len().saturating_sub(1) {
                let _ = writeln!(obj, "l {} {}", vertex_offset + i + 1, vertex_offset + i + 2);
            }
            vertex_offset += line.len();
        }

        let escaped = escape_obj_data(&obj);
        THREE_JS_VIEWER_TEMPLATE.replace("{}", &escaped)
    }

    pub fn dump_lines_to_html(lines: Vec<Vec<DVec2>>, filename: &str) {
        write_file(filename, &make_three_js_viewer_lines(lines));
    }

    pub fn gradient_vertical_to_obj_three(geom: &[IfcCurve]) -> String {
        let mut obj = String::new();
        for curve in geom {
            if !curve.base.points.is_empty() {
                for point in &curve.base.points {
                    let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                    let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                }
            }
        }
        let mut idx = 0u32;
        for curve in geom {
            if !curve.base.points.is_empty() {
                for _ in 0..curve.base.points.len().saturating_sub(1) {
                    let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                    idx += 1;
                }
            }
        }
        obj
    }

    pub fn gradient_horizontal_to_obj_three(geom: &IfcCurve) -> String {
        let mut obj = String::new();
        for point in &geom.base.points {
            let t = DVec4::new(point.x, point.y, 0.0, 1.0);
            let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
        }
        let mut idx = 0u32;
        for _ in 0..geom.base.points.len().saturating_sub(1) {
            let _ = writeln!(obj, "l {} {}", idx, idx + 1);
            idx += 1;
        }
        obj
    }

    pub fn v_alignment_to_obj_three(geom: &[IfcAlignment]) -> String {
        let mut obj = String::new();
        for alignment in geom {
            for curve in &alignment.vertical.curves {
                for point in &curve.base.points {
                    let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                    let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                }
            }
        }
        let mut idx = 0u32;
        for alignment in geom {
            if !alignment.vertical.curves.is_empty() {
                for curve in &alignment.vertical.curves {
                    if !curve.base.points.is_empty() {
                        for _ in 0..curve.base.points.len().saturating_sub(1) {
                            let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                            idx += 1;
                        }
                    }
                }
            }
        }
        obj
    }

    pub fn h_alignment_to_obj_three(geom: &[IfcAlignment]) -> String {
        let mut obj = String::new();
        for alignment in geom {
            for curve in &alignment.horizontal.curves {
                if !curve.base.points.is_empty() {
                    for point in &curve.base.points {
                        let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                        let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                    }
                }
            }
        }
        let mut idx = 0u32;
        for alignment in geom {
            for curve in &alignment.horizontal.curves {
                if !curve.base.points.is_empty() {
                    for _ in 0..curve.base.points.len().saturating_sub(1) {
                        let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                        idx += 1;
                    }
                }
            }
        }
        obj
    }

    pub fn cross_section_to_obj_three(geom: &[IfcCrossSections]) -> String {
        let mut obj = String::new();
        for section in geom {
            for curve in &section.curves {
                for point in &curve.base.points {
                    let t = DVec4::new(point.x, point.y, 0.0, 1.0);
                    let _ = writeln!(obj, "v {} {} {}", t.x, t.y, t.z);
                }
            }
        }
        let mut idx = 0u32;
        for section in geom {
            for curve in &section.curves {
                for _ in 0..curve.base.points.len().saturating_sub(1) {
                    let _ = writeln!(obj, "l {} {}", idx, idx + 1);
                    idx += 1;
                }
            }
        }
        obj
    }

    pub fn to_obj_three_geometry(
        geom: &IfcGeometry,
        offset: &mut usize,
        transform: DMat4,
        input_scale: f64,
    ) -> String {
        let mut obj = String::new();
        let scale = input_scale;

        for i in 0..geom.base.base.num_points as usize {
            let point = geom.base.base.get_point(i);
            let t = transform * DVec4::new(point.x, point.y, point.z, 1.0);
            let _ = writeln!(obj, "v {} {} {}", t.x * scale, t.y * scale, t.z * scale);
        }

        for i in 0..geom.base.base.num_faces as usize {
            let face = geom.base.base.get_face(i);
            let _ = writeln!(
                obj,
                "f {}// {}// {}//",
                face.i0 + 1 + *offset as u32,
                face.i1 + 1 + *offset as u32,
                face.i2 + 1 + *offset as u32
            );
        }

        *offset += geom.base.base.num_points as usize;
        obj
    }

    pub fn to_obj_three_mesh(
        mesh: &IfcComposedMesh,
        processor: &mut IfcGeometryProcessor,
        offset: &mut usize,
        mat: DMat4,
    ) -> String {
        let mut complete = String::new();
        let trans = mat * mesh.transformation;
        let geom = processor.get_geometry(mesh.express_id).clone();
        complete.push_str(&to_obj_three_geometry(&geom, offset, trans, 1.0));
        for child in &mesh.children {
            complete.push_str(&to_obj_three_mesh(child, processor, offset, trans));
        }
        complete
    }

    pub fn dump_gradient_curve_three(
        curves: Vec<IfcCurve>,
        curve: IfcCurve,
        filename_v: &str,
        filename_h: &str,
    ) {
        write_file(filename_v, &gradient_vertical_to_obj_three(&curves));
        write_file(filename_h, &gradient_horizontal_to_obj_three(&curve));
    }

    pub fn dump_section_curves_three(curves: Vec<IfcCurve>, filename: &str) {
        write_file(filename, &gradient_vertical_to_obj_three(&curves));
    }

    pub fn dump_alignment_three(align: Vec<IfcAlignment>, filename_v: &str, filename_h: &str) {
        write_file(filename_v, &v_alignment_to_obj_three(&align));
        write_file(filename_h, &h_alignment_to_obj_three(&align));
    }

    pub fn dump_cross_sections_three(cross_section: Vec<IfcCrossSections>, filename: &str) {
        write_file(filename, &cross_section_to_obj_three(&cross_section));
    }

    pub fn dump_ifc_geometry_to_path_three(geom: &IfcGeometry, path: &str, input_scale: f64) {
        let mut offset = 0usize;
        write_file(
            path,
            &to_obj_three_geometry(geom, &mut offset, DMat4::IDENTITY, input_scale),
        );
    }

    pub fn to_obj_three_geometry_default(
        geom: &IfcGeometry,
        offset: &mut usize,
        transform: DMat4,
    ) -> String {
        to_obj_three_geometry(geom, offset, transform, 1.0)
    }

    pub fn to_obj_three_flat_mesh(
        mesh: &IfcFlatMesh,
        processor: &mut IfcGeometryProcessor,
        offset: &mut usize,
        mat: DMat4,
    ) -> String {
        let mut complete = String::new();
        for geom in &mesh.geometries {
            let flat_geom = processor.get_geometry(geom.geometry_express_id).clone();
            let trans = mat * geom.transformation;
            complete.push_str(&to_obj_three_geometry(&flat_geom, offset, trans, 1.0));
        }
        complete
    }

    pub fn dump_ifc_geometry_three(geom: &IfcGeometry, filename: &str) {
        let mut offset = 0usize;
        write_file(
            filename,
            &to_obj_three_geometry_default(geom, &mut offset, DMat4::IDENTITY),
        );
    }

    pub fn dump_flat_mesh_three(
        mesh: &IfcFlatMesh,
        processor: &mut IfcGeometryProcessor,
        filename: &str,
    ) {
        let mut offset = 0usize;
        write_file(
            filename,
            &to_obj_three_flat_mesh(mesh, processor, &mut offset, DMat4::IDENTITY),
        );
    }

    pub fn dump_curve_to_html(points: Vec<DVec3>, filename: &str, indices: Vec<u32>) {
        write_file(filename, &make_three_js_viewer(points, indices));
    }

    pub fn dump_mesh_three(
        mesh: &IfcComposedMesh,
        processor: &mut IfcGeometryProcessor,
        filename: &str,
    ) {
        let mut offset = 0usize;
        write_file(
            filename,
            &to_obj_three_mesh(mesh, processor, &mut offset, normalize_ifc()),
        );
    }
}

pub use dump::*;
