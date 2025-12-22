//! Rust port of IfcGeometry and related wrappers.

use glam::{DMat4, DVec3};

use crate::web_ifc::geometry::operations::bim_geometry::aabb::AABB as BimAabb;
use crate::web_ifc::geometry::operations::bim_geometry::geometry::Geometry as BimGeometry;
use crate::web_ifc::geometry::operations::bim_geometry::plane::Plane as BimPlane;
use crate::web_ifc::geometry::representation::geometry::{SweptDiskSolid};

pub const VERTEX_FORMAT_SIZE_FLOATS: i32 = 0;

#[derive(Clone, Debug, Default)]
pub struct Plane {
    pub base: BimPlane,
}

#[derive(Clone, Debug, Default)]
pub struct AABB {
    pub base: BimAabb,
}

#[derive(Clone, Debug, Default)]
pub struct Geometry {
    pub base: BimGeometry,
    pub is_polygon: bool,
}

impl Geometry {
    pub fn build_from_vectors(&mut self, _data: &mut Vec<f64>, _indices: &mut Vec<u32>) {
        unimplemented!("Geometry::build_from_vectors not yet implemented");
    }

    pub fn get_face_box(&self, _index: usize) -> AABB {
        unimplemented!("Geometry::get_face_box not yet implemented");
    }

    pub fn get_center_extents(&self, _center: &mut DVec3, _extents: &mut DVec3) {
        unimplemented!("Geometry::get_center_extents not yet implemented");
    }

    pub fn normalize(&self, _center: DVec3, _extents: DVec3) -> Geometry {
        unimplemented!("Geometry::normalize not yet implemented");
    }

    pub fn denormalize(&self, _center: DVec3, _extents: DVec3) -> Geometry {
        unimplemented!("Geometry::denormalize not yet implemented");
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    pub fn volume(&self, _trans: DMat4) -> f64 {
        self.base.volume(_trans)
    }
}

#[derive(Clone, Debug, Default)]
pub struct IfcGeometry {
    pub base: Geometry,
    pub half_space: bool,
    pub part: Vec<IfcGeometry>,
    pub half_space_x: DVec3,
    pub half_space_y: DVec3,
    pub half_space_z: DVec3,
    pub half_space_origin: DVec3,
    pub normalization_center: DVec3,
    pub swept_disk_solid: SweptDiskSolid,
    normalized: bool,
}

impl IfcGeometry {
    pub fn reverse_faces(&mut self) {
        unimplemented!("IfcGeometry::reverse_faces not yet implemented");
    }

    pub fn add_part(&mut self, _geom: IfcGeometry) {
        unimplemented!("IfcGeometry::add_part not yet implemented");
    }

    pub fn add_part_from_geometry(&mut self, _geom: Geometry) {
        unimplemented!("IfcGeometry::add_part_from_geometry not yet implemented");
    }

    pub fn add_geometry(&mut self, _geom: Geometry, _trans: DMat4, _scx: f64, _scy: f64, _scz: f64, _origin: DVec3) {
        unimplemented!("IfcGeometry::add_geometry not yet implemented");
    }

    pub fn merge_geometry(&mut self, _geom: Geometry) {
        unimplemented!("IfcGeometry::merge_geometry not yet implemented");
    }

    pub fn get_vertex_data(&self) -> *const u8 {
        std::ptr::null()
    }

    pub fn get_vertex_data_size(&self) -> u32 {
        0
    }

    pub fn get_index_data(&self) -> *const u8 {
        std::ptr::null()
    }

    pub fn get_index_data_size(&self) -> u32 {
        0
    }

    pub fn get_swept_disk_solid(&self) -> SweptDiskSolid {
        self.swept_disk_solid.clone()
    }

    pub fn normalize(&mut self) -> DMat4 {
        self.normalized = true;
        DMat4::IDENTITY
    }

    fn reverse_face(&mut self, _index: u32) {
        unimplemented!("IfcGeometry::reverse_face not yet implemented");
    }
}
