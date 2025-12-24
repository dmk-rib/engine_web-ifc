//! Rust port of IfcGeometryLoader public API.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::web_ifc::geometry::operations::bim_geometry::utils::{
    get_c_shaped_curve, get_ellipse_curve, get_i_shaped_curve, get_l_shaped_curve,
    get_rectangle_curve, get_t_shaped_curve, get_trapezium_curve, get_u_shaped_curve,
    get_z_shaped_curve, vector_to_angle, CONST_PI,
};
use crate::web_ifc::parsing::ifc_token_stream::IfcTokenType;
use crate::web_ifc::schema::ifc_schema as schema;

use glam::{DMat3, DMat4, DVec2, DVec3, DVec4};

use crate::web_ifc::geometry::representation::geometry::{
    IfcAlignment, IfcBound3D, IfcCrossSections, IfcProfile, IfcTrimmingSelect, TrimSense,
};
use crate::web_ifc::geometry::representation::ifc_curve::IfcCurve;
use crate::web_ifc::parsing::ifc_loader::IfcLoader;
use crate::web_ifc::schema::ifc_schema_manager::IfcSchemaManager;

#[derive(Debug)]
pub struct IfcGeometryLoader<'a> {
    loader: &'a IfcLoader,
    schema_manager: &'a IfcSchemaManager,
    circle_segments: u16,
    linear_scaling_factor: f64,
    squared_scaling_factor: f64,
    cubic_scaling_factor: f64,
    angular_scaling_factor: f64,
    angle_units: String,
    rel_voids: HashMap<u32, Vec<u32>>,
    rel_nests: HashMap<u32, Vec<u32>>,
    rel_aggregates: HashMap<u32, Vec<u32>>,
    styled_items: HashMap<u32, Vec<(u32, u32)>>,
    rel_materials: HashMap<u32, Vec<(u32, u32)>>,
    material_definitions: HashMap<u32, Vec<(u32, u32)>>,
    local_curves_list: RefCell<Vec<IfcCurve>>,
    local_curves_indices: RefCell<Vec<u32>>,
    cartesian_point_3d_cache: RefCell<HashMap<u32, DVec3>>,
    cartesian_point_2d_cache: RefCell<HashMap<u32, DVec2>>,
    express_id_to_placement: RefCell<HashMap<u32, DMat4>>,
}

impl<'a> IfcGeometryLoader<'a> {
    pub fn new(
        loader: &'a IfcLoader,
        schema_manager: &'a IfcSchemaManager,
        circle_segments: u16,
        _tolerance_plane_intersection: f64,
        _tolerance_plane_deviation: f64,
        _tolerance_back_deviation_distance: f64,
        _tolerance_inside_outside_perimeter: f64,
        _tolerance_scalar_equality: f64,
        _unused: f64,
        _boolean_union_threshold: f64,
    ) -> Self {
        let mut loader_instance = Self {
            loader,
            schema_manager,
            circle_segments,
            linear_scaling_factor: 1.0,
            squared_scaling_factor: 1.0,
            cubic_scaling_factor: 1.0,
            angular_scaling_factor: 1.0,
            angle_units: String::new(),
            rel_voids: HashMap::new(),
            rel_nests: HashMap::new(),
            rel_aggregates: HashMap::new(),
            styled_items: HashMap::new(),
            rel_materials: HashMap::new(),
            material_definitions: HashMap::new(),
            local_curves_list: RefCell::new(Vec::new()),
            local_curves_indices: RefCell::new(Vec::new()),
            cartesian_point_3d_cache: RefCell::new(HashMap::new()),
            cartesian_point_2d_cache: RefCell::new(HashMap::new()),
            express_id_to_placement: RefCell::new(HashMap::new()),
        };
        loader_instance.read_linear_scaling_factor();
        loader_instance.rel_voids = loader_instance.populate_rel_voids_map();
        loader_instance.rel_nests = loader_instance.populate_rel_nests_map();
        loader_instance.rel_aggregates = loader_instance.populate_rel_aggregates_map();
        loader_instance.styled_items = loader_instance.populate_styled_item_map();
        loader_instance.rel_materials = loader_instance.populate_rel_materials_map();
        loader_instance.material_definitions = loader_instance.populate_material_definitions_map();
        loader_instance
    }

    pub fn reset_cache(&mut self) {
        self.rel_voids = self.populate_rel_voids_map();
        self.rel_aggregates = self.populate_rel_aggregates_map();
        self.rel_nests = self.populate_rel_nests_map();
        self.styled_items = self.populate_styled_item_map();
        self.rel_materials = self.populate_rel_materials_map();
        self.material_definitions = self.populate_material_definitions_map();
    }

    pub fn get_axis1_placement(&self, _express_id: u32) -> [DVec3; 2] {
        self.loader.move_to_argument_offset(_express_id, 0);
        let location_id = self.loader.get_ref_argument();
        let dir_token = self.loader.get_token_type();

        let mut axis = DVec3::new(0.0, 0.0, 1.0);
        if dir_token == IfcTokenType::Ref {
            self.loader.step_back();
            axis = self.get_cartesian_point_3d(self.loader.get_ref_argument());
        }

        let pos = self.get_cartesian_point_3d(location_id);

        [axis, pos]
    }

    pub fn get_axis2_placement_2d(&self, _express_id: u32) -> DMat3 {
        if _express_id < 1 {
            return DMat3::IDENTITY;
        }

        let line_type = self.loader.get_line_type(_express_id);
        match line_type {
            schema::IFCAXIS2PLACEMENT2D => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let location_id = self.loader.get_ref_argument();
                let dir_token = self.loader.get_token_type();

                let mut x_axis = DVec2::new(1.0, 0.0);
                if dir_token == IfcTokenType::Ref {
                    self.loader.step_back();
                    let axis = self.get_cartesian_point_2d(self.loader.get_ref_argument());
                    if axis.length() > 0.0 {
                        x_axis = axis.normalize();
                    }
                }

                let pos = self.get_cartesian_point_2d(location_id);
                let y_axis = DVec2::new(-x_axis.y, x_axis.x);

                DMat3::from_cols(
                    DVec3::new(x_axis.x, x_axis.y, 0.0),
                    DVec3::new(y_axis.x, y_axis.y, 0.0),
                    DVec3::new(pos.x, pos.y, 1.0),
                )
            }
            schema::IFCCARTESIANTRANSFORMATIONOPERATOR2D
            | schema::IFCCARTESIANTRANSFORMATIONOPERATOR2DNONUNIFORM => {
                let mut scale1 = 1.0;
                let mut scale2 = 1.0;

                let mut axis1 = DVec2::new(1.0, 0.0);
                let mut axis2 = DVec2::new(0.0, 1.0);

                self.loader.move_to_argument_offset(_express_id, 0);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    let axis = self.get_cartesian_point_3d(self.loader.get_ref_argument());
                    if axis.length() > 0.0 {
                        axis1 = axis.truncate().normalize();
                    }
                }
                self.loader.move_to_argument_offset(_express_id, 1);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    let axis = self.get_cartesian_point_3d(self.loader.get_ref_argument());
                    if axis.length() > 0.0 {
                        axis2 = axis.truncate().normalize();
                    }
                }

                self.loader.move_to_argument_offset(_express_id, 2);
                let pos_id = self.loader.get_ref_argument();
                let pos = self.get_cartesian_point_2d(pos_id);

                self.loader.move_to_argument_offset(_express_id, 3);
                if self.loader.get_token_type() == IfcTokenType::Real {
                    self.loader.step_back();
                    scale1 = self.loader.get_double_argument();
                }

                if line_type == schema::IFCCARTESIANTRANSFORMATIONOPERATOR2DNONUNIFORM {
                    self.loader.move_to_argument_offset(_express_id, 4);
                    if self.loader.get_token_type() == IfcTokenType::Real {
                        self.loader.step_back();
                        scale2 = self.loader.get_double_argument();
                    }
                }

                if line_type == schema::IFCCARTESIANTRANSFORMATIONOPERATOR2D {
                    scale2 = scale1;
                }

