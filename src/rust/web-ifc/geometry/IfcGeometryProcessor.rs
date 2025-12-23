//! Rust port of IfcGeometryProcessor public API.

use std::collections::HashMap;

use glam::{DMat4, DVec4};

use crate::web_ifc::geometry::representation::geometry::{
    normalize_ifc, IfcBound3D, IfcComposedMesh, IfcFlatMesh, IfcSurface,
};
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;
use crate::web_ifc::geometry::IfcGeometryLoader;
use crate::web_ifc::geometry::operations::boolean_utils::fuzzy_bools;
use crate::web_ifc::geometry::operations::boolean_utils::geometry as bool_geometry;
use crate::web_ifc::geometry::operations::geometryutils;
use crate::web_ifc::parsing::ifc_loader::IfcLoader;
use crate::web_ifc::parsing::ifc_token_type::IfcTokenType;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;
use crate::web_ifc::schema::ifc_schema::{IFCBSPLINESURFACE, IFCBSPLINESURFACEWITHKNOTS, IFCCYLINDRICALSURFACE, IFCINDEXEDPOLYGONALFACE, IFCINDEXEDPOLYGONALFACEWITHVOIDS, IFCPLANE, IFCRATIONALBSPLINESURFACEWITHKNOTS, IFCSURFACEOFREVOLUTION, IFCSURFACEOFLINEAREXTRUSION};

#[derive(Clone, Debug)]
pub struct IfcGeometrySettings {
    pub coordinate_to_origin: bool,
    pub optimize_profiles: bool,
    pub export_polylines: bool,
    pub circle_segments: u16,
    pub tolerance_plane_intersection: f64,
    pub tolerance_plane_deviation: f64,
    pub tolerance_back_deviation_distance: f64,
    pub tolerance_inside_outside_perimeter: f64,
    pub tolerance_bounding_box: f64,
    pub boolean_union_threshold: u16,
}

impl Default for IfcGeometrySettings {
    fn default() -> Self {
        Self {
            coordinate_to_origin: false,
            optimize_profiles: true,
            export_polylines: false,
            circle_segments: 12,
            tolerance_plane_intersection: 1.0E-04,
            tolerance_plane_deviation: 1.0E-04,
            tolerance_back_deviation_distance: 1.0E-04,
            tolerance_inside_outside_perimeter: 1.0E-10,
            tolerance_bounding_box: 1.0E-02,
            boolean_union_threshold: 150,
        }
    }
}

#[derive(Debug, Default)]
pub struct BooleanManager;

