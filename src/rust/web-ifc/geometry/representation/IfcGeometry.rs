//! Rust port of IfcGeometry and related wrappers.

use glam::{DMat4, DVec3, DVec4};

use crate::web_ifc::geometry::operations::bim_geometry::aabb::AABB as BimAabb;
use crate::web_ifc::geometry::operations::bim_geometry::geometry::Geometry as BimGeometry;
use crate::web_ifc::geometry::operations::bim_geometry::plane::Plane as BimPlane;
use crate::web_ifc::geometry::operations::bim_geometry::utils::{
    area_of_triangle, compute_safe_normal, EPS_SMALL,
};
use crate::web_ifc::geometry::representation::geometry::SweptDiskSolid;

pub const VERTEX_FORMAT_SIZE_FLOATS: i32 =
    crate::web_ifc::geometry::operations::bim_geometry::utils::VERTEX_FORMAT_SIZE_FLOATS;

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
    pub fn build_from_vectors(&mut self, data: &mut Vec<f64>, indices: &mut Vec<u32>) {
        self.base.vertex_data = std::mem::take(data);
        self.base.index_data = std::mem::take(indices);

        self.base.num_points = self.base.index_data.len() as u32;
        self.base.num_faces = (self.base.index_data.len() / 3) as u32;
    }

    pub fn get_face_box(&self, index: usize) -> AABB {
        let mut aabb = AABB::default();
        aabb.base.index = index as u32;

        let a = self
            .base
            .get_point(self.base.index_data[index * 3] as usize);
        let b = self
            .base
            .get_point(self.base.index_data[index * 3 + 1] as usize);
        let c = self
            .base
            .get_point(self.base.index_data[index * 3 + 2] as usize);

        aabb.base.min = aabb.base.min.min(a);
        aabb.base.min = aabb.base.min.min(b);
        aabb.base.min = aabb.base.min.min(c);

        aabb.base.max = aabb.base.max.max(a);
        aabb.base.max = aabb.base.max.max(b);
        aabb.base.max = aabb.base.max.max(c);

        aabb.base.center = (aabb.base.max + aabb.base.min) / 2.0;

        aabb
    }

    pub fn get_center_extents(&self, center: &mut DVec3, extents: &mut DVec3) {
        let mut min = DVec3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = DVec3::new(-f64::MAX, -f64::MAX, -f64::MAX);

        for i in 0..self.base.num_points as usize {
            let pt = self.base.get_point(i);
            min = min.min(pt);
            max = max.max(pt);
        }

        *extents = max - min;
        *center = min + *extents / 2.0;
    }

    pub fn normalize(&self, center: DVec3, extents: DVec3) -> Geometry {
        let mut new_geom = Geometry::default();
        let scale = extents.x.max(extents.y.max(extents.z)) / 10.0;

        for i in 0..self.base.num_faces as usize {
            let face = self.base.get_face(i);
            let pa = self.base.get_point(face.i0 as usize);
            let pb = self.base.get_point(face.i1 as usize);
            let pc = self.base.get_point(face.i2 as usize);

            let _a = (pa - center) / scale;
            let _b = (pb - center) / scale;
            let _c = (pc - center) / scale;

            new_geom.base.add_face_points(pa, pb, pc, face.p_id);
        }

        new_geom
    }

    pub fn denormalize(&self, center: DVec3, extents: DVec3) -> Geometry {
        let mut new_geom = Geometry::default();
        let scale = extents.x.max(extents.y.max(extents.z)) / 10.0;

        for i in 0..self.base.num_faces as usize {
            let face = self.base.get_face(i);
            let pa = self.base.get_point(face.i0 as usize);
            let pb = self.base.get_point(face.i1 as usize);
            let pc = self.base.get_point(face.i2 as usize);

            let a = pa * scale + center;
            let b = pb * scale + center;
            let c = pc * scale + center;

            new_geom.base.add_face_points(a, b, c, face.p_id);
        }

        new_geom
    }

    pub fn is_empty(&self) -> bool {
        self.base.vertex_data.is_empty()
    }

    pub fn volume(&self, trans: DMat4) -> f64 {
        let mut total_volume = 0.0;

        for i in 0..self.base.num_faces as usize {
            let face = self.base.get_face(i);

            let a = (trans
                * DVec4::new(
                    self.base.get_point(face.i0 as usize).x,
                    self.base.get_point(face.i0 as usize).y,
                    self.base.get_point(face.i0 as usize).z,
                    1.0,
                ))
            .truncate();
            let b = (trans
                * DVec4::new(
                    self.base.get_point(face.i1 as usize).x,
                    self.base.get_point(face.i1 as usize).y,
                    self.base.get_point(face.i1 as usize).z,
                    1.0,
                ))
            .truncate();
            let c = (trans
                * DVec4::new(
                    self.base.get_point(face.i2 as usize).x,
                    self.base.get_point(face.i2 as usize).y,
                    self.base.get_point(face.i2 as usize).z,
                    1.0,
                ))
            .truncate();

            let mut norm = DVec3::ZERO;
            if compute_safe_normal(a, b, c, &mut norm, EPS_SMALL) {
                let area = area_of_triangle(a, b, c);
                let height = norm.dot(a);
                let tetra_volume = area * height / 3.0;
                total_volume += tetra_volume;
            }
        }

        total_volume
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
        for i in 0..self.base.base.num_faces {
            self.reverse_face(i);
        }
    }

    pub fn add_part(&mut self, geom: IfcGeometry) {
        self.part.push(geom);
    }

    pub fn add_part_from_geometry(&mut self, geom: Geometry) {
        let mut new_geom = IfcGeometry::default();
        new_geom.merge_geometry(geom);
        self.part.push(new_geom);
    }

    pub fn add_geometry(
        &mut self,
        geom: Geometry,
        trans: DMat4,
        scx: f64,
        scy: f64,
        scz: f64,
        origin: DVec3,
    ) {
        for i in 0..geom.base.num_faces as usize {
            let face = geom.base.get_face(i);
            let mut a = geom.base.get_point(face.i0 as usize);
            let mut b = geom.base.get_point(face.i1 as usize);
            let mut c = geom.base.get_point(face.i2 as usize);
            if scx != 1.0 || scy != 1.0 || scz != 1.0 {
                let a_vec = DVec4::new(a.x - origin.x, a.y - origin.y, a.z - origin.z, 1.0);
                let aax = trans.x_axis.dot(a_vec) * scx;
                let aay = trans.y_axis.dot(a_vec) * scy;
                let aaz = trans.z_axis.dot(a_vec) * scz;
                a = origin
                    + trans.x_axis.truncate() * aax
                    + trans.y_axis.truncate() * aay
                    + trans.z_axis.truncate() * aaz;

                let b_vec = DVec4::new(b.x - origin.x, b.y - origin.y, b.z - origin.z, 1.0);
                let bbx = trans.x_axis.dot(b_vec) * scx;
                let bby = trans.y_axis.dot(b_vec) * scy;
                let bbz = trans.z_axis.dot(b_vec) * scz;
                b = origin
                    + trans.x_axis.truncate() * bbx
                    + trans.y_axis.truncate() * bby
                    + trans.z_axis.truncate() * bbz;

                let c_vec = DVec4::new(c.x - origin.x, c.y - origin.y, c.z - origin.z, 1.0);
                let ccx = trans.x_axis.dot(c_vec) * scx;
                let ccy = trans.y_axis.dot(c_vec) * scy;
                let ccz = trans.z_axis.dot(c_vec) * scz;
                c = origin
                    + trans.x_axis.truncate() * ccx
                    + trans.y_axis.truncate() * ccy
                    + trans.z_axis.truncate() * ccz;
            }
            self.base.base.add_face_points(a, b, c, face.p_id);
        }
        self.add_part_from_geometry(geom);
    }

    pub fn merge_geometry(&mut self, geom: Geometry) {
        for i in 0..geom.base.num_faces as usize {
            let face = geom.base.get_face(i);
            let a = geom.base.get_point(face.i0 as usize);
            let b = geom.base.get_point(face.i1 as usize);
            let c = geom.base.get_point(face.i2 as usize);
            self.base.base.add_face_points(a, b, c, face.p_id);
        }
    }

    pub fn get_vertex_data(&mut self) -> *const u8 {
        if self.base.base.fvertex_data.len() != self.base.base.vertex_data.len() {
            self.base.base.fvertex_data = self
                .base
                .base
                .vertex_data
                .iter()
                .map(|value| *value as f32)
                .collect();
        }
        if self.base.base.fvertex_data.is_empty() {
            return std::ptr::null();
        }
        self.base.base.fvertex_data.as_ptr() as *const u8
    }

    pub fn get_vertex_data_size(&self) -> u32 {
        self.base.base.fvertex_data.len() as u32
    }

    pub fn get_index_data(&self) -> *const u8 {
        if self.base.base.index_data.is_empty() {
            return std::ptr::null();
        }
        self.base.base.index_data.as_ptr() as *const u8
    }

    pub fn get_index_data_size(&self) -> u32 {
        self.base.base.index_data.len() as u32
    }

    pub fn get_swept_disk_solid(&self) -> SweptDiskSolid {
        self.swept_disk_solid.clone()
    }

    pub fn normalize(&mut self) -> DMat4 {
        let mut center = self.normalization_center;
        if !self.normalized {
            let mut extents = DVec3::ZERO;
            self.base.get_center_extents(&mut center, &mut extents);

            for i in (0..self.base.base.vertex_data.len()).step_by(6) {
                self.base.base.vertex_data[i] -= center.x;
                self.base.base.vertex_data[i + 1] -= center.y;
                self.base.base.vertex_data[i + 2] -= center.z;
            }

            for curve in &mut self.swept_disk_solid.axis {
                for point in &mut curve.base.points {
                    point.x -= center.x;
                    point.y -= center.y;
                    point.z -= center.z;
                }
            }

            for profile in &mut self.swept_disk_solid.profiles {
                for point in &mut profile.curve.base.points {
                    point.x -= center.x;
                    point.y -= center.y;
                    point.z -= center.z;
                }
                for hole in &mut profile.holes {
                    for point in &mut hole.base.points {
                        point.x -= center.x;
                        point.y -= center.y;
                        point.z -= center.z;
                    }
                }
            }

            self.normalization_center = center;
            self.normalized = true;
        }

        DMat4::from_translation(center)
    }

    fn reverse_face(&mut self, _index: u32) {
        let face = self.base.base.get_face(_index as usize);
        let base_index = _index as usize * 3;
        self.base.base.index_data[base_index] = face.i2;
        self.base.base.index_data[base_index + 1] = face.i1;
        self.base.base.index_data[base_index + 2] = face.i0;
    }
}
