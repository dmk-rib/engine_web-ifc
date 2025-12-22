//! Alignment curve generation.

use glam::DVec3;

use super::buffers::Buffers;
use super::curve::Curve;
use super::utils::convert_2d_alignments_to_3d;

#[derive(Clone, Debug, Default)]
pub struct Alignment {
    pub segments: u16,
    pub horizontal: Vec<DVec3>,
    pub vertical: Vec<DVec3>,
    pub points: Vec<DVec3>,
}

impl Alignment {
    pub fn set_values(&mut self, horizontal: Vec<f64>, vertical: Vec<f64>) {
        self.horizontal.clear();
        for chunk in horizontal.chunks(3) {
            if chunk.len() == 3 {
                self.horizontal.push(DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
        self.vertical.clear();
        for chunk in vertical.chunks(3) {
            if chunk.len() == 3 {
                self.vertical.push(DVec3::new(chunk[0], chunk[1], chunk[2]));
            }
        }
    }

    pub fn get_buffers(&mut self) -> Buffers {
        let mut buffers = Buffers::default();
        self.points.clear();
        let points3d = convert_2d_alignments_to_3d(&self.horizontal, &self.vertical);
        self.points.extend(points3d.iter().copied());
        for point in &self.points {
            buffers.add_point(*point);
        }
        buffers
    }
}
