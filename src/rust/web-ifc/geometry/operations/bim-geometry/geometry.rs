//! Geometry mesh container.

use glam::{DVec3, DVec4};

use super::aabb::Aabb;
use super::epsilons::{EPS_SMALL, RECONSTRUCT_TOLERANCE, TOLERANCE_ADD_FACE};
use super::face::Face;
use super::plane::Plane;
use super::utils::{area_of_triangle, compute_safe_normal, VERTEX_FORMAT_SIZE_FLOATS};

#[derive(Clone, Debug, Default)]
pub struct Geometry {
    pub has_planes: bool,
    pub num_points: u32,
    pub num_faces: u32,
    pub fvertex_data: Vec<f32>,
    pub vertex_data: Vec<f64>,
    pub index_data: Vec<u32>,
    pub plane_data: Vec<u32>,
    pub planes: Vec<Plane>,
}

impl Geometry {
    pub fn add_point_vec4(&mut self, pt: DVec4, n: DVec3) {
        self.add_point(pt.truncate(), n);
    }

    pub fn get_aabb(&self) -> Aabb {
        let mut aabb = Aabb::default();
        for i in 0..self.num_points {
            aabb.min = aabb.min.min(self.get_point(i as usize));
            aabb.max = aabb.max.max(self.get_point(i as usize));
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
        self.num_points += 1;
    }

    pub fn get_point(&self, index: usize) -> DVec3 {
        let base = index * VERTEX_FORMAT_SIZE_FLOATS as usize;
        DVec3::new(
            self.vertex_data[base],
            self.vertex_data[base + 1],
            self.vertex_data[base + 2],
        )
    }

    pub fn set_point(&mut self, x: f64, y: f64, z: f64, index: usize) {
        let base = index * VERTEX_FORMAT_SIZE_FLOATS as usize;
        self.vertex_data[base] = x;
        self.vertex_data[base + 1] = y;
        self.vertex_data[base + 2] = z;
    }

    pub fn get_face(&self, index: usize) -> Face {
        Face {
            i0: self.index_data[index * 3],
            i1: self.index_data[index * 3 + 1],
            i2: self.index_data[index * 3 + 2],
            p_id: self.plane_data[index],
        }
    }

    pub fn add_face_points(&mut self, a: DVec3, b: DVec3, c: DVec3, p_id: u32) {
        let mut normal = DVec3::ZERO;
        if !compute_safe_normal(a, b, c, &mut normal, TOLERANCE_ADD_FACE) {
            return;
        }
        self.add_point(a, normal);
        self.add_point(b, normal);
        self.add_point(c, normal);
        self.add_face_indices(self.num_points - 3, self.num_points - 2, self.num_points - 1, p_id);
    }

    pub fn add_face_indices(&mut self, a: u32, b: u32, c: u32, p_id: u32) {
        self.index_data.push(a);
        self.index_data.push(b);
        self.index_data.push(c);
        let _area = area_of_triangle(self.get_point(a as usize), self.get_point(b as usize), self.get_point(c as usize));
        self.num_faces += 1;
        self.plane_data.push(p_id);
    }

    pub fn add_plane(&mut self, normal: DVec3, distance: f64) -> usize {
        for plane in &self.planes {
            if plane.is_equal_to(normal, distance) {
                return plane.id;
            }
        }

        let id = self.planes.len();
        self.planes.push(Plane {
            id,
            normal: normal.normalize(),
            distance,
        });
        id
    }

    pub fn build_planes(&mut self) {
        if self.has_planes {
            return;
        }

        let stored_vertex_data = self.vertex_data.clone();
        let get_stored_point = |idx: usize| -> DVec3 {
            let base = idx * VERTEX_FORMAT_SIZE_FLOATS as usize;
            DVec3::new(stored_vertex_data[base], stored_vertex_data[base + 1], stored_vertex_data[base + 2])
        };

        let iterations = unsafe { super::epsilons::PLANE_REFIT_ITERATIONS } as u32;
        for _ in 0..iterations {
            self.planes.clear();
            self.plane_data = vec![u32::MAX; self.num_faces as usize];

            let mut centroid = DVec3::ZERO;
            for i in 0..self.num_faces as usize {
                let f = self.get_face(i);
                let a = self.get_point(f.i0 as usize);
                let b = self.get_point(f.i1 as usize);
                let c = self.get_point(f.i2 as usize);
                centroid += (a + b + c) / 3.0;
            }
            centroid /= self.num_faces as f64;

            for i in 0..self.num_faces as usize {
                let f = self.get_face(i);
                let a = self.get_point(f.i0 as usize);
                let b = self.get_point(f.i1 as usize);
                let c = self.get_point(f.i2 as usize);
                let mut norm = DVec3::ZERO;
                if compute_safe_normal(a, b, c, &mut norm, EPS_SMALL) {
                    let da = norm.dot(a - centroid);
                    let db = norm.dot(b - centroid);
                    let dc = norm.dot(c - centroid);
                    let id = self.add_plane(norm, (da + db + dc) / 3.0);
                    self.plane_data[i] = id as u32;
                    self.has_planes = true;
                }
            }

            for i in 0..self.num_faces as usize {
                let f = self.get_face(i);
                if f.p_id != u32::MAX {
                    let plane = self.planes[f.p_id as usize].clone();
                    let a = self.get_point(f.i0 as usize);
                    let b = self.get_point(f.i1 as usize);
                    let c = self.get_point(f.i2 as usize);

                    let da = plane.normal.dot(a - centroid);
                    let db = plane.normal.dot(b - centroid);
                    let dc = plane.normal.dot(c - centroid);

                    let da = plane.distance - da;
                    let db = plane.distance - db;
                    let dc = plane.distance - dc;

                    let mut va = a + plane.normal * da;
                    let mut vb = b + plane.normal * db;
                    let mut vc = c + plane.normal * dc;

                    let mut dsa = get_stored_point(f.i0 as usize) - va;
                    let mut dsb = get_stored_point(f.i1 as usize) - vb;
                    let mut dsc = get_stored_point(f.i2 as usize) - vc;

                    let mut fa = dsa.length() / RECONSTRUCT_TOLERANCE;
                    let mut fb = dsb.length() / RECONSTRUCT_TOLERANCE;
                    let mut fc = dsc.length() / RECONSTRUCT_TOLERANCE;

                    if fa > 1.0 {
                        fa = dsa.length() / fa;
                        dsa = dsa.normalize() * fa;
                        va += dsa;
                    }
                    if fb > 1.0 {
                        fb = dsb.length() / fb;
                        dsb = dsb.normalize() * fb;
                        vb += dsb;
                    }
                    if fc > 1.0 {
                        fc = dsc.length() / fc;
                        dsc = dsc.normalize() * fc;
                        vc += dsc;
                    }

                    self.set_point(va.x, va.y, va.z, f.i0 as usize);
                    self.set_point(vb.x, vb.y, vb.z, f.i1 as usize);
                    self.set_point(vc.x, vc.y, vc.z, f.i2 as usize);
                }
            }
        }

        for i in 0..self.num_faces as usize {
            let f = self.get_face(i);
            if f.p_id != u32::MAX {
                let a = self.get_point(f.i0 as usize);
                let plane = &mut self.planes[f.p_id as usize];
                plane.distance = plane.normal.dot(a);
            }
        }
    }

    pub fn add_geometry(&mut self, geom: Geometry) {
        for i in 0..geom.num_faces as usize {
            let f = geom.get_face(i);
            let a = geom.get_point(f.i0 as usize);
            let b = geom.get_point(f.i1 as usize);
            let c = geom.get_point(f.i2 as usize);
            self.add_face_points(a, b, c, f.p_id);
        }

        let plane_data_offset = geom.planes.len() as u32;
        for p in geom.plane_data {
            self.plane_data.push(plane_data_offset + p);
        }

        self.planes.extend(geom.planes);
    }
}
