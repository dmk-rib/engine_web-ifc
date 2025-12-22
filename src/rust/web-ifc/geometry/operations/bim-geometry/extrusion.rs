//! Extrusion helpers.

use glam::DVec3;

use super::buffers::Buffers;
use super::geometry::Geometry;
use super::utils::{extrude, extrude_with_profiles};

#[derive(Clone, Debug, Default)]
pub struct Extrusion {
    pub cap: bool,
    pub len: f64,
    pub dir: DVec3,
    pub cutting_plane_pos: DVec3,
    pub cutting_plane_normal: DVec3,
    pub profile: Vec<DVec3>,
    pub holes: Vec<Vec<DVec3>>,
}

impl Extrusion {
    pub fn set_values(
        &mut self,
        profile: Vec<f64>,
        dir: Vec<f64>,
        len: f64,
        cutting_plane_normal: Vec<f64>,
        cutting_plane_pos: Vec<f64>,
        cap: bool,
    ) {
        self.profile.clear();
        for chunk in profile.chunks(3) {
            if chunk.len() == 3 {
                self.profile.push(DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
        if dir.len() == 3 {
            self.dir = DVec3::new(dir[0], dir[1], dir[2]);
        }
        self.len = len;
        self.cap = cap;
        if cutting_plane_pos.len() == 3 {
            self.cutting_plane_pos = DVec3::new(cutting_plane_pos[0], cutting_plane_pos[1], cutting_plane_pos[2]);
        }
        if cutting_plane_normal.len() == 3 {
            self.cutting_plane_normal = DVec3::new(
                cutting_plane_normal[0],
                cutting_plane_normal[1],
                cutting_plane_normal[2],
            );
        }
    }

    pub fn set_holes(&mut self, hole: Vec<f64>) {
        let mut points = Vec::new();
        for chunk in hole.chunks(3) {
            if chunk.len() == 3 {
                points.push(DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
        self.holes.push(points);
    }

    pub fn clear_holes(&mut self) {
        self.holes.clear();
    }

    pub fn get_buffers(&self) -> Buffers {
        let mut buffers = Buffers::default();
        let geom: Geometry = if !self.cap {
            extrude(&self.profile, self.dir, self.len)
        } else {
            let mut profiles = Vec::with_capacity(1 + self.holes.len());
            profiles.push(self.profile.clone());
            profiles.extend(self.holes.clone());
            extrude_with_profiles(
                profiles,
                self.dir,
                self.len,
                self.cutting_plane_normal,
                self.cutting_plane_pos,
            )
        };

        for r in 0..geom.num_faces {
            let f = geom.get_face(r as usize);
            buffers.add_tri(
                geom.get_point(f.i0 as usize),
                geom.get_point(f.i1 as usize),
                geom.get_point(f.i2 as usize),
            );
        }

        buffers
    }
}