impl BooleanManager {
    pub fn bool_process(
        &self,
        first_geoms: &[IfcGeometry],
        second_geoms: &mut [IfcGeometry],
        op: &str,
        settings: &IfcGeometrySettings,
    ) -> IfcGeometry {
        let mut final_result = IfcGeometry::default();

        for first_geom in first_geoms {
            let mut first_operator = first_geom.clone();
            for second_geom in second_geoms.iter() {
                let mut doit = true;
                if second_geom.base.base.num_faces == 0 {
                    doit = false;
                }
                if first_operator.base.base.num_faces == 0 && op != "UNION" {
                    break;
                }

                if doit {
                    let mut second_operator = IfcGeometry::default();
                    if second_geom.half_space {
                        let origin = second_geom.half_space_origin;
                        let x = second_geom.half_space_x - origin;
                        let y = second_geom.half_space_y - origin;
                        let z = second_geom.half_space_z - origin;

                        let trans = DMat4::from_cols(
                            x.extend(0.0),
                            y.extend(0.0),
                            z.extend(0.0),
                            DVec4::new(0.0, 0.0, 0.0, 1.0),
                        );

                        let mut scale_x = 1.0;
                        let mut scale_y = 1.0;
                        let mut scale_z = 1.0;

                        for i in 0..first_operator.base.base.num_points {
                            let p = first_operator.base.base.get_point(i as usize);
                            let vec = p - origin;
                            let dx = vec.dot(x);
                            let dy = vec.dot(y);
                            let dz = vec.dot(z);
                            if dx.abs() > scale_x {
                                scale_x = dx.abs();
                            }
                            if dy.abs() > scale_y {
                                scale_y = dy.abs();
                            }
                            if dz.abs() > scale_z {
                                scale_z = dz.abs();
                            }
                        }

                        second_operator.add_geometry(
                            second_geom.base.base.clone(),
                            trans,
                            scale_x * 2.0,
                            scale_y * 2.0,
                            scale_z * 2.0,
                            second_geom.half_space_origin,
                        );
                    } else {
                        second_operator = second_geom.clone();
                    }

                    first_operator.base.base.build_planes();
                    second_operator.base.base.build_planes();

                    fuzzy_bools::set_epsilons(
                        settings.tolerance_plane_intersection,
                        settings.tolerance_plane_deviation,
                        settings.tolerance_back_deviation_distance,
                        settings.tolerance_inside_outside_perimeter,
                        settings.tolerance_bounding_box,
                        0.0,
                    );

                    if op == "DIFFERENCE" {
                        first_operator =
                            self.subtract(first_operator, second_operator);
                    } else if op == "UNION" {
                        first_operator = self.union(first_operator, second_operator);
                    }
                }
            }
            final_result.merge_geometry(first_operator.base.base);
        }

        final_result
    }

    fn convert_to_engine(&self, geom: &IfcGeometry) -> bool_geometry::Geometry {
        let mut new_geom = bool_geometry::Geometry::default();
        new_geom.fvertex_data = geom.base.base.fvertex_data.clone();
        new_geom.vertex_data = geom.base.base.vertex_data.clone();
        new_geom.index_data = geom.base.base.index_data.clone();
        new_geom.plane_data = geom.base.base.plane_data.clone();
        new_geom.num_points = geom.base.base.num_points;
        new_geom.num_faces = geom.base.base.num_faces;
        for plane in &geom.base.base.planes {
            let mut new_plane = bool_geometry::SimplePlane::default();
            new_plane.distance = plane.distance;
            new_plane.normal = plane.normal;
            new_geom.planes.push(new_plane);
        }
        new_geom.has_planes = geom.base.base.has_planes;
        new_geom
    }

    fn convert_to_web_ifc(&self, geom: bool_geometry::Geometry) -> IfcGeometry {
        let mut new_geom = IfcGeometry::default();
        new_geom.base.base.fvertex_data = geom.fvertex_data;
        new_geom.base.base.vertex_data = geom.vertex_data;
        new_geom.base.base.index_data = geom.index_data;
        new_geom.base.base.plane_data = geom.plane_data;
        new_geom.base.base.num_points = geom.num_points;
        new_geom.base.base.num_faces = geom.num_faces;
        for (id, plane) in geom.planes.iter().enumerate() {
            new_geom.base.base.planes.push(crate::web_ifc::geometry::operations::bim_geometry::plane::Plane {
                id,
                normal: plane.normal,
                distance: plane.distance,
            });
        }
        new_geom.base.base.has_planes = geom.has_planes;
        new_geom
    }

    fn union(&self, first: IfcGeometry, second: IfcGeometry) -> IfcGeometry {
        let first_eng = self.convert_to_engine(&first);
        let second_eng = self.convert_to_engine(&second);
        self.convert_to_web_ifc(fuzzy_bools::union(&first_eng, &second_eng))
    }

    fn subtract(&self, first: IfcGeometry, second: IfcGeometry) -> IfcGeometry {
        let first_eng = self.convert_to_engine(&first);
        let second_eng = self.convert_to_engine(&second);
        self.convert_to_web_ifc(fuzzy_bools::subtract(&first_eng, &second_eng))
    }
}