                DMat3::from_cols(
                    DVec3::new(axis1.x * scale1, axis1.y * scale1, 0.0),
                    DVec3::new(axis2.x * scale2, axis2.y * scale2, 0.0),
                    DVec3::new(pos.x, pos.y, 1.0),
                )
            }
            _ => {
                eprintln!(
                    "[get_axis2_placement_2d] unexpected 2D placement type {}",
                    _express_id
                );
                DMat3::IDENTITY
            }
        }
    }

    pub fn get_local_placement(&self, _express_id: u32, _vector: DVec3) -> DMat4 {
        if let Some(placement) = self.express_id_to_placement.borrow().get(&_express_id) {
            return *placement;
        }

        let line_type = self.loader.get_line_type(_express_id);
        let result = match line_type {
            schema::IFCPOINTBYDISTANCEEXPRESSION => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let mut distance_along = 0.0;
                if self.loader.get_token_type() != IfcTokenType::Empty {
                    self.loader.step_back();
                    distance_along = self.read_length_measure();
                }

                self.loader.move_to_argument_offset(_express_id, 2);
                let mut offset_lateral = 0.0;
                match self.loader.get_token_type() {
                    IfcTokenType::Label => {
                        offset_lateral = self.read_length_measure();
                    }
                    IfcTokenType::Real => {
                        self.loader.step_back();
                        offset_lateral = self.loader.get_double_argument();
                    }
                    _ => {}
                }

                self.loader.move_to_argument_offset(_express_id, 3);
                let mut offset_vertical = 0.0;
                match self.loader.get_token_type() {
                    IfcTokenType::Label => {
                        offset_vertical = self.read_length_measure();
                    }
                    IfcTokenType::Real => {
                        self.loader.step_back();
                        offset_vertical = self.loader.get_double_argument();
                    }
                    _ => {}
                }

                self.loader.move_to_argument_offset(_express_id, 4);
                let mut offset_longitudinal = 0.0;
                match self.loader.get_token_type() {
                    IfcTokenType::Label => {
                        offset_longitudinal = self.read_length_measure();
                    }
                    IfcTokenType::Real => {
                        self.loader.step_back();
                        offset_longitudinal = self.loader.get_double_argument();
                    }
                    _ => {}
                }

                self.loader.move_to_argument_offset(_express_id, 5);
                let mut curve = IfcCurve::default();
                let mut placement = DMat4::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    let curve_id = self.loader.get_ref_argument();
                    curve = self.get_local_curve(curve_id);
                    placement = curve.get_placement_at_distance(
                        distance_along,
                        crate::web_ifc::geometry::representation::ifc_curve::CurvePlacementMode::GlobalZAxis,
                    );
                } else {
                    placement = curve.get_placement_at_distance(
                        distance_along,
                        crate::web_ifc::geometry::representation::ifc_curve::CurvePlacementMode::GlobalZAxis,
                    );
                }

                if offset_lateral.abs()
                    > crate::web_ifc::geometry::representation::geometry::EPS_SMALL
                    || offset_vertical.abs()
                        > crate::web_ifc::geometry::representation::geometry::EPS_SMALL
                    || offset_longitudinal.abs()
                        > crate::web_ifc::geometry::representation::geometry::EPS_SMALL
                {
                    let local_translation = DMat4::from_translation(DVec3::new(
                        offset_longitudinal,
                        offset_lateral,
                        0.0,
                    ));
                    let global_vertical_translation =
                        DMat4::from_translation(DVec3::new(0.0, 0.0, offset_vertical));
                    placement = global_vertical_translation * (placement * local_translation);
                }
                placement
            }
            schema::IFCAXIS1PLACEMENT => {
                let mut z_axis = DVec3::new(0.0, 0.0, 1.0);
                let mut x_axis = DVec3::new(1.0, 0.0, 0.0);
                self.loader.move_to_argument_offset(_express_id, 0);
                let pos_id = self.loader.get_ref_argument();
                let z_id = self.loader.get_token_type();
                if z_id == IfcTokenType::Ref {
                    self.loader.step_back();
                    let tmp = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                    if tmp.length() > 0.0 {
                        z_axis = tmp;
                    }
                }
                let pos = self.get_cartesian_point_3d(pos_id);
                if x_axis.dot(z_axis).abs() > 0.9 {
                    x_axis = DVec3::new(0.0, 1.0, 0.0);
                }
                let y_axis = z_axis.cross(x_axis).normalize_or_zero();
                x_axis = y_axis.cross(z_axis).normalize_or_zero();

                DMat4::from_cols(
                    x_axis.extend(0.0),
                    y_axis.extend(0.0),
                    z_axis.extend(0.0),
                    pos.extend(1.0),
                )
            }
            schema::IFCAXIS2PLACEMENT3D => {
                let mut z_axis = DVec3::new(0.0, 0.0, 1.0);
                let mut x_axis = DVec3::new(1.0, 0.0, 0.0);

                self.loader.move_to_argument_offset(_express_id, 0);
                let pos_id = self.loader.get_ref_argument();
                let z_id = self.loader.get_token_type();
                if z_id == IfcTokenType::Ref {
                    self.loader.step_back();
                    let tmp = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                    if tmp.length() > 0.0 {
                        z_axis = tmp;
                    }
                }

                self.loader.move_to_argument_offset(_express_id, 2);
                let x_id = self.loader.get_token_type();
                if x_id == IfcTokenType::Ref {
                    self.loader.step_back();
                    let tmp = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                    if tmp.length() > 0.0 {
                        x_axis = tmp;
                    }
                }

                let pos = self.get_cartesian_point_3d(pos_id);
                let y_axis = z_axis.cross(x_axis).normalize_or_zero();
                x_axis = y_axis.cross(z_axis).normalize_or_zero();

                DMat4::from_cols(
                    x_axis.extend(0.0),
                    y_axis.extend(0.0),
                    z_axis.extend(0.0),
                    pos.extend(1.0),
                )
            }
            schema::IFCAXIS2PLACEMENT2D => {
                let mut x_axis = DVec3::new(1.0, 0.0, 0.0);
                let z_axis = DVec3::new(0.0, 0.0, 1.0);

                self.loader.move_to_argument_offset(_express_id, 0);
                let pos_id = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(_express_id, 1);
                let x_id = self.loader.get_token_type();
                if x_id == IfcTokenType::Ref {
                    self.loader.step_back();
                    let tmp = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                    if tmp.length() > 0.0 {
                        x_axis = tmp;
                    }
                }

                let pos = self.get_cartesian_point_3d(pos_id);
                let y_axis = z_axis.cross(x_axis).normalize_or_zero();
                x_axis = y_axis.cross(z_axis).normalize_or_zero();

                DMat4::from_cols(
                    x_axis.extend(0.0),
                    y_axis.extend(0.0),
                    z_axis.extend(0.0),
                    pos.extend(1.0),
                )
            }
            schema::IFCLOCALPLACEMENT => {
                let mut rel_placement = DMat4::IDENTITY;

                self.loader.move_to_argument_offset(_express_id, 0);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    rel_placement =
                        self.get_local_placement(self.loader.get_ref_argument(), DVec3::ONE);
                }

                self.loader.move_to_argument_offset(_express_id, 1);
                let axis2_placement_id = self.loader.get_ref_argument();
                let axis2_placement = self.get_local_placement(axis2_placement_id, DVec3::ONE);
                rel_placement * axis2_placement
            }
            schema::IFCCARTESIANTRANSFORMATIONOPERATOR3D
            | schema::IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM => {
                let mut scale1 = 1.0;
                let mut scale2 = 1.0;
                let mut scale3 = 1.0;

                let mut axis1 = DVec3::new(1.0, 0.0, 0.0);
                let mut axis2 = DVec3::new(0.0, 1.0, 0.0);
                let mut axis3 = DVec3::new(0.0, 0.0, 1.0);

                self.loader.move_to_argument_offset(_express_id, 0);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    axis1 = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                }
                self.loader.move_to_argument_offset(_express_id, 1);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    axis2 = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                }

                self.loader.move_to_argument_offset(_express_id, 2);
                let local_origin_id = self.loader.get_ref_argument();
                let local_origin = self.get_cartesian_point_3d(local_origin_id);

                self.loader.move_to_argument_offset(_express_id, 3);
                if self.loader.get_token_type() == IfcTokenType::Real {
                    self.loader.step_back();
                    scale1 = self.loader.get_double_argument();
                }

                self.loader.move_to_argument_offset(_express_id, 4);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    axis3 = self
                        .get_cartesian_point_3d(self.loader.get_ref_argument())
                        .normalize_or_zero();
                }

                if line_type == schema::IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM {
                    self.loader.move_to_argument_offset(_express_id, 5);
                    if self.loader.get_token_type() == IfcTokenType::Real {
                        self.loader.step_back();
                        scale2 = self.loader.get_double_argument();
                    }

                    self.loader.move_to_argument_offset(_express_id, 6);
                    if self.loader.get_token_type() == IfcTokenType::Real {
                        self.loader.step_back();
                        scale3 = self.loader.get_double_argument();
                    }
                }

                if line_type == schema::IFCCARTESIANTRANSFORMATIONOPERATOR3D {
                    scale2 = scale1;
                    scale3 = scale1;
                }

                DMat4::from_cols(
                    (axis1 * scale1).extend(0.0),
                    (axis2 * scale2).extend(0.0),
                    (axis3 * scale3).extend(0.0),
                    local_origin.extend(1.0),
                )
            }
            schema::IFCAXIS2PLACEMENTLINEAR => {
                let mut vector = DVec3::new(0.0, 0.0, 1.0);
                let mut placement = DMat4::IDENTITY;
                self.loader.move_to_argument_offset(_express_id, 0);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    let pos_id = self.loader.get_ref_argument();

                    self.loader.move_to_argument_offset(_express_id, 1);
                    if self.loader.get_token_type() == IfcTokenType::Ref {
                        self.loader.step_back();
                        vector = self.get_cartesian_point_3d(self.loader.get_ref_argument());
                    }
                    placement = self.get_local_placement(pos_id, vector);
                }
                placement
            }
            schema::IFCLINEARPLACEMENT => {
                self.loader.move_to_argument_offset(_express_id, 1);
                let pos_id = self.loader.get_ref_argument();
                self.get_local_placement(pos_id, DVec3::ONE)
            }
            _ => {
                eprintln!(
                    "[get_local_placement] unexpected placement type {}",
                    _express_id
                );
                DMat4::IDENTITY
            }
        };

        self.express_id_to_placement
            .borrow_mut()
            .insert(_express_id, result);
        result
    }

    // C++ default argument mapping: GetLocalPlacement(expressID).
    pub fn get_local_placement_default(&self, express_id: u32) -> DMat4 {
        self.get_local_placement(express_id, DVec3::ONE)
    }

    pub fn get_cartesian_point_3d(&self, _express_id: u32) -> DVec3 {
        if let Some(point) = self.cartesian_point_3d_cache.borrow().get(&_express_id) {
            return *point;
        }
        self.loader.move_to_argument_offset(_express_id, 0);
        self.loader.get_token_type();
        let x = self.loader.get_double_argument();
        let y = self.loader.get_double_argument();
        let z = self.loader.get_optional_double_param(0.0);
        let point = DVec3::new(x, y, z);
        self.cartesian_point_3d_cache
            .borrow_mut()
            .insert(_express_id, point);
        point
    }

    pub fn get_cartesian_point_2d(&self, _express_id: u32) -> DVec2 {
        if let Some(point) = self.cartesian_point_2d_cache.borrow().get(&_express_id) {
            return *point;
        }
        self.loader.move_to_argument_offset(_express_id, 0);
        self.loader.get_token_type();
        let x = self.loader.get_double_argument();
        let y = self.loader.get_double_argument();
        let point = DVec2::new(x, y);
        self.cartesian_point_2d_cache
            .borrow_mut()
            .insert(_express_id, point);
        point
    }

    pub fn get_vector(&self, _express_id: u32) -> DVec3 {
        self.loader.move_to_argument_offset(_express_id, 0);
        let position_id = self.loader.get_ref_argument();
        let length = self.loader.get_double_argument();

        let mut direction = self.get_cartesian_point_3d(position_id);
        direction.x *= length;
        direction.y *= length;
        direction.z *= length;

        direction
    }

    pub fn get_profile(&self, _express_id: u32) -> IfcProfile {
        let mut profile = self.get_profile_by_line(_express_id);

        if !profile.is_composite {
            if !profile.curve.base.is_ccw() {
                profile.curve.base.invert();
            }
            for hole in &mut profile.holes {
                if hole.base.is_ccw() {
                    hole.base.invert();
                }
            }
        } else {
            for sub_profile in &mut profile.profiles {
                if !sub_profile.curve.base.is_ccw() {
                    sub_profile.curve.base.invert();
                }
                for hole in &mut sub_profile.holes {
                    if hole.base.is_ccw() {
                        hole.base.invert();
                    }
                }
            }
        }

        profile
    }

    pub fn get_profile_3d(&self, _express_id: u32) -> IfcProfile {
        let line_type = self.loader.get_line_type(_express_id);
        match line_type {
            schema::IFCARBITRARYOPENPROFILEDEF => {
                let mut profile = IfcProfile {
                    r#type: String::new(),
                    curve: IfcCurve::default(),
                    holes: Vec::new(),
                    is_convex: false,
                    is_composite: false,
                    profiles: Vec::new(),
                    tags: Vec::new(),
                };
                self.loader.move_to_argument_offset(_express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                self.loader.move_to_argument_offset(_express_id, 2);
                profile.curve = self.get_curve(self.loader.get_ref_argument(), 3, false);
                profile
            }
            _ => {
                eprintln!(
                    "[get_profile_3d] unexpected 3D profile type {}",
                    _express_id
                );
                IfcProfile {
                    r#type: String::new(),
                    curve: IfcCurve::default(),
                    holes: Vec::new(),
                    is_convex: false,
                    is_composite: false,
                    profiles: Vec::new(),
                    tags: Vec::new(),
                }
            }
        }
    }

    pub fn get_local_curve(&self, _express_id: u32) -> IfcCurve {
        for (index, stored_id) in self.local_curves_indices.borrow().iter().enumerate() {
            if *stored_id == _express_id {
                return self.local_curves_list.borrow()[index].clone();
            }
        }
        let curve = self.get_curve(_express_id, 3, false);
        self.local_curves_indices.borrow_mut().push(_express_id);
        self.local_curves_list.borrow_mut().push(curve.clone());
        curve
    }

    pub fn get_curve(&self, _express_id: u32, _dimensions: u8, _edge: bool) -> IfcCurve {
        let mut curve = IfcCurve::default();
        let params = ComputeCurveParams {
            dimensions: _dimensions,
            ignore_placement: false,
            edge: _edge,
            same_sense: -1,
            has_trim: false,
            trim_start: IfcTrimmingSelect::default(),
            trim_end: IfcTrimmingSelect::default(),
            trim_sense: TrimSense::Same,
        };
        self.compute_curve(_express_id, &mut curve, &params);
        curve
    }

    // C++ default argument mapping: GetCurve(expressID, dimensions, edge=false).
    pub fn get_curve_default(&self, express_id: u32, dimensions: u8) -> IfcCurve {
        self.get_curve(express_id, dimensions, false)
    }

    pub fn compute_curve_length(&self, _curve: &IfcCurve) -> f64 {
        let mut total_length = 0.0;
        if _curve.base.points.len() < 2 {
            return total_length;
        }
        for i in 1.._curve.base.points.len() {
            let p1 = _curve.base.points[i - 1];
            let p2 = _curve.base.points[i];
            total_length += p1.distance(p2);
        }
        total_length
    }

    pub fn compute_length_to_point(&self, _curve: &IfcCurve, _target_point: &DVec3) -> f64 {
        let mut length = 0.0;
        if _curve.base.points.len() < 2 {
            return length;
        }
        for i in 1.._curve.base.points.len() {
            let p1 = _curve.base.points[i - 1];
            let p2 = _curve.base.points[i];
            if _target_point.distance(p1) < 1e-6 {
                return length;
            }
            if _target_point.distance(p2) < 1e-6 {
                return length + p1.distance(p2);
            }
            length += p1.distance(p2);
        }
        length
    }

    pub fn get_parameter_for_point(
        &self,
        _curve: &IfcCurve,
        _total_length: f64,
        _point: &DVec3,
    ) -> f64 {
        if _curve.base.points.is_empty() {
            return 0.0;
        }
        let length_to_point = self.compute_length_to_point(_curve, _point);
        if _total_length > 0.0 {
            length_to_point / _total_length
        } else {
            0.0
        }
    }

    pub fn read_ifc_cartesian_point_list(&self, _express_id: u32) -> bool {
        let list_type = self.loader.get_line_type(_express_id);
        self.loader.move_to_argument_offset(_express_id, 0);
        if list_type == schema::IFCCARTESIANPOINTLIST3D {
            return false;
        }
        if list_type == schema::IFCCARTESIANPOINTLIST2D {
            return true;
        }
        false
    }

    pub fn read_ifc_cartesian_point_list_3d(&self, _express_id: u32) -> Vec<DVec3> {
        self.loader.move_to_argument_offset(_express_id, 0);
        let mut result = Vec::new();
        self.loader.get_token_type();

        while self.loader.get_token_type() == IfcTokenType::SetBegin {
            let x = self.loader.get_double_argument();
            let y = self.loader.get_double_argument();
            let z = self.loader.get_double_argument();
            result.push(DVec3::new(x, y, z));
            self.loader.get_token_type();
        }
        result
    }

    pub fn read_ifc_cartesian_point_list_2d(&self, _express_id: u32) -> Vec<DVec2> {
        self.loader.move_to_argument_offset(_express_id, 0);
        let mut result = Vec::new();
        self.loader.get_token_type();
        while self.loader.get_token_type() == IfcTokenType::SetBegin {
            let x = self.loader.get_double_argument();
            let y = self.loader.get_double_argument();
            result.push(DVec2::new(x, y));
            self.loader.get_token_type();
        }
        result
    }

    pub fn get_oriented_edge(&self, _express_id: u32) -> IfcCurve {
        self.loader.move_to_argument_offset(_express_id, 3);
        let orient_value = self.loader.get_string_argument();
        let orient = orient_value == "T";
        self.loader.move_to_argument_offset(_express_id, 2);
        let edge_curve_ref = self.loader.get_ref_argument();
        let mut curve_edge = self.get_edge(edge_curve_ref);

        if orient {
            curve_edge.base.points.reverse();
        }

        curve_edge
    }

    pub fn get_edge(&self, _express_id: u32) -> IfcCurve {
        let line_type = self.loader.get_line_type(_express_id);

        match line_type {
            schema::IFCEDGE => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let p1 = self.get_vertex_point(self.loader.get_ref_argument());
                self.loader.move_to_argument_offset(_express_id, 1);
                let p2 = self.get_vertex_point(self.loader.get_ref_argument());

                let mut curve = IfcCurve::default();
                curve.base.points.push(p1);
                curve.base.points.push(p2);
                curve
            }
            schema::IFCEDGECURVE => {
                let mut edge_params = ComputeCurveParams::default();
                edge_params.has_trim = true;
                self.loader.move_to_argument_offset(_express_id, 0);
                let p1 = self.get_vertex_point(self.loader.get_ref_argument());
                self.loader.move_to_argument_offset(_express_id, 1);
                let p2 = self.get_vertex_point(self.loader.get_ref_argument());
                edge_params.trim_start.pos3d = p1;
                edge_params.trim_start.trim_type =
                    crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition;
                edge_params.trim_end.pos3d = p2;
                edge_params.trim_end.trim_type =
                    crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition;

                self.loader.move_to_argument_offset(_express_id, 2);
                let curve_ref = self.loader.get_ref_argument();
                let mut curve = IfcCurve::default();

                edge_params.dimensions = 3;
                edge_params.edge = true;
                edge_params.same_sense = -1;
                edge_params.trim_sense = TrimSense::Same;
                self.compute_curve(curve_ref, &mut curve, &edge_params);

                curve
            }
            _ => {
                eprintln!("[get_edge] unexpected edgecurve type {}", _express_id);
                IfcCurve::default()
            }
        }
    }

    pub fn get_bound(&self, _express_id: u32) -> IfcBound3D {
        let line_type = self.loader.get_line_type(_express_id);
        match line_type {
            schema::IFCFACEOUTERBOUND => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let loop_id = self.loader.get_ref_argument();
                self.loader.move_to_argument_offset(_express_id, 1);
                let orient_value = self.loader.get_string_argument();
                let orient = orient_value == "T";

                let mut curve = self.get_loop(loop_id);
                if !orient {
                    curve.base.points.reverse();
                }

                IfcBound3D {
                    bound_type:
                        crate::web_ifc::geometry::representation::geometry::IfcBoundType::OuterBound,
                    orientation: orient,
                    curve,
                }
            }
            schema::IFCFACEBOUND => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let loop_id = self.loader.get_ref_argument();
                self.loader.move_to_argument_offset(_express_id, 1);
                let orient_value = self.loader.get_string_argument();
                let orient = orient_value == "T";

                let mut curve = self.get_loop(loop_id);
                if !orient {
                    curve.base.points.reverse();
                }

                IfcBound3D {
                    bound_type:
                        crate::web_ifc::geometry::representation::geometry::IfcBoundType::Bound,
                    orientation: orient,
                    curve,
                }
            }
            _ => {
                eprintln!("[get_bound] unexpected bound type {}", _express_id);
                IfcBound3D {
                    bound_type:
                        crate::web_ifc::geometry::representation::geometry::IfcBoundType::Bound,
                    orientation: true,
                    curve: IfcCurve::default(),
                }
            }
        }
    }

    pub fn get_loop(&self, _express_id: u32) -> IfcCurve {
        let line_type = self.loader.get_line_type(_express_id);
        match line_type {
            schema::IFCPOLYLOOP => {
                let mut curve = IfcCurve::default();

                self.loader.move_to_argument_offset(_express_id, 0);
                let points = self.loader.get_set_argument();

                curve.base.points.reserve(points.len());

                let mut prev_id = 0;
                for token in points {
                    let point_id = self.loader.get_ref_argument_at(token);
                    if point_id != prev_id {
                        curve
                            .base
                            .points
                            .push(self.get_cartesian_point_3d(point_id));
                    }
                    prev_id = point_id;
                }

                curve
            }
            schema::IFCEDGELOOP => {
                let mut curve = IfcCurve::default();
                self.loader.move_to_argument_offset(_express_id, 0);
                let edges = self.loader.get_set_argument();
                let mut id = 0u16;

                for token in edges {
                    let edge_id = self.loader.get_ref_argument_at(token);
                    let edge_curve = self.get_oriented_edge(edge_id);
                    if curve.base.points.is_empty() {
                        for pt in edge_curve.base.points {
                            curve.base.points.push(pt);
                            curve.indices.push(id);
                        }
                    } else {
                        for pt in edge_curve.base.points {
                            if self.not_present(pt, &curve.base.points) {
                                curve.base.points.push(pt);
                                curve.indices.push(id);
                            }
                        }
                    }
                    id += 1;
                }
                curve
            }
            _ => {
                eprintln!("[get_loop] unexpected loop type {}", _express_id);
                IfcCurve::default()
            }
        }
    }

    pub fn get_color(&self, _express_id: u32) -> Option<DVec4> {
        let line_type = self.loader.get_line_type(_express_id);
        match line_type {
            schema::IFCPRESENTATIONSTYLEASSIGNMENT => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let style_selects = self.loader.get_set_argument();
                for style_select in style_selects {
                    let style_id = self.loader.get_ref_argument_at(style_select);
                    if let Some(color) = self.get_color(style_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCDRAUGHTINGPREDEFINEDCOLOUR => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let color = self.loader.get_string_argument();
                match color {
                    "black" => Some(DVec4::new(0.0, 0.0, 0.0, 1.0)),
                    "red" => Some(DVec4::new(1.0, 0.0, 0.0, 1.0)),
                    "green" => Some(DVec4::new(0.0, 1.0, 0.0, 1.0)),
                    "blue" => Some(DVec4::new(0.0, 0.0, 1.0, 1.0)),
                    "yellow" => Some(DVec4::new(1.0, 1.0, 0.0, 1.0)),
                    "magenta" => Some(DVec4::new(1.0, 0.0, 1.0, 1.0)),
                    "cyan" => Some(DVec4::new(0.0, 1.0, 1.0, 1.0)),
                    "white" => Some(DVec4::new(1.0, 1.0, 1.0, 1.0)),
                    _ => None,
                }
            }
            schema::IFCCURVESTYLE => {
                self.loader.move_to_argument_offset(_express_id, 3);
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    return self.get_color(self.loader.get_ref_argument());
                }
                None
            }
            schema::IFCFILLAREASTYLEHATCHING => {
                self.loader.move_to_argument_offset(_express_id, 0);
                self.get_color(self.loader.get_ref_argument())
            }
            schema::IFCSURFACESTYLE => {
                self.loader.move_to_argument_offset(_express_id, 2);
                let style_selects = self.loader.get_set_argument();
                for style_select in style_selects {
                    let style_id = self.loader.get_ref_argument_at(style_select);
                    if let Some(color) = self.get_color(style_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCSURFACESTYLERENDERING => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let mut output_color = self.get_color(self.loader.get_ref_argument())?;
                self.loader.move_to_argument_offset(_express_id, 1);
                if self.loader.get_token_type() == IfcTokenType::Real {
                    self.loader.step_back();
                    output_color.w = 1.0 - self.loader.get_double_argument();
                }
                Some(output_color)
            }
            schema::IFCSURFACESTYLESHADING => {
                self.loader.move_to_argument_offset(_express_id, 0);
                self.get_color(self.loader.get_ref_argument())
            }
            schema::IFCSTYLEDREPRESENTATION => {
                self.loader.move_to_argument_offset(_express_id, 3);
                let rep_items = self.loader.get_set_argument();
                for rep_item in rep_items {
                    let rep_item_id = self.loader.get_ref_argument_at(rep_item);
                    if let Some(color) = self.get_color(rep_item_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCSTYLEDITEM => {
                self.loader.move_to_argument_offset(_express_id, 1);
                let styled_items = self.loader.get_set_argument();
                for styled_item in styled_items {
                    let styled_item_id = self.loader.get_ref_argument_at(styled_item);
                    if let Some(color) = self.get_color(styled_item_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCCOLOURRGB => {
                self.loader.move_to_argument_offset(_express_id, 1);
                let r = self.loader.get_double_argument();
                let g = self.loader.get_double_argument();
                let b = self.loader.get_double_argument();
                Some(DVec4::new(r, g, b, 1.0))
            }
            schema::IFCMATERIALLAYERSETUSAGE => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let layer_set_id = self.loader.get_ref_argument();
                self.get_color(layer_set_id)
            }
            schema::IFCMATERIALLAYERSET => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let layers = self.loader.get_set_argument();
                for layer in layers {
                    let layer_id = self.loader.get_ref_argument_at(layer);
                    if let Some(color) = self.get_color(layer_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCMATERIALLAYER => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let mat_rep_id = self.loader.get_ref_argument();
                self.get_color(mat_rep_id)
            }
            schema::IFCMATERIAL => {
                if let Some(defs) = self.material_definitions.get(&_express_id) {
                    for def in defs {
                        if let Some(color) = self.get_color(def.1) {
                            return Some(color);
                        }
                    }
                }
                None
            }
            schema::IFCFILLAREASTYLE => {
                self.loader.move_to_argument_offset(_express_id, 1);
                let style_selects = self.loader.get_set_argument();
                for style_select in style_selects {
                    let style_id = self.loader.get_ref_argument_at(style_select);
                    if let Some(color) = self.get_color(style_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCMATERIALLIST => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let materials = self.loader.get_set_argument();
                let mut last_color = None;
                let mut result = false;
                for material in materials {
                    let material_id = self.loader.get_ref_argument_at(material);
                    if let Some(color) = self.get_color(material_id) {
                        last_color = Some(color);
                    }
                    result = true;
                }
                if result {
                    last_color
                } else {
                    None
                }
            }
            schema::IFCMATERIALCONSTITUENTSET => {
                self.loader.move_to_argument_offset(_express_id, 2);
                let constituents = self.loader.get_set_argument();
                for constituent in constituents {
                    let constituent_id = self.loader.get_ref_argument_at(constituent);
                    if let Some(color) = self.get_color(constituent_id) {
                        return Some(color);
                    }
                }
                None
            }
            schema::IFCMATERIALCONSTITUENT => {
                self.loader.move_to_argument_offset(_express_id, 2);
                let material_id = self.loader.get_ref_argument();
                self.get_color(material_id)
            }
            schema::IFCMATERIALPROFILESETUSAGE => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let profile_set = self.loader.get_ref_argument();
                self.get_color(profile_set)
            }
            schema::IFCMATERIALPROFILE => {
                self.loader.move_to_argument_offset(_express_id, 2);
                let profile_set = self.loader.get_ref_argument();
                self.get_color(profile_set)
            }
            schema::IFCMATERIALPROFILESET => {
                self.loader.move_to_argument_offset(_express_id, 2);
                let material_profiles = self.loader.get_set_argument();
                for material_profile in material_profiles {
                    let material_profile_id = self.loader.get_ref_argument_at(material_profile);
                    if let Some(color) = self.get_color(material_profile_id) {
                        return Some(color);
                    }
                }
                None
            }
            _ => {
                eprintln!("[get_color] unexpected style type {}", _express_id);
                None
            }
        }
    }

    pub fn get_cross_sections_2d(&self, _express_id: u32) -> IfcCrossSections {
        let line_type = self.loader.get_line_type(_express_id);
        let mut sections = IfcCrossSections::default();
        match line_type {
            schema::IFCSECTIONEDSOLIDHORIZONTAL | schema::IFCSECTIONEDSOLID => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let curve_id = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(_express_id, 1);
                let faces = self.loader.get_set_argument();

                self.loader.move_to_argument_offset(_express_id, 2);
                let _linear_positions = self.loader.get_set_argument();

                let _curve = self.get_curve(curve_id, 3, false);

                let mut curves = Vec::new();
                let mut express_ids = Vec::new();

                for face in faces {
                    let express_id = self.loader.get_ref_argument_at(face);
                    let profile = self.get_profile(express_id);
                    curves.push(profile.curve);
                    express_ids.push(express_id);
                }

                sections.curves = curves;
                sections.express_id = express_ids;
                sections
            }
            schema::IFCSECTIONEDSURFACE => {
                self.loader.move_to_argument_offset(_express_id, 1);
                let _linear_positions = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 2);
                let faces = self.loader.get_set_argument();

                let mut curves = Vec::new();
                let mut express_ids = Vec::new();

                for face in faces {
                    let express_id = self.loader.get_ref_argument_at(face);
                    let profile = self.get_profile(express_id);
                    curves.push(profile.curve);
                    express_ids.push(express_id);
                }

                sections.curves = curves;
                sections.express_id = express_ids;
                sections
            }
            _ => sections,
        }
    }

    pub fn get_cross_sections_3d(
        &self,
        _express_id: u32,
        _scaled: bool,
        _coordination: DMat4,
    ) -> IfcCrossSections {
        let line_type = self.loader.get_line_type(_express_id);
        let mut sections = IfcCrossSections::default();
        let mut scale = 1.0;
        let mut coordination = _coordination;
        if _scaled {
            scale = self.linear_scaling_factor;
            let mut ps = coordination.w_axis;
            let y = ps.y;
            let z = -ps.z;
            ps.y = z;
            ps.z = y;
            coordination.w_axis = ps;
        }

        match line_type {
            schema::IFCSECTIONEDSOLIDHORIZONTAL => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let directrix_id = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(_express_id, 1);
                let cross_sections_offsets = self.loader.get_set_argument();

                self.loader.move_to_argument_offset(_express_id, 2);
                let cross_section_position_offsets = self.loader.get_set_argument();

                let _directrix = self.get_curve(directrix_id, 3, false);

                let mut curves = Vec::new();
                let mut cross_section_ids = Vec::new();

                let mut map_cross_section_positions: HashMap<f64, DMat4> = HashMap::new();
                let mut map_curve_to_distance: HashMap<u32, HashSet<f64>> = HashMap::new();
                let mut _fallback_positions: Vec<DMat4> = Vec::new();

                for offset in cross_section_position_offsets {
                    let cross_section_position_id = self.loader.get_ref_argument_at(offset);
                    let linear_placement_type =
                        self.loader.get_line_type(cross_section_position_id);
                    if linear_placement_type != schema::IFCAXIS2PLACEMENTLINEAR {
                        eprintln!(
                            "[IFCSECTIONEDSOLIDHORIZONTAL] unexpected Location type {}",
                            cross_section_position_id
                        );
                        continue;
                    }

                    let mut axis = DVec3::new(0.0, 0.0, 1.0);
                    self.loader
                        .move_to_argument_offset(cross_section_position_id, 0);
                    if self.loader.get_token_type() == IfcTokenType::Ref {
                        self.loader.step_back();
                        let location_id = self.loader.get_ref_argument();

                        self.loader
                            .move_to_argument_offset(cross_section_position_id, 1);
                        if self.loader.get_token_type() == IfcTokenType::Ref {
                            self.loader.step_back();
                            axis = self.get_cartesian_point_3d(self.loader.get_ref_argument());
                        }

                        let location_type = self.loader.get_line_type(location_id);
                        if location_type == schema::IFCPOINTBYDISTANCEEXPRESSION {
                            self.loader.move_to_argument_offset(location_id, 0);
                            if self.loader.get_token_type() != IfcTokenType::Label {
                                eprintln!(
                                    "[IFCSECTIONEDSOLIDHORIZONTAL] unexpected DistanceAlong type {}",
                                    _express_id
                                );
                                continue;
                            }

                            self.loader.step_back();
                            let distance_label = self.loader.get_string_argument();
                            if distance_label != "IFCLENGTHMEASURE" {
                                eprintln!(
                                    "[IFCSECTIONEDSOLIDHORIZONTAL] unexpected DistanceAlong type {}",
                                    _express_id
                                );
                                continue;
                            }
                            self.loader.get_token_type();
                            let distance_along = self.loader.get_double_argument();

                            self.loader.move_to_argument_offset(location_id, 5);
                            if self.loader.get_token_type() == IfcTokenType::Ref {
                                self.loader.step_back();
                                let basis_curve_id = self.loader.get_ref_argument();
                                map_curve_to_distance
                                    .entry(basis_curve_id)
                                    .or_default()
                                    .insert(distance_along);
                                map_cross_section_positions.insert(distance_along, DMat4::IDENTITY);
                            }
                        } else {
                            let mut linear_placement = self.get_local_placement(location_id, axis);
                            linear_placement *= scale;
                            _fallback_positions.push(linear_placement);
                        }
                    }
                }

                if map_cross_section_positions.is_empty() || map_curve_to_distance.is_empty() {
                    eprintln!("[IFCSECTIONEDSOLIDHORIZONTAL] no valid CrossSectionPositions");
                    return sections;
                }

                if cross_sections_offsets.is_empty() {
                    eprintln!("[IFCSECTIONEDSOLIDHORIZONTAL] no valid CrossSections");
                    return sections;
                }

                let distances_with_cross_section = map_curve_to_distance
                    .get(&directrix_id)
                    .cloned()
                    .unwrap_or_else(|| {
                        map_curve_to_distance
                            .values()
                            .next()
                            .cloned()
                            .unwrap_or_default()
                    });

                self.get_placements_on_curve_points(directrix_id, &mut map_cross_section_positions);
                let ordered_positions: BTreeMap<f64, DMat4> =
                    map_cross_section_positions.into_iter().collect();

                let mut cross_section_index = 0usize;
                let mut current_cross_section_id = self
                    .loader
                    .get_ref_argument_at(cross_sections_offsets[cross_section_index]);

                for (distance, placement) in ordered_positions {
                    let mut current_profile = self.get_profile(current_cross_section_id);
                    for point in &mut current_profile.curve.base.points {
                        let p_temp = placement * point.extend(1.0);
                        let coord = coordination * p_temp;
                        *point = coord.truncate();
                    }

                    curves.push(current_profile.curve);
                    cross_section_ids.push(current_cross_section_id);

                    if distances_with_cross_section.contains(&distance) {
                        cross_section_index += 1;
                        if cross_section_index >= cross_sections_offsets.len() {
                            cross_section_index = cross_sections_offsets.len() - 1;
                        }
                        current_cross_section_id = self
                            .loader
                            .get_ref_argument_at(cross_sections_offsets[cross_section_index]);
                    }
                }

                sections.curves = curves;
                sections.express_id = cross_section_ids;
                sections
            }
            schema::IFCSECTIONEDSOLID => {
                self.loader.move_to_argument_offset(_express_id, 0);
                let curve_id = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(_express_id, 1);
                let faces = self.loader.get_set_argument();

                self.loader.move_to_argument_offset(_express_id, 2);
                let linear_positions = self.loader.get_set_argument();

                let _curve = self.get_curve(curve_id, 3, false);

                let mut transform = Vec::new();
                for linear_position in linear_positions {
                    let express_id = self.loader.get_ref_argument_at(linear_position);
                    let linear_placement = self.get_local_placement(express_id, DVec3::ONE) * scale;
                    transform.push(linear_placement);
                }

                let mut curves = Vec::new();
                let mut express_ids = Vec::new();
                let mut id = 0usize;

                for face in faces {
                    let express_id = self.loader.get_ref_argument_at(face);
                    let mut profile = self.get_profile(express_id);
                    for point in &mut profile.curve.base.points {
                        let p_temp = transform[id] * point.extend(1.0);
                        let coord = coordination * p_temp;
                        *point = coord.truncate();
                    }
                    curves.push(profile.curve);
                    express_ids.push(express_id);
                    id += 1;
                }

                sections.curves = curves;
                sections.express_id = express_ids;
                sections
            }
            schema::IFCSECTIONEDSURFACE => {
                self.loader.move_to_argument_offset(_express_id, 1);
                let linear_positions = self.loader.get_set_argument();
                self.loader.move_to_argument_offset(_express_id, 2);
                let faces = self.loader.get_set_argument();

                let mut transform = Vec::new();
                for linear_position in linear_positions {
                    let express_id = self.loader.get_ref_argument_at(linear_position);
                    let linear_placement = self.get_local_placement(express_id, DVec3::ONE) * scale;
                    transform.push(linear_placement);
                }

                let mut curves = Vec::new();
                let mut express_ids = Vec::new();
                let mut id = 0usize;
                for face in faces {
                    let express_id = self.loader.get_ref_argument_at(face);
                    let mut profile = self.get_profile(express_id);
                    for point in &mut profile.curve.base.points {
                        let p_temp = transform[id] * point.extend(1.0);
                        let coord = coordination * p_temp;
                        *point = coord.truncate();
                    }
                    curves.push(profile.curve);
                    express_ids.push(express_id);
                    id += 1;
                }

                sections.curves = curves;
                sections.express_id = express_ids;
                sections
            }
            _ => sections,
        }
    }

    // C++ default argument mapping: GetCrossSections3D(expressID, scaled=false, coordination=identity).
    pub fn get_cross_sections_3d_default(&self, express_id: u32) -> IfcCrossSections {
        self.get_cross_sections_3d(express_id, false, DMat4::IDENTITY)
    }

    pub fn get_placements_on_curve_points(
        &self,
        _curve_id: u32,
        _placements: &mut HashMap<f64, DMat4>,
    ) {
        let mut sorted: BTreeMap<f64, DMat4> = _placements.iter().map(|(k, v)| (*k, *v)).collect();
        if sorted.len() < 2 {
            return;
        }
        let min_distance = *sorted.keys().next().unwrap();
        let max_distance = *sorted.keys().next_back().unwrap();

        let basis_curve = self.get_local_curve(_curve_id);
        if basis_curve.base.points.is_empty() {
            eprintln!(
                "[get_placements_on_curve_points] BasisCurve has no points {}",
                _curve_id
            );
            return;
        }

        for (distance, placement) in sorted.iter_mut() {
            *placement = basis_curve.get_placement_at_distance(
                *distance,
                crate::web_ifc::geometry::representation::ifc_curve::CurvePlacementMode::TangentAsZAxis,
            );
        }

        if min_distance <= 0.0 {
            let result = basis_curve.get_placement_at_distance(
                0.0,
                crate::web_ifc::geometry::representation::ifc_curve::CurvePlacementMode::TangentAsZAxis,
            );
            sorted.insert(0.0, result);
        }

        let mut sum_length = 0.0;
        let mut previous_point = basis_curve.base.points[0];
        for point in basis_curve.base.points.iter().skip(1) {
            let length = point.distance(previous_point);
            sum_length += length;
            if sum_length >= min_distance {
                let result = basis_curve.get_placement_at_distance(
                    sum_length,
                    crate::web_ifc::geometry::representation::ifc_curve::CurvePlacementMode::TangentAsZAxis,
                );
                sorted.insert(sum_length, result);
            }
            if sum_length > max_distance {
                break;
            }
            previous_point = *point;
        }

        if max_distance >= sum_length {
            let result = basis_curve.get_placement_at_distance(
                sum_length,
                crate::web_ifc::geometry::representation::ifc_curve::CurvePlacementMode::TangentAsZAxis,
            );
            sorted.insert(sum_length, result);
        }

        _placements.clear();
        _placements.extend(sorted.into_iter());
    }

    pub fn get_alignment(
        &self,
        _express_id: u32,
        _alignment: IfcAlignment,
        _transform: DMat4,
        _source_express_id: u32,
    ) -> IfcAlignment {
        let line_type = self.loader.get_line_type(_express_id);
        let mut alignment = _alignment;

        match line_type {
            schema::IFCALIGNMENT => {
                self.loader.move_to_argument_offset(_express_id, 5);
                let mut local_placement = 0;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    local_placement = self.loader.get_ref_argument();
                }

                let mut transform_t = DMat4::IDENTITY;
                if local_placement != 0 && self.loader.is_valid_express_id(local_placement) {
                    transform_t = self.get_local_placement(local_placement, DVec3::ONE);
                    alignment.placement_express_id = local_placement;
                }

                if let Some(rel_agg) = self.rel_aggregates.get(&_express_id) {
                    for express_id in rel_agg {
                        alignment = self.get_alignment(
                            *express_id,
                            alignment,
                            _transform * transform_t,
                            *express_id,
                        );
                    }
                }

                if let Some(rel_nest) = self.rel_nests.get(&_express_id) {
                    for express_id in rel_nest {
                        alignment = self.get_alignment(
                            *express_id,
                            alignment,
                            _transform * transform_t,
                            *express_id,
                        );
                    }
                }
            }
            schema::IFCALIGNMENTHORIZONTAL => {
                self.loader.move_to_argument_offset(_express_id, 5);
                let mut local_placement = 0;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    local_placement = self.loader.get_ref_argument();
                }

                let mut transform_t = DMat4::IDENTITY;
                if local_placement != 0 && self.loader.is_valid_express_id(local_placement) {
                    transform_t = self.get_local_placement(local_placement, DVec3::ONE);
                }

                if let Some(rel_agg) = self.rel_aggregates.get(&_express_id) {
                    for express_id in rel_agg {
                        alignment
                            .horizontal
                            .curves
                            .push(self.get_alignment_curve(*express_id, _source_express_id));
                    }

                    for curve in &mut alignment.horizontal.curves {
                        for point in &mut curve.base.points {
                            let transformed = (_transform * transform_t) * point.extend(1.0);
                            *point = transformed.truncate();
                        }
                    }
                }

                if let Some(rel_nest) = self.rel_nests.get(&_express_id) {
                    for express_id in rel_nest {
                        alignment
                            .horizontal
                            .curves
                            .push(self.get_alignment_curve(*express_id, _source_express_id));
                    }

                    for curve in &mut alignment.horizontal.curves {
                        for point in &mut curve.base.points {
                            let transformed = (_transform * transform_t) * point.extend(1.0);
                            *point = transformed.truncate();
                        }
                    }
                }
            }
            schema::IFCALIGNMENTVERTICAL => {
                self.loader.move_to_argument_offset(_express_id, 5);
                let mut local_placement = 0;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    local_placement = self.loader.get_ref_argument();
                }

                let mut transform_t = DMat4::IDENTITY;
                if local_placement != 0 && self.loader.is_valid_express_id(local_placement) {
                    transform_t = self.get_local_placement(local_placement, DVec3::ONE);
                }

                if let Some(rel_agg) = self.rel_aggregates.get(&_express_id) {
                    for express_id in rel_agg {
                        alignment
                            .vertical
                            .curves
                            .push(self.get_alignment_curve(*express_id, _source_express_id));
                    }

                    for curve in &mut alignment.vertical.curves {
                        for point in &mut curve.base.points {
                            let transformed = (_transform * transform_t) * point.extend(1.0);
                            *point = transformed.truncate();
                        }
                    }
                }

                if let Some(rel_nest) = self.rel_nests.get(&_express_id) {
                    for express_id in rel_nest {
                        alignment
                            .vertical
                            .curves
                            .push(self.get_alignment_curve(*express_id, _source_express_id));
                    }

                    for curve in &mut alignment.vertical.curves {
                        for point in &mut curve.base.points {
                            let transformed = (_transform * transform_t) * point.extend(1.0);
                            *point = transformed.truncate();
                        }
                    }
                }
            }
            _ => {}
        }

        alignment
    }

    // C++ default argument mapping: GetAlignment(expressID, alignment=default, transform=identity, sourceExpressID=-1).
    pub fn get_alignment_default(&self, express_id: u32) -> IfcAlignment {
        self.get_alignment(
            express_id,
            IfcAlignment::default(),
            DMat4::IDENTITY,
            u32::MAX,
        )
    }

    // C++ mapping: GetColor(expressID, outputColor).
    pub fn get_color_into(&self, _express_id: u32, _output_color: &mut DVec4) -> bool {
        if let Some(color) = self.get_color(_express_id) {
            *_output_color = color;
            return true;
        }
        false
    }

    pub fn get_rel_voids(&self) -> &HashMap<u32, Vec<u32>> {
        &self.rel_voids
    }

    pub fn get_styled_items(&self) -> &HashMap<u32, Vec<(u32, u32)>> {
        &self.styled_items
    }

    pub fn get_rel_materials(&self) -> &HashMap<u32, Vec<(u32, u32)>> {
        &self.rel_materials
    }

    pub fn get_material_definitions(&self) -> &HashMap<u32, Vec<(u32, u32)>> {
        &self.material_definitions
    }

    pub fn get_linear_scaling_factor(&self) -> f64 {
        self.linear_scaling_factor
    }

    pub fn get_angle_units(&self) -> &str {
        &self.angle_units
    }

    pub fn clear(&self) {
        self.express_id_to_placement.borrow_mut().clear();
        self.cartesian_point_3d_cache.borrow_mut().clear();
        self.cartesian_point_2d_cache.borrow_mut().clear();
    }

    fn populate_rel_voids_map(&self) -> HashMap<u32, Vec<u32>> {
        let mut result = HashMap::new();
        let rel_voids = self
            .loader
            .get_express_ids_with_type(schema::IFCRELVOIDSELEMENT);

        for rel_void_id in rel_voids {
            self.loader.move_to_argument_offset(rel_void_id, 4);
            let relating_building_element = self.loader.get_ref_argument();
            let related_opening_element = self.loader.get_ref_argument();
            result
                .entry(relating_building_element)
                .or_default()
                .push(related_opening_element);
        }

        result
    }

    fn populate_rel_aggregates_map(&mut self) -> HashMap<u32, Vec<u32>> {
        let mut result = HashMap::new();
        let rel_aggregates = self
            .loader
            .get_express_ids_with_type(schema::IFCRELAGGREGATES);

        for rel_aggregate_id in rel_aggregates {
            self.loader.move_to_argument_offset(rel_aggregate_id, 4);
            let relating_building_element = self.loader.get_ref_argument();
            let aggregates = self.loader.get_set_argument();
            let rel_voids_for_relating = self.rel_voids.get(&relating_building_element).cloned();

            for aggregate in aggregates {
                let aggregate_id = self.loader.get_ref_argument_at(aggregate);
                result
                    .entry(relating_building_element)
                    .or_default()
                    .push(aggregate_id);
                if let Some(voids) = &rel_voids_for_relating {
                    let rel_voids_entry = self.rel_voids.entry(aggregate_id).or_default();
                    rel_voids_entry.extend(voids.iter().copied());
                }
            }
        }

        result
    }

    fn populate_rel_nests_map(&self) -> HashMap<u32, Vec<u32>> {
        let mut result = HashMap::new();
        let rel_nests = self.loader.get_express_ids_with_type(schema::IFCRELNESTS);

        for rel_nest_id in rel_nests {
            self.loader.move_to_argument_offset(rel_nest_id, 4);
            let relating_building_element = self.loader.get_ref_argument();
            let nests = self.loader.get_set_argument();
            for nest in nests {
                let nest_id = self.loader.get_ref_argument_at(nest);
                result
                    .entry(relating_building_element)
                    .or_default()
                    .push(nest_id);
            }
        }

        result
    }

    fn populate_styled_item_map(&self) -> HashMap<u32, Vec<(u32, u32)>> {
        let mut result = HashMap::new();
        let styled_items = self.loader.get_express_ids_with_type(schema::IFCSTYLEDITEM);

        for styled_item_id in styled_items {
            self.loader.move_to_argument_offset(styled_item_id, 0);
            let relating_item = self.loader.get_optional_ref_argument();
            if relating_item == 0 {
                continue;
            }
            self.loader.move_to_argument_offset(styled_item_id, 1);
            let styles = self.loader.get_set_argument();
            for style in styles {
                let style_id = self.loader.get_ref_argument_at(style);
                result
                    .entry(relating_item)
                    .or_default()
                    .push((styled_item_id, style_id));
            }
        }

        result
    }

    fn populate_rel_materials_map(&self) -> HashMap<u32, Vec<(u32, u32)>> {
        let mut result = HashMap::new();
        let rel_materials = self
            .loader
            .get_express_ids_with_type(schema::IFCRELASSOCIATESMATERIAL);

        for rel_material_id in rel_materials {
            self.loader.move_to_argument_offset(rel_material_id, 4);
            let relating_material = self.loader.get_ref_argument();
            self.loader.move_to_argument_offset(rel_material_id, 5);
            let related_objects = self.loader.get_set_argument();

            for related_object in related_objects {
                let related_object_id = self.loader.get_ref_argument_at(related_object);
                result
                    .entry(related_object_id)
                    .or_default()
                    .push((rel_material_id, relating_material));
            }
        }

        result
    }

    fn populate_material_definitions_map(&self) -> HashMap<u32, Vec<(u32, u32)>> {
        let mut result = HashMap::new();
        let material_defs = self
            .loader
            .get_express_ids_with_type(schema::IFCRELDEFINESBYPROPERTIES);

        for material_def_id in material_defs {
            self.loader.move_to_argument_offset(material_def_id, 4);
            let related_objects = self.loader.get_set_argument();
            self.loader.move_to_argument_offset(material_def_id, 5);
            let relating_property_definition = self.loader.get_ref_argument();

            for related_object in related_objects {
                let related_object_id = self.loader.get_ref_argument_at(related_object);
                result
                    .entry(related_object_id)
                    .or_default()
                    .push((material_def_id, relating_property_definition));
            }
        }

        result
    }

    fn read_linear_scaling_factor(&mut self) {
        let projects = self.loader.get_express_ids_with_type(schema::IFCPROJECT);
        if projects.len() != 1 {
            eprintln!("[read_linear_scaling_factor] unexpected empty ifc project");
            return;
        }

        let project_eid = projects[0];
        self.loader.move_to_argument_offset(project_eid, 8);
        let units_id = self.loader.get_ref_argument();
        self.loader.move_to_argument_offset(units_id, 0);
        let unit_ids = self.loader.get_set_argument();

        for unit_id in unit_ids {
            let unit_ref = self.loader.get_ref_argument_at(unit_id);
            let line_type = self.loader.get_line_type(unit_ref);

            if line_type == schema::IFCSIUNIT {
                self.loader.move_to_argument_offset(unit_ref, 1);
                let unit_type = self.loader.get_string_argument();

                let mut unit_prefix = "";
                self.loader.move_to_argument_offset(unit_ref, 2);
                if self.loader.get_token_type() == IfcTokenType::Enum {
                    self.loader.step_back();
                    unit_prefix = self.loader.get_string_argument();
                }

                self.loader.move_to_argument_offset(unit_ref, 3);
                let unit_name = self.loader.get_string_argument();

                if unit_type == "LENGTHUNIT" && unit_name == "METRE" {
                    let prefix = self.convert_prefix(unit_prefix);
                    self.linear_scaling_factor *= prefix;
                }
                if unit_type == "PLANEANGLEUNIT" {
                    self.angle_units = unit_name.to_string();
                }
            }

            if line_type == schema::IFCCONVERSIONBASEDUNIT {
                self.loader.move_to_argument_offset(unit_ref, 1);
                let unit_type = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(unit_ref, 3);
                let unit_ref_line = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(unit_ref_line, 1);
                let ratios = self.loader.get_set_argument();

                self.loader.move_to_argument_offset(unit_ref_line, 2);
                let scale_ref_line = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(scale_ref_line, 1);
                let unit_type_scale = self.loader.get_string_argument();

                let mut unit_prefix = "";
                self.loader.move_to_argument_offset(scale_ref_line, 2);
                if self.loader.get_token_type() == IfcTokenType::Enum {
                    self.loader.step_back();
                    unit_prefix = self.loader.get_string_argument();
                }

                self.loader.move_to_argument_offset(scale_ref_line, 3);
                let unit_name = self.loader.get_string_argument();

                if unit_type_scale == "LENGTHUNIT" && unit_name == "METRE" {
                    let prefix = self.convert_prefix(unit_prefix);
                    self.linear_scaling_factor *= prefix;
                }

                let ratio = self.loader.get_double_argument_at(ratios[0]);
                if unit_type == "LENGTHUNIT" {
                    self.linear_scaling_factor *= ratio;
                } else if unit_type == "AREAUNIT" {
                    self.squared_scaling_factor *= ratio;
                } else if unit_type == "VOLUMEUNIT" {
                    self.cubic_scaling_factor *= ratio;
                } else if unit_type == "PLANEANGLEUNIT" {
                    self.angular_scaling_factor *= ratio;
                }
            }
        }
    }

    fn convert_prefix(&self, prefix: &str) -> f64 {
        match prefix {
            "" => 1.0,
            "EXA" => 1e18,
            "PETA" => 1e15,
            "TERA" => 1e12,
            "GIGA" => 1e9,
            "MEGA" => 1e6,
            "KILO" => 1e3,
            "HECTO" => 1e2,
            "DECA" => 10.0,
            "DECI" => 1e-1,
            "CENTI" => 1e-2,
            "MILLI" => 1e-3,
            "MICRO" => 1e-6,
            "NANO" => 1e-9,
            "PICO" => 1e-12,
            "FEMTO" => 1e-15,
            "ATTO" => 1e-18,
            _ => 1.0,
        }
    }

    fn read_length_measure(&self) -> f64 {
        let t = self.loader.get_token_type();
        if t == IfcTokenType::Label {
            self.loader.step_back();
            let label = self.loader.get_string_argument();
            if label == "IFCNONNEGATIVELENGTHMEASURE" || label == "IFCLENGTHMEASURE" {
                self.loader.get_token_type();
                return self.loader.get_double_argument();
            }
            eprintln!("[read_length_measure] unrecognized type {}", label);
        }
        0.0
    }

    fn read_curve_measure_select(&self, trim: &mut IfcTrimmingSelect) {
        let t = self.loader.get_token_type();
        if t == IfcTokenType::Label {
            self.loader.step_back();
            let label = self.loader.get_string_argument();
            if label == "IFCPARAMETERVALUE" {
                self.loader.get_token_type();
                trim.trim_type =
                    crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter;
                trim.value = self.loader.get_double_argument();
            } else if label == "IFCLENGTHMEASURE" {
                self.loader.get_token_type();
                trim.trim_type =
                    crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByLength;
                trim.value = self.loader.get_double_argument();
            } else {
                eprintln!("[read_curve_measure_select] unrecognized type {}", label);
            }
        }
    }

    fn read_curve_indices(
        &self,
    ) -> Vec<crate::web_ifc::geometry::representation::geometry::IfcSegmentIndexSelect> {
        let mut result = Vec::new();
        let mut t = self.loader.get_token_type();
        if t == IfcTokenType::Ref {
            self.loader.step_back();
            self.loader
                .move_to_argument_offset(self.loader.get_ref_argument(), 0);
        }

        self.loader.step_back();
        while self.loader.get_token_type() != IfcTokenType::SetEnd {
            self.loader.step_back();
            if self.loader.get_token_type() == IfcTokenType::Label {
                let mut segment =
                    crate::web_ifc::geometry::representation::geometry::IfcSegmentIndexSelect {
                        r#type: String::new(),
                        indexs: Vec::new(),
                    };
                self.loader.step_back();
                segment.r#type = self.loader.get_string_argument().to_string();
                while self.loader.get_token_type() != IfcTokenType::SetEnd {
                    self.loader.step_back();
                    while self.loader.get_token_type() != IfcTokenType::SetEnd {
                        self.loader.step_back();
                        t = self.loader.get_token_type();
                        if t == IfcTokenType::Integer {
                            self.loader.step_back();
                            segment.indexs.push(self.loader.get_int_argument() as u32);
                        }
                    }
                }
                result.push(segment);
            }
        }
        result
    }

    fn compute_curve(&self, express_id: u32, curve: &mut IfcCurve, params: &ComputeCurveParams) {
        let line_type = self.loader.get_line_type(express_id);
        match line_type {
            schema::IFCPOLYLINE => {
                self.loader.move_to_argument_offset(express_id, 0);
                let points = self.loader.get_set_argument();
                for token in points {
                    let point_id = self.loader.get_ref_argument_at(token);
                    if params.dimensions == 2 {
                        curve.base.add_2d(self.get_cartesian_point_2d(point_id));
                    } else {
                        curve.base.add(self.get_cartesian_point_3d(point_id), true);
                    }
                }
                if params.edge {
                    if params.same_sense == 1 || params.same_sense == -1 {
                        curve.base.points.reverse();
                    }
                }
            }
            schema::IFCCOMPOSITECURVE => {
                self.loader.move_to_argument_offset(express_id, 0);
                let segments = self.loader.get_set_argument();
                let self_intersects = self.loader.get_string_argument();
                if self_intersects == "T" {
                    eprintln!(
                        "[compute_curve] Self intersecting composite curve {}",
                        express_id
                    );
                }
                for segment in segments {
                    let segment_id = self.loader.get_ref_argument_at(segment);
                    self.compute_curve(segment_id, curve, params);
                }
            }
            schema::IFCCOMPOSITECURVESEGMENT => {
                self.loader.move_to_argument_offset(express_id, 0);
                let _transition = self.loader.get_string_argument();
                let same_sense_s = self.loader.get_string_argument();
                let parent_id = self.loader.get_ref_argument();
                let mut segment_params = params.clone();
                segment_params.same_sense = if same_sense_s == "T" { 1 } else { 0 };
                self.compute_curve(parent_id, curve, &segment_params);
            }
            schema::IFCLINE => {
                let mut condition = params.same_sense == 1 || params.same_sense == -1;
                if params.edge {
                    condition = !condition;
                }
                if params.dimensions == 2 && params.has_trim {
                    if params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition
                    {
                        if condition {
                            curve.base.add_2d(params.trim_start.pos);
                            curve.base.add_2d(params.trim_end.pos);
                        } else {
                            curve.base.add_2d(params.trim_end.pos);
                            curve.base.add_2d(params.trim_start.pos);
                        }
                    } else if params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter
                    {
                        self.loader.move_to_argument_offset(express_id, 0);
                        let position_id = self.loader.get_ref_argument();
                        let vector_id = self.loader.get_ref_argument();
                        let mut placement = DVec3::new(
                            self.get_cartesian_point_2d(position_id).x,
                            self.get_cartesian_point_2d(position_id).y,
                            0.0,
                        );
                        let mut vector = self.get_vector(vector_id);

                        if params.ignore_placement {
                            vector = DVec3::new(1.0, 0.0, 0.0);
                            placement = DVec3::ZERO;
                        }

                        if condition {
                            let p1 = placement + vector * params.trim_start.value;
                            let p2 = placement + vector * params.trim_end.value;
                            curve.base.add(p1, true);
                            curve.base.add(p2, true);
                        } else {
                            let p2 = placement + vector * params.trim_start.value;
                            let p1 = placement + vector * params.trim_end.value;
                            curve.base.add(p1, true);
                            curve.base.add(p2, true);
                        }
                    } else {
                        eprintln!(
                            "[compute_curve] Unsupported trimmingselect 2D IFCLINE {}",
                            express_id
                        );
                    }
                } else if params.dimensions == 3 && params.has_trim {
                    if params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition
                    {
                        if condition {
                            curve.base.add(params.trim_start.pos3d, true);
                            curve.base.add(params.trim_end.pos3d, true);
                        } else {
                            curve.base.add(params.trim_end.pos3d, true);
                            curve.base.add(params.trim_start.pos3d, true);
                        }
                    } else if (params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter)
                        || (params.trim_start.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByLength
                            && params.trim_end.trim_type
                                == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByLength)
                    {
                        self.loader.move_to_argument_offset(express_id, 0);
                        let position_id = self.loader.get_ref_argument();
                        let vector_id = self.loader.get_ref_argument();
                        let mut placement = self.get_cartesian_point_3d(position_id);
                        let mut vector = self.get_vector(vector_id);

                        if params.ignore_placement {
                            vector = DVec3::new(1.0, 0.0, 0.0);
                            placement = DVec3::ZERO;
                        }

                        if condition {
                            let p1 = placement + vector * params.trim_start.value;
                            curve.base.add(p1, true);
                            if params.trim_start.value != params.trim_end.value {
                                let p2 = placement + vector * params.trim_end.value;
                                curve.base.add(p2, true);
                            }
                        } else {
                            let p1 = placement + vector * params.trim_end.value;
                            curve.base.add(p1, true);
                            if params.trim_start.value != params.trim_end.value {
                                let p2 = placement + vector * params.trim_start.value;
                                curve.base.add(p2, true);
                            }
                        }
                        if vector.length() > 0.0 {
                            curve.end_tangent = vector.normalize();
                            curve.segment_start_tangents.push(vector.normalize());
                        }
                    } else {
                        eprintln!(
                            "[compute_curve] Unsupported trimmingselect 3D IFCLINE {}",
                            express_id
                        );
                    }
                }
            }
            schema::IFCTRIMMEDCURVE => {
                self.loader.move_to_argument_offset(express_id, 0);
                let basis_curve_id = self.loader.get_ref_argument();
                let trim1_set = self.loader.get_set_argument();
                let trim2_set = self.loader.get_set_argument();
                let sense_agreement_s = self.loader.get_string_argument();
                let _trimming_preference = self.loader.get_string_argument();

                let trim1 = self.get_trim_select(params.dimensions, &trim1_set);
                let trim2 = self.get_trim_select(params.dimensions, &trim2_set);

                let mut basis_params = params.clone();
                basis_params.has_trim = true;
                basis_params.trim_start = trim1;
                basis_params.trim_end = trim2;

                let mut sense_agreement = if sense_agreement_s == "T" {
                    TrimSense::Same
                } else {
                    TrimSense::Reverse
                };
                if params.trim_sense == TrimSense::Reverse {
                    sense_agreement = if sense_agreement == TrimSense::Same {
                        TrimSense::Reverse
                    } else {
                        TrimSense::Same
                    };
                }
                basis_params.trim_sense = sense_agreement;
                self.compute_curve(basis_curve_id, curve, &basis_params);
            }
            schema::IFCINDEXEDPOLYCURVE => {
                self.loader.move_to_argument_offset(express_id, 0);
                let pts_ref = self.loader.get_ref_argument();

                self.loader.move_to_argument_offset(express_id, 2);
                if self.loader.get_token_type() != IfcTokenType::Empty {
                    self.loader.step_back();
                    let self_intersects = self.loader.get_string_argument();
                    if self_intersects == "T" {
                        eprintln!(
                            "[compute_curve] Self intersecting ifcindexedpolycurve {}",
                            express_id
                        );
                    }
                }

                if self.read_ifc_cartesian_point_list(pts_ref) {
                    self.loader.move_to_argument_offset(express_id, 1);
                    if self.loader.get_token_type() != IfcTokenType::Empty {
                        let pn_segment = self.read_curve_indices();
                        let pts = self.read_ifc_cartesian_point_list_2d(pts_ref);
                        for sg in pn_segment {
                            if sg.r#type == "IFCLINEINDEX" {
                                for pt in sg.indexs {
                                    curve.base.add_2d(pts[pt as usize - 1]);
                                }
                            }
                            if sg.r#type == "IFCARCINDEX" {
                                let arc = self.build_arc_3pt(
                                    pts[sg.indexs[0] as usize - 1],
                                    pts[sg.indexs[1] as usize - 1],
                                    pts[sg.indexs[2] as usize - 1],
                                );
                                for pt in arc.base.points {
                                    curve.base.add(pt, true);
                                }
                                let arc_len = arc.base.points.len();
                                if arc_len > 0 {
                                    curve
                                        .arc_segments
                                        .push((curve.base.points.len() - 1 - arc_len) as u32);
                                    curve
                                        .arc_segments
                                        .push((curve.base.points.len() - 1) as u32);
                                }
                            }
                        }
                    } else {
                        let pts = self.read_ifc_cartesian_point_list_2d(pts_ref);
                        for pt in pts {
                            curve.base.add_2d(pt);
                        }
                    }
                } else if !self.read_ifc_cartesian_point_list(pts_ref) {
                    self.loader.move_to_argument_offset(express_id, 1);
                    if self.loader.get_token_type() != IfcTokenType::Empty {
                        let pn_segment = self.read_curve_indices();
                        let pts = self.read_ifc_cartesian_point_list_3d(pts_ref);
                        for sg in pn_segment {
                            if sg.r#type == "IFCLINEINDEX" {
                                for pt in sg.indexs {
                                    curve.base.add(pts[pt as usize - 1], false);
                                }
                            }
                            if sg.r#type == "IFCARCINDEX" {
                                let arc = self.build_3d_arc_3pt(
                                    pts[sg.indexs[0] as usize - 1],
                                    pts[sg.indexs[1] as usize - 1],
                                    pts[sg.indexs[2] as usize - 1],
                                );
                                let arc_len = arc.base.points.len();
                                for pt in arc.base.points {
                                    curve.base.add(pt, false);
                                }
                                if arc_len > 0 {
                                    curve
                                        .arc_segments
                                        .push((curve.base.points.len() - 1 - arc_len) as u32);
                                    curve
                                        .arc_segments
                                        .push((curve.base.points.len() - 1) as u32);
                                }
                            }
                        }
                    } else {
                        let pts = self.read_ifc_cartesian_point_list_3d(pts_ref);
                        for pt in pts {
                            curve.base.add(pt, true);
                        }
                    }
                }
            }
            schema::IFCCIRCLE | schema::IFCELLIPSE => {
                self.loader.move_to_argument_offset(express_id, 0);
                let position_id = self.loader.get_ref_argument();
                let type_placement = self.loader.get_line_type(position_id);
                let dimensions = if type_placement == schema::IFCAXIS2PLACEMENT3D {
                    3
                } else {
                    2
                };

                self.loader.move_to_argument_offset(express_id, 1);
                let radius1 = self.loader.get_double_argument();
                let mut radius2 = radius1;
                if line_type == schema::IFCELLIPSE {
                    self.loader.move_to_argument_offset(express_id, 2);
                    if self.loader.get_token_type() == IfcTokenType::Real {
                        self.loader.step_back();
                        radius2 = self.loader.get_double_argument();
                    }
                }

                let mut start_rad = 0.0;
                let mut end_rad = 2.0 * CONST_PI;
                let mut by_pos = false;
                let trim_sense = params.trim_sense;
                if params.has_trim {
                    if params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter
                    {
                        start_rad = params.trim_start.value * self.angular_scaling_factor;
                        end_rad = params.trim_end.value * self.angular_scaling_factor;
                    } else if params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByLength
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByLength
                    {
                        if radius1 > 0.0 {
                            start_rad = params.trim_start.value / radius1;
                            end_rad = params.trim_end.value / radius1;
                        } else {
                            eprintln!(
                                "IFCCIRCLE (ID: {}) has zero radius1, cannot compute angles.",
                                express_id
                            );
                            return;
                        }
                    } else if params.trim_start.trim_type
                        == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition
                        && params.trim_end.trim_type
                            == crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition
                    {
                        by_pos = true;
                        if dimensions == 2 {
                            let placement = if params.ignore_placement {
                                DMat3::IDENTITY
                            } else {
                                self.get_axis2_placement_2d(position_id)
                            };
                            let xx = params.trim_start.pos.x - placement.z_axis.x;
                            let yy = params.trim_start.pos.y - placement.z_axis.y;
                            start_rad = vector_to_angle(xx, yy) * CONST_PI / 180.0;
                            let xx = params.trim_end.pos.x - placement.z_axis.x;
                            let yy = params.trim_end.pos.y - placement.z_axis.y;
                            end_rad = vector_to_angle(xx, yy) * CONST_PI / 180.0;
                        } else {
                            let placement = if params.ignore_placement {
                                DMat4::IDENTITY
                            } else {
                                self.get_local_placement(position_id, DVec3::ONE)
                            };
                            let vec_x = placement.x_axis;
                            let vec_y = placement.y_axis;
                            let v1 = DVec3::new(
                                params.trim_start.pos3d.x - placement.w_axis.x,
                                params.trim_start.pos3d.y - placement.w_axis.y,
                                params.trim_start.pos3d.z - placement.w_axis.z,
                            );
                            let v2 = DVec3::new(
                                params.trim_end.pos3d.x - placement.w_axis.x,
                                params.trim_end.pos3d.y - placement.w_axis.y,
                                params.trim_end.pos3d.z - placement.w_axis.z,
                            );
                            let dx_s = vec_x.truncate().dot(v1);
                            let dy_s = vec_y.truncate().dot(v1);
                            let dx_e = vec_x.truncate().dot(v2);
                            let dy_e = vec_y.truncate().dot(v2);
                            start_rad = vector_to_angle(dx_s, dy_s) * CONST_PI / 180.0;
                            end_rad = vector_to_angle(dx_e, dy_e) * CONST_PI / 180.0;
                        }
                    }
                }

                while start_rad < -2.0 * CONST_PI {
                    start_rad += 2.0 * CONST_PI;
                }
                while end_rad < -2.0 * CONST_PI {
                    end_rad += 2.0 * CONST_PI;
                }

                if trim_sense == TrimSense::Reverse {
                    std::mem::swap(&mut start_rad, &mut end_rad);
                }

                if by_pos && params.trim_sense == TrimSense::Reverse {
                    std::mem::swap(&mut start_rad, &mut end_rad);
                }

                if dimensions == 2 {
                    let placement = if params.ignore_placement {
                        DMat3::IDENTITY
                    } else {
                        self.get_axis2_placement_2d(position_id)
                    };
                    let curve_base = get_ellipse_curve(
                        radius1 as f32,
                        radius2 as f32,
                        self.circle_segments as i32,
                        placement,
                        start_rad,
                        end_rad,
                        true,
                        false,
                    );
                    curve.base = curve_base;
                } else {
                    let placement = if params.ignore_placement {
                        DMat4::IDENTITY
                    } else {
                        self.get_local_placement(position_id, DVec3::ONE)
                    };
                    let placement_2d = self.mat4_to_mat3(placement);
                    let curve_base = get_ellipse_curve(
                        radius1 as f32,
                        radius2 as f32,
                        self.circle_segments as i32,
                        placement_2d,
                        start_rad,
                        end_rad,
                        true,
                        false,
                    );
                    curve.base = curve_base;
                }
            }
            _ => {
                eprintln!("[compute_curve] Unsupported curve type {}", express_id);
            }
        }
    }

    fn get_trim_select(&self, dim: u8, tape_offsets: &[u32]) -> IfcTrimmingSelect {
        let mut ts = IfcTrimmingSelect::default();
        for (idx, offset) in tape_offsets.iter().enumerate() {
            let token_type = self.loader.get_token_type_at(*offset);
            self.loader.step_back();
            if token_type == IfcTokenType::Ref {
                let cartesian_point_ref = self.loader.get_ref_argument();
                ts.trim_type =
                    crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByPosition;
                if dim == 2 {
                    ts.pos = self.get_cartesian_point_2d(cartesian_point_ref);
                    ts.pos3d = DVec3::new(ts.pos.x, ts.pos.y, 0.0);
                } else {
                    ts.pos3d = self.get_cartesian_point_3d(cartesian_point_ref);
                    ts.pos = DVec2::new(ts.pos3d.x, ts.pos3d.y);
                }
            } else if token_type == IfcTokenType::Label {
                let r#type = self.loader.get_string_argument();
                if r#type == "IFCPARAMETERVALUE" && idx + 1 < tape_offsets.len() {
                    ts.trim_type =
                        crate::web_ifc::geometry::representation::geometry::IfcTrimmingSelectType::TrimByParameter;
                    ts.value = self.loader.get_double_argument_at(tape_offsets[idx + 1]);
                }
            }
        }
        ts
    }

    fn get_vertex_point(&self, express_id: u32) -> DVec3 {
        self.loader.move_to_argument_offset(express_id, 0);
        let point_ref = self.loader.get_ref_argument();
        let point_type = self.loader.get_line_type(point_ref);
        if point_type == schema::IFCCARTESIANPOINT {
            self.get_cartesian_point_3d(point_ref)
        } else {
            eprintln!(
                "[get_vertex_point] unexpected vertexpoint type {}",
                point_ref
            );
            DVec3::ZERO
        }
    }

    fn get_profile_by_line(&self, express_id: u32) -> IfcProfile {
        let line_type = self.loader.get_line_type(express_id);
        match line_type {
            schema::IFCARBITRARYOPENPROFILEDEF | schema::IFCARBITRARYCLOSEDPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                self.loader.move_to_argument_offset(express_id, 2);
                let curve_id = self.loader.get_ref_argument();
                profile.curve = self.get_curve(curve_id, 2, false);
                profile.is_convex = self.is_curve_convex(&profile.curve);
                profile
            }
            schema::IFCARBITRARYPROFILEDEFWITHVOIDS => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                self.loader.move_to_argument_offset(express_id, 2);
                profile.curve = self.get_curve(self.loader.get_ref_argument(), 2, false);
                profile.is_convex = self.is_curve_convex(&profile.curve);
                self.loader.move_to_argument_offset(express_id, 3);
                let holes = self.loader.get_set_argument();
                for hole in holes {
                    let hole_curve =
                        self.get_curve(self.loader.get_ref_argument_at(hole), 2, false);
                    profile.holes.push(hole_curve);
                }
                profile
            }
            schema::IFCRECTANGLEPROFILEDEF | schema::IFCROUNDEDRECTANGLEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let placement_id = self.loader.get_ref_argument();
                let xdim = self.loader.get_double_argument();
                let ydim = self.loader.get_double_argument();
                let placement = if placement_id != 0 {
                    self.get_axis2_placement_2d(placement_id)
                } else {
                    DMat3::IDENTITY
                };
                let placement_4 = self.mat3_to_mat4(placement);
                profile.curve.base =
                    get_rectangle_curve(xdim, ydim, placement_4, self.circle_segments as i32, 0.0);
                profile
            }
            schema::IFCRECTANGLEHOLLOWPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let placement_id = self.loader.get_ref_argument();
                let xdim = self.loader.get_double_argument();
                let ydim = self.loader.get_double_argument();
                let thickness = self.loader.get_double_argument();
                let mut inner_radius = 0.0;
                let mut outer_radius = 0.0;
                if self.loader.get_token_type() == IfcTokenType::Real {
                    self.loader.step_back();
                    inner_radius = self.loader.get_double_argument();
                }
                if self.loader.get_token_type() == IfcTokenType::Real {
                    self.loader.step_back();
                    outer_radius = self.loader.get_double_argument();
                }
                let placement = self.get_axis2_placement_2d(placement_id);
                let placement_4 = self.mat3_to_mat4(placement);
                profile.curve.base = get_rectangle_curve(
                    xdim,
                    ydim,
                    placement_4,
                    self.circle_segments as i32,
                    outer_radius,
                );
                let mut hole = IfcCurve::default();
                hole.base = get_rectangle_curve(
                    xdim - thickness,
                    ydim - thickness,
                    placement_4,
                    self.circle_segments as i32,
                    inner_radius,
                );
                hole.base.points.reverse();
                profile.holes.push(hole);
                profile
            }
            schema::IFCCIRCLEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let placement_id = self.loader.get_optional_ref_argument();
                let radius = self.loader.get_double_argument();
                let placement = if placement_id != 0 {
                    self.get_axis2_placement_2d(placement_id)
                } else {
                    DMat3::IDENTITY
                };
                profile.curve.base = get_ellipse_curve(
                    radius as f32,
                    radius as f32,
                    self.circle_segments as i32,
                    placement,
                    0.0,
                    CONST_PI * 2.0,
                    true,
                    false,
                );
                profile
            }
            schema::IFCELLIPSEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let placement_id = self.loader.get_ref_argument();
                let radius_x = self.loader.get_double_argument();
                let radius_y = self.loader.get_double_argument();
                let placement = self.get_axis2_placement_2d(placement_id);
                profile.curve.base = get_ellipse_curve(
                    radius_x as f32,
                    radius_y as f32,
                    self.circle_segments as i32,
                    placement,
                    0.0,
                    CONST_PI * 2.0,
                    true,
                    false,
                );
                profile
            }
            schema::IFCCIRCLEHOLLOWPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let placement_id = self.loader.get_optional_ref_argument();
                let radius = self.loader.get_double_argument();
                let thickness = self.loader.get_double_argument();
                let placement = if placement_id != 0 {
                    self.get_axis2_placement_2d(placement_id)
                } else {
                    DMat3::IDENTITY
                };
                profile.curve.base = get_ellipse_curve(
                    radius as f32,
                    radius as f32,
                    self.circle_segments as i32,
                    placement,
                    0.0,
                    CONST_PI * 2.0,
                    true,
                    false,
                );
                let mut hole = IfcCurve::default();
                hole.base = get_ellipse_curve(
                    (radius - thickness) as f32,
                    (radius - thickness) as f32,
                    self.circle_segments as i32,
                    placement,
                    0.0,
                    CONST_PI * 2.0,
                    true,
                    false,
                );
                hole.base.points.reverse();
                profile.holes.push(hole);
                profile
            }
            schema::IFCISHAPEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let mut placement = DMat3::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    placement = self.get_axis2_placement_2d(self.loader.get_ref_argument());
                }
                self.loader.move_to_argument_offset(express_id, 3);
                let width = self.loader.get_double_argument();
                let depth = self.loader.get_double_argument();
                let web_thickness = self.loader.get_double_argument();
                let flange_thickness = self.loader.get_double_argument();
                let fillet_radius = self.loader.get_optional_double_param(0.0);
                let has_fillet = fillet_radius > 0.0;
                profile.curve.base = get_i_shaped_curve(
                    width,
                    depth,
                    web_thickness,
                    flange_thickness,
                    has_fillet,
                    fillet_radius,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            schema::IFCLSHAPEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = false;
                self.loader.move_to_argument_offset(express_id, 2);
                let mut placement = DMat3::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    placement = self.get_axis2_placement_2d(self.loader.get_ref_argument());
                }
                self.loader.move_to_argument_offset(express_id, 3);
                let depth = self.loader.get_double_argument();
                let mut width = self.loader.get_optional_double_param(0.0);
                if width == 0.0 {
                    width = depth;
                }
                let thickness = self.loader.get_double_argument();
                let fillet_radius = self.loader.get_optional_double_param(0.0);
                let has_fillet = fillet_radius > 0.0;
                let edge_radius = self.loader.get_optional_double_param(0.0);
                let leg_slope = self.loader.get_optional_double_param(0.0);
                self.loader.get_optional_double_param(0.0);
                self.loader.get_optional_double_param(0.0);
                profile.curve.base = get_l_shaped_curve(
                    width,
                    depth,
                    thickness,
                    has_fillet,
                    fillet_radius,
                    edge_radius,
                    leg_slope,
                    self.circle_segments as u16,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            schema::IFCTSHAPEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = false;
                self.loader.move_to_argument_offset(express_id, 2);
                let mut placement = DMat3::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    placement = self.get_axis2_placement_2d(self.loader.get_ref_argument());
                }
                self.loader.move_to_argument_offset(express_id, 3);
                let depth = self.loader.get_double_argument();
                let width = self.loader.get_double_argument();
                let web_thickness = self.loader.get_double_argument();
                self.loader.get_double_argument();
                let fillet_radius = self.loader.get_optional_double_param(0.0);
                let has_fillet = fillet_radius != 0.0;
                let flange_edge_radius = self.loader.get_optional_double_param(0.0);
                self.loader.get_optional_double_param(0.0);
                self.loader.get_optional_double_param(0.0);
                let flange_slope = self.loader.get_optional_double_param(0.0);
                profile.curve.base = get_t_shaped_curve(
                    width,
                    depth,
                    web_thickness,
                    has_fillet,
                    fillet_radius,
                    flange_edge_radius,
                    flange_slope,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            schema::IFCUSHAPEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let mut placement = DMat3::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    placement = self.get_axis2_placement_2d(self.loader.get_ref_argument());
                }
                self.loader.move_to_argument_offset(express_id, 3);
                let depth = self.loader.get_double_argument();
                let flange_width = self.loader.get_double_argument();
                let web_thickness = self.loader.get_double_argument();
                let flange_thickness = self.loader.get_double_argument();
                let fillet_radius = self.loader.get_optional_double_param(0.0);
                let edge_radius = 0.0;
                let flange_slope = 0.0;
                profile.curve.base = get_u_shaped_curve(
                    depth,
                    flange_width,
                    web_thickness,
                    flange_thickness,
                    fillet_radius,
                    edge_radius,
                    flange_slope,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            schema::IFCCSHAPEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let mut placement = DMat3::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    placement = self.get_axis2_placement_2d(self.loader.get_ref_argument());
                }
                self.loader.move_to_argument_offset(express_id, 3);
                let depth = self.loader.get_double_argument();
                let width = self.loader.get_double_argument();
                let thickness = self.loader.get_double_argument();
                let girth = self.loader.get_double_argument();
                let fillet_radius = self.loader.get_optional_double_param(0.0);
                let has_fillet = fillet_radius > 0.0;
                self.loader.get_optional_double_param(0.0);
                profile.curve.base = get_c_shaped_curve(
                    width,
                    depth,
                    thickness,
                    girth,
                    has_fillet,
                    fillet_radius,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            schema::IFCZSHAPEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let mut placement = DMat3::IDENTITY;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    placement = self.get_axis2_placement_2d(self.loader.get_ref_argument());
                }
                self.loader.move_to_argument_offset(express_id, 3);
                let depth = self.loader.get_double_argument();
                let flange_width = self.loader.get_double_argument();
                let web_thickness = self.loader.get_double_argument();
                let flange_thickness = self.loader.get_double_argument();
                let fillet_radius = self.loader.get_optional_double_param(0.0);
                let edge_radius = self.loader.get_optional_double_param(0.0);
                profile.curve.base = get_z_shaped_curve(
                    depth,
                    flange_width,
                    web_thickness,
                    flange_thickness,
                    fillet_radius,
                    edge_radius,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            schema::IFCDERIVEDPROFILEDEF => {
                self.loader.move_to_argument_offset(express_id, 2);
                let profile_id = self.loader.get_ref_argument();
                let mut profile = self.get_profile_by_line(profile_id);
                self.loader.move_to_argument_offset(express_id, 3);
                let transform_id = self.loader.get_ref_argument();
                let transformation = self.get_axis2_placement_2d(transform_id);
                if !profile.is_composite {
                    for point in &mut profile.curve.base.points {
                        let transformed = transformation * DVec3::new(point.x, point.y, 1.0);
                        *point = DVec3::new(transformed.x, transformed.y, 0.0);
                    }
                } else {
                    for sub in &mut profile.profiles {
                        for point in &mut sub.curve.base.points {
                            let transformed = transformation * DVec3::new(point.x, point.y, 1.0);
                            *point = DVec3::new(transformed.x, transformed.y, 0.0);
                        }
                    }
                }
                profile
            }
            schema::IFCCOMPOSITEPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 2);
                let mut lst = Vec::new();
                if self.loader.get_token_type() == IfcTokenType::SetBegin {
                    while self.loader.get_token_type() == IfcTokenType::Ref {
                        self.loader.step_back();
                        let profile_id = self.loader.get_ref_argument();
                        lst.push(profile_id);
                    }
                }
                profile.is_composite = true;
                for profile_id in lst {
                    profile.profiles.push(self.get_profile_by_line(profile_id));
                }
                profile
            }
            schema::IFCOPENCROSSPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 2);
                let _horizontal_width = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(express_id, 3);
                let widths_tokens = self.loader.get_set_list_argument();
                self.loader.move_to_argument_offset(express_id, 4);
                let slopes_tokens = self.loader.get_set_list_argument();
                self.loader.move_to_argument_offset(express_id, 5);
                let tags_tokens = self.loader.get_set_list_argument();

                let mut list_widths = Vec::new();
                for set in widths_tokens {
                    for token in set {
                        list_widths.push(self.loader.get_double_argument_at(token));
                    }
                }

                let mut list_slopes = Vec::new();
                for set in slopes_tokens {
                    for token in set {
                        list_slopes.push(self.loader.get_double_argument_at(token));
                    }
                }

                let mut list_tags = Vec::new();
                for set in tags_tokens {
                    for token in set {
                        list_tags.push(self.loader.get_double_argument_at(token));
                    }
                }
                profile.tags = list_tags;

                let mut x = 0.0;
                let mut y = 0.0;
                for (idx, width) in list_widths.iter().enumerate() {
                    let mut slope_degrees = list_slopes[idx];
                    let mut slope_radians = slope_degrees;
                    self.convert_angle_units(&mut slope_degrees, &mut slope_radians);
                    let dx = width * slope_radians.cos();
                    let dy = width * slope_radians.sin();
                    profile.curve.base.add_2d(DVec2::new(x, y));
                    x += dx;
                    y += dy;
                }
                profile
            }
            schema::IFCTRAPEZIUMPROFILEDEF => {
                let mut profile = self.empty_profile();
                self.loader.move_to_argument_offset(express_id, 0);
                profile.r#type = self.loader.get_string_argument().to_string();
                profile.is_convex = true;
                self.loader.move_to_argument_offset(express_id, 2);
                let placement_id = self.loader.get_ref_argument();
                let bottom_x_dim = self.loader.get_double_argument();
                let top_x_dim = self.loader.get_double_argument();
                let y_dim = self.loader.get_double_argument();
                let top_x_offset = self.loader.get_double_argument();
                let placement = if placement_id != 0 {
                    self.get_axis2_placement_2d(placement_id)
                } else {
                    DMat3::IDENTITY
                };
                profile.curve.base = get_trapezium_curve(
                    bottom_x_dim,
                    top_x_dim,
                    y_dim,
                    top_x_offset,
                    self.mat3_to_mat4(placement),
                );
                profile
            }
            _ => {
                eprintln!(
                    "[get_profile_by_line] unexpected profile type {}",
                    express_id
                );
                self.empty_profile()
            }
        }
    }

    fn get_alignment_curve(&self, express_id: u32, parent_express_id: u32) -> IfcCurve {
        let line_type = self.loader.get_line_type(express_id);
        let mut alignment_curve = IfcCurve::default();
        match line_type {
            schema::IFCALIGNMENTSEGMENT => {
                self.loader.move_to_argument_offset(express_id, 5);
                let mut local_placement = 0;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    local_placement = self.loader.get_ref_argument();
                }
                let mut transform_t = DMat4::IDENTITY;
                if local_placement != 0 && self.loader.is_valid_express_id(local_placement) {
                    transform_t = self.get_local_placement(local_placement, DVec3::ONE);
                }
                self.loader.move_to_argument_offset(express_id, 7);
                let mut curve_id = 0;
                if self.loader.get_token_type() == IfcTokenType::Ref {
                    self.loader.step_back();
                    curve_id = self.loader.get_ref_argument();
                }
                if curve_id != 0 && self.loader.is_valid_express_id(curve_id) {
                    alignment_curve = self.get_alignment_curve(curve_id, parent_express_id);
                }
                for point in &mut alignment_curve.base.points {
                    let transformed = transform_t * point.extend(1.0);
                    *point = transformed.truncate();
                }
            }
            schema::IFCALIGNMENTHORIZONTALSEGMENT
            | schema::IFCALIGNMENTVERTICALSEGMENT
            | schema::IFCALIGNMENTSEGMENT1D
            | schema::IFCALIGNMENTSEGMENT2D => {
                self.loader.move_to_argument_offset(express_id, 8);
                let _type = self.loader.get_string_argument();
                self.loader.move_to_argument_offset(express_id, 2);
                let ifc_start_point = self.loader.get_ref_argument();
                let start_point = self.get_cartesian_point_2d(ifc_start_point);
                self.loader.move_to_argument_offset(express_id, 3);
                let _start_direction = self.loader.get_double_argument();
                self.loader.move_to_argument_offset(express_id, 4);
                let _start_radius = self.loader.get_double_argument();
                self.loader.move_to_argument_offset(express_id, 5);
                let _end_radius = self.loader.get_double_argument();
                self.loader.move_to_argument_offset(express_id, 6);
                let _segment_length = self.loader.get_double_argument();

                alignment_curve.base.add_2d(start_point);
            }
            _ => {}
        }
        alignment_curve
    }

    fn convert_angle_units(&self, degrees: &mut f64, rad: &mut f64) {
        if self.angle_units == "RADIAN" {
            *degrees = (*rad / CONST_PI) * 180.0;
        } else {
            *rad = *degrees / 180.0 * CONST_PI;
        }
    }

    fn empty_profile(&self) -> IfcProfile {
        IfcProfile {
            r#type: String::new(),
            curve: IfcCurve::default(),
            holes: Vec::new(),
            is_convex: false,
            is_composite: false,
            profiles: Vec::new(),
            tags: Vec::new(),
        }
    }

    fn is_curve_convex(&self, curve: &IfcCurve) -> bool {
        let points = &curve.base.points;
        if points.len() < 3 {
            return true;
        }
        for i in 2..points.len() {
            let a = points[i - 2];
            let b = points[i - 1];
            let c = points[i];
            let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            if cross < 0.0 {
                return false;
            }
        }
        true
    }

    fn not_present(&self, pt: DVec3, points: &[DVec3]) -> bool {
        !points
            .iter()
            .any(|p| p.x == pt.x && p.y == pt.y && p.z == pt.z)
    }

    fn mat3_to_mat4(&self, mat: DMat3) -> DMat4 {
        DMat4::from_cols(
            mat.x_axis.extend(0.0),
            mat.y_axis.extend(0.0),
            DVec4::new(0.0, 0.0, 1.0, 0.0),
            DVec4::new(mat.z_axis.x, mat.z_axis.y, 0.0, 1.0),
        )
    }

    fn mat4_to_mat3(&self, mat: DMat4) -> DMat3 {
        DMat3::from_cols(
            mat.x_axis.truncate(),
            mat.y_axis.truncate(),
            mat.w_axis.truncate(),
        )
    }

    fn build_arc_3pt(&self, p1: DVec2, p2: DVec2, p3: DVec2) -> IfcCurve {
        let f1 = p1.x * p1.x - p2.x * p2.x + p1.y * p1.y - p2.y * p2.y;
        let f2 = p1.x * p1.x - p3.x * p3.x + p1.y * p1.y - p3.y * p3.y;
        let v = 2.0 * (p1.x - p2.x) * (p1.y - p3.y) - 2.0 * (p1.x - p3.x) * (p1.y - p2.y);

        let cen_x = ((p1.y - p3.y) * f1 - (p1.y - p2.y) * f2) / v;
        let den1 = 2.0 * (p1.y - p3.y);
        let den2 = 2.0 * (p1.y - p2.y);
        let cen_y_a = (f2 - 2.0 * cen_x * (p1.x - p3.x)) / den1;
        let cen_y_b = (f1 - 2.0 * cen_x * (p1.x - p2.x)) / den2;
        let cen_y = if den1.abs() > den2.abs() {
            cen_y_a
        } else {
            cen_y_b
        };
        let center = DVec2::new(cen_x, cen_y);
        let radius = ((cen_x - p1.x).powi(2) + (cen_y - p1.y).powi(2)).sqrt();

        let mut point_list = vec![p1, p2, p3];
        while point_list.len() < self.circle_segments as usize {
            let mut temp_list = Vec::with_capacity(point_list.len() * 2);
            for j in 0..(point_list.len() - 1) {
                let mut pt = (point_list[j] + point_list[j + 1]) * 0.5;
                let vc = (pt - center).normalize();
                pt = center + vc * radius;
                temp_list.push(point_list[j]);
                temp_list.push(pt);
            }
            temp_list.push(*point_list.last().unwrap());
            point_list = temp_list;
        }

        let mut curve = IfcCurve::default();
        for pt in point_list {
            curve.base.add_2d(pt);
        }
        curve
    }

    fn build_3d_arc_3pt(&self, p1: DVec3, p2: DVec3, p3: DVec3) -> IfcCurve {
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let normal = v1.cross(v2).normalize_or_zero();
        if normal.length() < crate::web_ifc::geometry::representation::geometry::EPS_TINY {
            return IfcCurve::default();
        }

        let c_x = p2.x - p1.x;
        let c_y = p2.y - p1.y;
        let c_z = p2.z - p1.z;
        let b_x = p3.x - p1.x;
        let b_y = p3.y - p1.y;
        let b_z = p3.z - p1.z;
        let b2 = p1.x * p1.x - p3.x * p3.x + p1.y * p1.y - p3.y * p3.y + p1.z * p1.z - p3.z * p3.z;
        let c2 = p1.x * p1.x - p2.x * p2.x + p1.y * p1.y - p2.y * p2.y + p1.z * p1.z - p2.z * p2.z;

        let cb_yz = c_y * b_z - c_z * b_y;
        let cb_xz = c_x * b_z - c_z * b_x;
        let cb_xy = c_x * b_y - c_y * b_x;
        let zz1 = -(b_z - c_z * b_x / c_x) / (b_y - c_y * b_x / c_x);
        let z01 = -(b2 - b_x / c_x * c2) / (2.0 * (b_y - c_y * b_x / c_x));
        let zz2 = -(zz1 * c_y + c_z) / c_x;
        let z02 = -(2.0 * z01 * c_y + c2) / (2.0 * c_x);

        let dz = -((z02 - p1.x) * cb_yz - (z01 - p1.y) * cb_xz - p1.z * cb_xy)
            / (zz2 * cb_yz - zz1 * cb_xz + cb_xy);
        let dx = zz2 * dz + z02;
        let dy = zz1 * dz + z01;
        let center = DVec3::new(dx, dy, dz);

        let radius = center.distance(p1);

        let mut point_list = vec![p1, p2, p3];
        while point_list.len() < self.circle_segments as usize {
            let mut temp_list = Vec::with_capacity(point_list.len() * 2);
            for j in 0..(point_list.len() - 1) {
                let mut pt = (point_list[j] + point_list[j + 1]) * 0.5;
                let vc = (pt - center).normalize_or_zero();
                pt = center + vc * radius;
                temp_list.push(point_list[j]);
                temp_list.push(pt);
            }
            temp_list.push(*point_list.last().unwrap());
            point_list = temp_list;
        }

        let mut curve = IfcCurve::default();
        for pt in point_list {
            curve.base.add(pt, true);
        }
        curve
    }

    pub fn clone_loader(&self, loader: &'a IfcLoader) -> Self {
        Self::new(
            loader,
            self.schema_manager,
            self.circle_segments,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct ComputeCurveParams {
    pub dimensions: u8,
    pub ignore_placement: bool,
    pub edge: bool,
    pub same_sense: i32,
    pub has_trim: bool,
    pub trim_start: IfcTrimmingSelect,
    pub trim_end: IfcTrimmingSelect,
    pub trim_sense: TrimSense,
}
