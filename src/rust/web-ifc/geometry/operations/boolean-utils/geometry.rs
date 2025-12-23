//! Rust port of `web-ifc/geometry/operations/boolean-utils/geometry.h`.

use glam::{DMat4, DVec3, DVec4};

use super::aabb::{Aabb, AABB};
use super::eps::{
    EPS_SMALL, MESSAGES, TOLERANCE_ADD_FACE, TOLERANCE_SCALAR_EQUALITY, TOLERANCE_VECTOR_EQUALITY,
};
use super::math::{area_of_triangle, compute_safe_normal, equals, equals_vec3};

pub type Vec = DVec3;

pub const VERTEX_FORMAT_SIZE_FLOATS: i32 = 6;

#[derive(Clone, Debug, Default)]
pub struct SimplePlane {
    pub distance: f64,
    pub normal: Vec,
}

impl SimplePlane {
    pub fn is_equal_to(&self, n: Vec, d: f64) -> bool {
        equals_vec3(self.normal, n, TOLERANCE_VECTOR_EQUALITY)
            && equals(self.distance, d, TOLERANCE_SCALAR_EQUALITY)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Face {
    pub p_id: u32,
    pub i0: u32,
    pub i1: u32,
    pub i2: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Geometry {
    pub fvertex_data: Vec<f32>,
    pub vertex_data: Vec<f64>,
    pub index_data: Vec<u32>,
    pub plane_data: Vec<u32>,
    pub planes: Vec<SimplePlane>,
    pub has_planes: bool,
    pub num_points: u32,
    pub num_faces: u32,
    pub data: u32,
}

impl Geometry {
    pub fn build_from_vectors(&mut self, d: &mut Vec<f64>, i: &mut Vec<u32>) {
        self.vertex_data = std::mem::take(d);
        self.index_data = std::mem::take(i);

        self.num_points = self.index_data.len() as u32;
        self.num_faces = (self.index_data.len() / 3) as u32;
    }

    pub fn add_point_vec4(&mut self, pt: DVec4, n: DVec3) {
        self.add_point(pt.truncate(), n);
    }

    pub fn get_aabb(&self) -> Aabb {
        let mut aabb = Aabb::default();
        for i in 0..self.num_points as usize {
            aabb.min = aabb.min.min(self.get_point(i));
            aabb.max = aabb.max.max(self.get_point(i));
        }
        aabb
    }

    pub fn add_point(&mut self, pt: DVec3, n: DVec3) {
        self.vertex_data.push(pt.x);
        self.vertex_data.push(pt.y);
        self.vertex_data.push(pt.z);
        self.vertex_data.push(n.x);
        self.vertex_data.push(n.y);
        self.vertex_data.push(n.z);

        if (pt.x.is_nan() || pt.y.is_nan() || pt.z.is_nan()) && MESSAGES {
            println!("NaN in geom!");
        }
        if (n.x.is_nan() || n.y.is_nan() || n.z.is_nan()) && MESSAGES {
            println!("NaN in geom!");
        }

        self.num_points += 1;
    }

    pub fn add_face_points(&mut self, a: DVec3, b: DVec3, c: DVec3, p_id: u32) {
        let mut normal = DVec3::ZERO;
        let _area = area_of_triangle(a, b, c);
        if !compute_safe_normal(a, b, c, &mut normal, TOLERANCE_ADD_FACE) {
            if MESSAGES {
                println!("zero triangle, AddFace(vec, vec, vec)");
            }
            return;
        }
        self.add_point(a, normal);
        self.add_point(b, normal);
        self.add_point(c, normal);
        self.add_face_indices(
            self.num_points - 3,
            self.num_points - 2,
            self.num_points - 1,
            p_id,
        );
    }

    pub fn add_face_indices(&mut self, a: u32, b: u32, c: u32, p_id: u32) {
        self.index_data.push(a);
        self.index_data.push(b);
        self.index_data.push(c);
        self.plane_data.push(p_id);

        let mut normal = DVec3::ZERO;
        if !compute_safe_normal(
            self.get_point(a as usize),
            self.get_point(b as usize),
            self.get_point(c as usize),
            &mut normal,
            TOLERANCE_ADD_FACE,
        ) && MESSAGES
        {
            println!("zero triangle, AddFace(int, int, int)");
        }

        self.num_faces += 1;
    }

    pub fn get_face(&self, index: usize) -> Face {
        Face {
            i0: self.index_data[index * 3],
            i1: self.index_data[index * 3 + 1],
            i2: self.index_data[index * 3 + 2],
            p_id: self.plane_data[index],
        }
    }

    pub fn get_face_box(&self, index: usize) -> AABB {
        let mut aabb = AABB::default();
        aabb.index = index as u32;

        let a = self.get_point(self.index_data[index * 3] as usize);
        let b = self.get_point(self.index_data[index * 3 + 1] as usize);
        let c = self.get_point(self.index_data[index * 3 + 2] as usize);

        aabb.min = aabb.min.min(a);
        aabb.min = aabb.min.min(b);
        aabb.min = aabb.min.min(c);

        aabb.max = aabb.max.max(a);
        aabb.max = aabb.max.max(b);
        aabb.max = aabb.max.max(c);

        aabb.center = (aabb.max + aabb.min) / 2.0;

        aabb
    }

    pub fn get_point(&self, index: usize) -> DVec3 {
        let base = index * VERTEX_FORMAT_SIZE_FLOATS as usize;
        DVec3::new(
            self.vertex_data[base],
            self.vertex_data[base + 1],
            self.vertex_data[base + 2],
        )
    }

    pub fn get_center_extents(&self, center: &mut DVec3, extents: &mut DVec3) {
        let mut min = DVec3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = DVec3::new(f64::MIN, f64::MIN, f64::MIN);

        for i in 0..self.num_points as usize {
            let pt = self.get_point(i);
            min = min.min(pt);
            max = max.max(pt);
        }

        *extents = max - min;
        *center = min + *extents / 2.0;
    }

    pub fn normalize(&self, center: DVec3, extents: DVec3) -> Geometry {
        let mut new_geom = Geometry::default();
        let scale = extents.x.max(extents.y.max(extents.z)) / 10.0;

        for i in 0..self.num_faces as usize {
            let face = self.get_face(i);
            let pa = self.get_point(face.i0 as usize);
            let pb = self.get_point(face.i1 as usize);
            let pc = self.get_point(face.i2 as usize);

            let _a = (pa - center) / scale;
            let _b = (pb - center) / scale;
            let _c = (pc - center) / scale;

            new_geom.add_face_points(pa, pb, pc, face.p_id);
        }

        new_geom
    }

    pub fn denormalize(&self, center: DVec3, extents: DVec3) -> Geometry {
        let mut new_geom = Geometry::default();
        let scale = extents.x.max(extents.y.max(extents.z)) / 10.0;

        for i in 0..self.num_faces as usize {
            let face = self.get_face(i);
            let pa = self.get_point(face.i0 as usize);
            let pb = self.get_point(face.i1 as usize);
            let pc = self.get_point(face.i2 as usize);

            let a = pa * scale + center;
            let b = pb * scale + center;
            let c = pc * scale + center;

            new_geom.add_face_points(a, b, c, face.p_id);
        }

        new_geom
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_data.is_empty()
    }

    pub fn volume(&self, trans: DMat4) -> f64 {
        let mut total_volume = 0.0;

        for i in 0..self.num_faces as usize {
            let face = self.get_face(i);

            let a = (trans
                * DVec4::new(
                    self.get_point(face.i0 as usize).x,
                    self.get_point(face.i0 as usize).y,
                    self.get_point(face.i0 as usize).z,
                    1.0,
                ))
            .truncate();
            let b = (trans
                * DVec4::new(
                    self.get_point(face.i1 as usize).x,
                    self.get_point(face.i1 as usize).y,
                    self.get_point(face.i1 as usize).z,
                    1.0,
                ))
            .truncate();
            let c = (trans
                * DVec4::new(
                    self.get_point(face.i2 as usize).x,
                    self.get_point(face.i2 as usize).y,
                    self.get_point(face.i2 as usize).z,
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