#[derive(Debug)]
pub struct IfcGeometryProcessor<'a> {
    settings: IfcGeometrySettings,
    express_id_to_geometry: HashMap<u32, IfcGeometry>,
    geometry_loader: IfcGeometryLoader<'a>,
    loader: &'a IfcLoader,
    schema_manager: &'a IfcSchemaManager,
    transformation: DMat4,
    coordination_matrix: DMat4,
    is_coordinated: bool,
    bool_engine: BooleanManager,
    predefined_cylinder: IfcGeometry,
    predefined_cube: IfcGeometry,
}

impl<'a> IfcGeometryProcessor<'a> {
    pub fn new(
        loader: &'a IfcLoader,
        schema_manager: &'a IfcSchemaManager,
        circle_segments: u16,
        coordinate_to_origin: bool,
        tolerance_plane_intersection: f64,
        tolerance_plane_deviation: f64,
        tolerance_back_deviation_distance: f64,
        tolerance_inside_outside_perimeter: f64,
        tolerance_scalar_equality: f64,
        plane_refit_iterations: f64,
        boolean_union_threshold: f64,
    ) -> Self {
        geometryutils::set_epsilons(tolerance_scalar_equality, plane_refit_iterations, boolean_union_threshold);
        let geometry_loader = IfcGeometryLoader::new(
            loader,
            schema_manager,
            circle_segments,
            tolerance_plane_intersection,
            tolerance_plane_deviation,
            tolerance_back_deviation_distance,
            tolerance_inside_outside_perimeter,
            tolerance_scalar_equality,
            plane_refit_iterations,
            boolean_union_threshold,
        );
        let mut settings = IfcGeometrySettings::default();
        settings.coordinate_to_origin = coordinate_to_origin;
        settings.circle_segments = circle_segments;
        settings.tolerance_plane_intersection = tolerance_plane_intersection;
        settings.tolerance_plane_deviation = tolerance_plane_deviation;
        settings.tolerance_back_deviation_distance = tolerance_back_deviation_distance;
        settings.tolerance_inside_outside_perimeter = tolerance_inside_outside_perimeter;
        settings.boolean_union_threshold = boolean_union_threshold as u16;
        Self {
            settings,
            express_id_to_geometry: HashMap::new(),
            geometry_loader,
            loader,
            schema_manager,
            transformation: DMat4::IDENTITY,
            coordination_matrix: DMat4::IDENTITY,
            is_coordinated: false,
            bool_engine: BooleanManager::default(),
            predefined_cylinder: IfcGeometry::default(),
            predefined_cube: IfcGeometry::default(),
        }
    }

    pub fn get_geometry(&mut self, express_id: u32) -> &IfcGeometry {
        self.express_id_to_geometry
            .entry(express_id)
            .or_insert_with(IfcGeometry::default)
    }

    pub fn get_flat_mesh(&mut self, express_id: u32, apply_linear_scaling_factor: bool) -> IfcFlatMesh {
        let mut flat_mesh = IfcFlatMesh::default();
        flat_mesh.express_id = express_id;

        let composed_mesh = self.get_mesh(express_id);
        let mut mat = DMat4::IDENTITY;
        if apply_linear_scaling_factor {
            mat = DMat4::from_scale(glam::DVec3::new(
                self.geometry_loader.get_linear_scaling_factor(),
                self.geometry_loader.get_linear_scaling_factor(),
                self.geometry_loader.get_linear_scaling_factor(),
            ));
        }

        let color = DVec4::new(1.0, 1.0, 1.0, 1.0);
        let has_color = false;
        self.add_composed_mesh_to_flat_mesh(
            &mut flat_mesh,
            &composed_mesh,
            self.transformation * normalize_ifc() * mat,
            color,
            has_color,
        );

        flat_mesh
    }

    pub fn get_mesh(&self, express_id: u32) -> IfcComposedMesh {
        let mut mesh = IfcComposedMesh::default();
        mesh.express_id = express_id;

        if let Some(geom) = self.express_id_to_geometry.get(&express_id) {
            if geom.base.base.num_faces > 0 {
                mesh.has_geometry = true;
            }
        }

        mesh
    }

    pub fn set_transformation(&mut self, val: [f64; 16]) {
        let v1 = DVec4::new(val[0], val[1], val[2], val[3]);
        let v2 = DVec4::new(val[4], val[5], val[6], val[7]);
        let v3 = DVec4::new(val[8], val[9], val[10], val[11]);
        let v4 = DVec4::new(val[12], val[13], val[14], val[15]);
        self.transformation = DMat4::from_cols(v1, v2, v3, v4);
    }

    pub fn get_flat_coordination_matrix(&self) -> [f64; 16] {
        self.coordination_matrix.to_cols_array()
    }

    pub fn get_coordination_matrix(&self) -> DMat4 {
        self.coordination_matrix
    }

    pub fn clear(&mut self) {
        self.express_id_to_geometry.clear();
    }

    pub fn clone_processor(&self, loader: &'a IfcLoader) -> Self {
        let mut cloned = Self::new(
            loader,
            self.schema_manager,
            self.settings.circle_segments,
            self.settings.coordinate_to_origin,
            self.settings.tolerance_plane_intersection,
            self.settings.tolerance_plane_deviation,
            self.settings.tolerance_back_deviation_distance,
            self.settings.tolerance_inside_outside_perimeter,
            0.0,
            0.0,
            self.settings.boolean_union_threshold as f64,
        );
        cloned.express_id_to_geometry = self.express_id_to_geometry.clone();
        cloned
    }

    pub fn add_composed_mesh_to_flat_mesh(
        &mut self,
        flat_mesh: &mut IfcFlatMesh,
        composed_mesh: &IfcComposedMesh,
        parent_matrix: DMat4,
        color: DVec4,
        has_color: bool,
    ) {
        let mut new_parent_color = color;
        let mut new_has_color = has_color;
        let new_matrix = parent_matrix * composed_mesh.transformation;

        if composed_mesh.has_color && !has_color {
            new_has_color = true;
            new_parent_color = composed_mesh.color;
        }

        if composed_mesh.has_geometry {
            if let Some(geom) = self.express_id_to_geometry.get(&composed_mesh.express_id) {
                if geom.base.is_polygon && !self.settings.export_polylines {
                    return;
                }

                let mut geometry = crate::web_ifc::geometry::representation::geometry::IfcPlacedGeometry::default();

                if !self.is_coordinated && self.settings.coordinate_to_origin {
                    if geom.base.base.num_points > 0 {
                        let pt = geom.base.base.get_point(0);
                        let transformed = new_matrix * pt.extend(1.0);
                        self.coordination_matrix = DMat4::from_translation(-transformed.truncate());
                        self.is_coordinated = true;
                    }
                }

                let mut geom = geom.clone();
                if geometryutils::matrix_flips_triangles(new_matrix) {
                    geom.reverse_faces();
                }

                let translation = geom.normalize();

                if !composed_mesh.has_color {
                    geometry.color = new_parent_color;
                } else {
                    geometry.color = composed_mesh.color;
                    new_parent_color = composed_mesh.color;
                    new_has_color = composed_mesh.has_color;
                }

                geometry.transformation = self.coordination_matrix * new_matrix * translation;
                geometry.set_flat_transformation();
                geometry.geometry_express_id = composed_mesh.express_id;
                flat_mesh.geometries.push(geometry);
            }
        } else if composed_mesh.has_color {
            new_parent_color = composed_mesh.color;
            new_has_color = composed_mesh.has_color;
        }

        for child in &composed_mesh.children {
            self.add_composed_mesh_to_flat_mesh(
                flat_mesh,
                child,
                new_matrix,
                new_parent_color,
                new_has_color,
            );
        }
    }

    pub fn read_2d_array_of_three_indices(&self) -> Vec<u32> {
        let mut result = Vec::new();

        let _ = self.loader.get_token_type();
        while self.loader.get_token_type() == IfcTokenType::SetBegin {
            result.push(self.loader.get_int_argument() as u32);
            result.push(self.loader.get_int_argument() as u32);
            result.push(self.loader.get_int_argument() as u32);
            let _ = self.loader.get_token_type();
        }

        result
    }

    pub fn read_indexed_polygonal_face(
        &self,
        express_id: u32,
        bounds: &mut Vec<IfcBound3D>,
        points: &[glam::DVec3],
    ) {
        let line_type = self.loader.get_line_type(express_id);

        bounds.push(IfcBound3D {
            bound_type: crate::web_ifc::geometry::representation::geometry::IfcBoundType::OuterBound,
            orientation: true,
            curve: Default::default(),
        });

        match line_type {
            IFCINDEXEDPOLYGONALFACE | IFCINDEXEDPOLYGONALFACEWITHVOIDS => {
                self.loader.move_to_argument_offset(express_id, 0);
                let index_ids = self.loader.get_set_argument();

                for index_id in index_ids {
                    let index = self.loader.get_int_argument(index_id) as usize;
                    let point = points[index - 1];
                    bounds
                        .last_mut()
                        .expect("bound exists")
                        .curve
                        .base
                        .add(point, true);
                }

                if line_type == IFCINDEXEDPOLYGONALFACE {
                    return;
                }

                self.loader.move_to_argument_offset(express_id, 1);
                let _ = self.loader.get_token_type();

                while self.loader.get_token_type() == IfcTokenType::SetBegin {
                    bounds.push(IfcBound3D {
                        bound_type:
                            crate::web_ifc::geometry::representation::geometry::IfcBoundType::Bound,
                        orientation: true,
                        curve: Default::default(),
                    });

                    while self.loader.get_token_type() != IfcTokenType::SetEnd {
                        self.loader.step_back();
                        let index = self.loader.get_int_argument() as usize;
                        let point = points[index - 1];
                        bounds
                            .last_mut()
                            .expect("bound exists")
                            .curve
                            .base
                            .add(point, true);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_surface(&self, _express_id: u32) -> IfcSurface {
        let line_type = self.loader.get_line_type(_express_id);

        match line_type {
            IFCPLANE => {
                let mut surface = IfcSurface::default();
                self.loader.move_to_argument_offset(_express_id, 0);
                let location_id = self.loader.get_ref_argument();
                surface.transformation = self.geometry_loader.get_local_placement(location_id, glam::DVec3::ZERO);
                surface
            }
            IFCBSPLINESURFACE => {
                let mut surface = IfcSurface::default();
                let mut ctrl_pts: Vec<Vec<glam::DVec3>> = Vec::new();
                self.loader.move_to_argument_offset(_express_id, 0);
                let u_degree = self.loader.get_int_argument();
                self.loader.move_to_argument_offset(_express_id, 1);
                let v_degree = self.loader.get_int_argument();
                self.loader.move_to_argument_offset(_express_id, 2);
                let ctrl_point_groups = self.loader.get_set_list_argument();
                for set in ctrl_point_groups {
                    let mut list = Vec::new();
                    for token in set {
                        let point_id = self.loader.get_ref_argument(token);
                        list.push(self.geometry_loader.get_cartesian_point_3d(point_id));
                    }
                    ctrl_pts.push(list);
                }
                self.loader.move_to_argument_offset(_express_id, 3);
                let curve_type = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 4);
                let closed_u = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 5);
                let closed_v = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 6);
                let _self_intersect = self.loader.get_string_argument();

                surface.b_spline_surface.active = true;
                surface.b_spline_surface.u_degree = u_degree as f64;
                surface.b_spline_surface.v_degree = v_degree as f64;
                surface.b_spline_surface.control_points = ctrl_pts;
                surface.b_spline_surface.closed_u = closed_u;
                surface.b_spline_surface.closed_v = closed_v;
                surface.b_spline_surface.curve_type = curve_type;
                surface
            }
            IFCBSPLINESURFACEWITHKNOTS => {
                let mut surface = IfcSurface::default();
                let mut ctrl_pts: Vec<Vec<glam::DVec3>> = Vec::new();
                let mut u_mult = Vec::new();
                let mut v_mult = Vec::new();
                let mut u_knots = Vec::new();
                let mut v_knots = Vec::new();

                self.loader.move_to_argument_offset(_express_id, 0);
                let u_degree = self.loader.get_int_argument();
                self.loader.move_to_argument_offset(_express_id, 1);
                let v_degree = self.loader.get_int_argument();
                self.loader.move_to_argument_offset(_express_id, 2);
                let ctrl_point_groups = self.loader.get_set_list_argument();
                for set in ctrl_point_groups {
                    let mut list = Vec::new();
                    for token in set {
                        let point_id = self.loader.get_ref_argument(token);
                        list.push(self.geometry_loader.get_cartesian_point_3d(point_id));
                    }
                    ctrl_pts.push(list);
                }

                self.loader.move_to_argument_offset(_express_id, 3);
                let _curve_type = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 4);
                let _closed_u = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 5);
                let _closed_v = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 6);
                let _self_intersect = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 7);
                let knot_set_u = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 8);
                let knot_set_v = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 9);
                let indexes_set_u = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 10);
                let indexes_set_v = self.loader.get_set_argument();

                for token in knot_set_u {
                    u_mult.push(self.loader.get_int_argument(token) as u32);
                }
                for token in knot_set_v {
                    v_mult.push(self.loader.get_int_argument(token) as u32);
                }
                for token in indexes_set_u {
                    u_knots.push(self.loader.get_double_argument(token));
                }
                for token in indexes_set_v {
                    v_knots.push(self.loader.get_double_argument(token));
                }

                surface.b_spline_surface.active = true;
                surface.b_spline_surface.u_degree = u_degree as f64;
                surface.b_spline_surface.v_degree = v_degree as f64;
                surface.b_spline_surface.control_points = ctrl_pts;
                surface.b_spline_surface.u_multiplicity = u_mult;
                surface.b_spline_surface.v_multiplicity = v_mult;
                surface.b_spline_surface.u_knots = u_knots;
                surface.b_spline_surface.v_knots = v_knots;
                surface
            }
            IFCRATIONALBSPLINESURFACEWITHKNOTS => {
                let mut surface = IfcSurface::default();
                let mut ctrl_pts: Vec<Vec<glam::DVec3>> = Vec::new();
                let mut weight_pts: Vec<Vec<f64>> = Vec::new();
                let mut u_mult = Vec::new();
                let mut v_mult = Vec::new();
                let mut u_knots = Vec::new();
                let mut v_knots = Vec::new();

                self.loader.move_to_argument_offset(_express_id, 0);
                let u_degree = self.loader.get_int_argument();
                self.loader.move_to_argument_offset(_express_id, 1);
                let v_degree = self.loader.get_int_argument();
                self.loader.move_to_argument_offset(_express_id, 2);
                let ctrl_point_groups = self.loader.get_set_list_argument();
                for set in ctrl_point_groups {
                    let mut list = Vec::new();
                    for token in set {
                        let point_id = self.loader.get_ref_argument(token);
                        list.push(self.geometry_loader.get_cartesian_point_3d(point_id));
                    }
                    ctrl_pts.push(list);
                }
                self.loader.move_to_argument_offset(_express_id, 3);
                let _curve_type = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 4);
                let _closed_u = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 5);
                let _closed_v = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 6);
                let _self_intersect = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(_express_id, 7);
                let knot_set_u = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 8);
                let knot_set_v = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 9);
                let indexes_set_u = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 10);
                let indexes_set_v = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 12);
                let weight_groups = self.loader.get_set_list_argument();
                for set in weight_groups {
                    let mut list = Vec::new();
                    for token in set {
                        list.push(self.loader.get_double_argument(token));
                    }
                    weight_pts.push(list);
                }
                for token in knot_set_u {
                    u_mult.push(self.loader.get_int_argument(token) as u32);
                }
                for token in knot_set_v {
                    v_mult.push(self.loader.get_int_argument(token) as u32);
                }
                for token in indexes_set_u {
                    u_knots.push(self.loader.get_double_argument(token));
                }
                for token in indexes_set_v {
                    v_knots.push(self.loader.get_double_argument(token));
                }

                surface.b_spline_surface.active = true;
                surface.b_spline_surface.u_degree = u_degree as f64;
                surface.b_spline_surface.v_degree = v_degree as f64;
                surface.b_spline_surface.control_points = ctrl_pts;
                surface.b_spline_surface.u_multiplicity = u_mult;
                surface.b_spline_surface.v_multiplicity = v_mult;
                surface.b_spline_surface.u_knots = u_knots;
                surface.b_spline_surface.v_knots = v_knots;
                surface.b_spline_surface.weight_points = weight_pts;
                surface
            }
            IFCCYLINDRICALSURFACE => {
                let mut surface = IfcSurface::default();
                self.loader.move_to_argument_offset(_express_id, 0);
                let location_id = self.loader.get_ref_argument();
                surface.transformation = self.geometry_loader.get_local_placement(location_id, glam::DVec3::ZERO);
                self.loader.move_to_argument_offset(_express_id, 1);
                let radius = self.loader.get_double_argument();
                surface.cylinder_surface.active = true;
                surface.cylinder_surface.radius = radius;
                surface
            }
            IFCSURFACEOFREVOLUTION => {
                let mut surface = IfcSurface::default();
                self.loader.move_to_argument_offset(_express_id, 0);
                let profile_id = self.loader.get_ref_argument();
                let profile = self.geometry_loader.get_profile_3d(profile_id);
                self.loader.move_to_argument_offset(_express_id, 1);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    let placement_id = self.loader.get_ref_argument();
                    surface.transformation =
                        self.geometry_loader.get_local_placement(placement_id, glam::DVec3::ZERO);
                }
                self.loader.move_to_argument_offset(_express_id, 2);
                let location_id = self.loader.get_ref_argument();
                surface.revolution_surface.active = true;
                surface.revolution_surface.direction =
                    self.geometry_loader.get_local_placement(location_id, glam::DVec3::ZERO);
                surface.revolution_surface.profile = profile;
                surface
            }
            IFCSURFACEOFLINEAREXTRUSION => {
                let mut surface = IfcSurface::default();
                self.loader.move_to_argument_offset(_express_id, 0);
                let profile_id = self.loader.get_ref_argument();
                let profile = self.geometry_loader.get_profile(profile_id);
                self.loader.move_to_argument_offset(_express_id, 2);
                let direction_id = self.loader.get_ref_argument();
                let direction = self.geometry_loader.get_cartesian_point_3d(direction_id);
                self.loader.move_to_argument_offset(_express_id, 3);
                let mut length = 0.0;
                if self.loader.get_token_type() == IfcTokenType::Real {
                    self.loader.step_back();
                    length = self.loader.get_double_argument();
                }
                surface.extrusion_surface.active = true;
                surface.extrusion_surface.length = length;
                surface.extrusion_surface.profile = profile;
                surface.extrusion_surface.direction = direction;
                self.loader.move_to_argument_offset(_express_id, 1);
                let location_id = self.loader.get_ref_argument();
                surface.transformation =
                    self.geometry_loader.get_local_placement(location_id, glam::DVec3::ZERO);
                surface
            }
            _ => IfcSurface::default(),
        }
    }
}
