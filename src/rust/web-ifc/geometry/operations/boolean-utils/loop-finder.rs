//! Rust port of `web-ifc/geometry/operations/boolean-utils/loop-finder.h`.

use std::collections::{BTreeSet, HashMap, HashSet};

use glam::DVec2;

use super::eps::{EPS_BIG, MESSAGES};
use super::math::{comparable_angle, do_line_segments_intersect};

#[derive(Clone, Debug, Default)]
pub struct Loop {
    pub loop_: Vec<usize>,
}

impl Loop {
    pub fn has_point(&self, p: usize) -> bool {
        self.loop_.iter().any(|value| *value == p)
    }
}

pub fn is_point_inside_loop(points: &[DVec2], loop_: &Loop, pt: DVec2) -> bool {
    let line = DVec2::new(10000.0, 0.0);
    let mut distances = Vec::new();

    for i in 0..loop_.loop_.len() {
        let prev = loop_.loop_[i];
        let cur = loop_.loop_[(i + 1) % loop_.loop_.len()];

        let a = points[prev];
        let b = points[cur];

        let dir = (a - b).normalize();
        let colinear = dir.dot(line).abs() > (1.0 - EPS_BIG);
        if colinear {
            continue;
        }

        let result = do_line_segments_intersect(pt, pt + line, a, b, EPS_BIG);
        if result.isect {
            distances.push(result.dist);
        }
    }

    distances.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let mut count = 0usize;
    let mut cur = -1.0;
    for dist in distances {
        if (dist - cur).abs() > EPS_BIG {
            count += 1;
        }
        cur = dist;
    }

    count % 2 == 1
}

pub fn a_inside_b(points: &[DVec2], a: &Loop, b: &Loop) -> bool {
    let mut non_shared = None;
    for index in &a.loop_ {
        if !b.has_point(*index) {
            non_shared = Some(*index);
        }
    }

    if non_shared.is_none() {
        return true;
    }

    let pt = points[non_shared.unwrap()];
    is_point_inside_loop(points, b, pt)
}

pub fn find_outer_loop(points: &[DVec2], edges: &BTreeSet<(usize, usize)>, forward: bool) -> Loop {
    let mut result = Loop::default();
    if edges.is_empty() {
        return result;
    }

    let mut connections: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (first, second) in edges {
        connections.entry(*first).or_default().insert(*second);
        connections.entry(*second).or_default().insert(*first);
    }

    let mut cur = edges.iter().next().unwrap().0;
    let mut prev = edges.iter().next().unwrap().1;
    if forward {
        std::mem::swap(&mut cur, &mut prev);
    }
    let start = prev;

    let mut loop_vec = Vec::new();
    let mut visited = vec![false; points.len()];

    loop {
        if visited[cur] {
            if MESSAGES {
                println!("Loop in findLoop ... how ironic ");
            }
            return Loop::default();
        }

        visited[cur] = true;
        loop_vec.push(cur);
        if cur == start {
            break;
        }

        let mut nbs = connections.get(&cur).cloned().unwrap_or_default();
        nbs.remove(&prev);
        nbs.remove(&cur);

        if nbs.is_empty() {
            if MESSAGES {
                println!("Found vert without neighbours other than the origin of this search!");
            }
            loop_vec.clear();
            return result;
        }

        let mut max_sign = f64::MIN;
        let mut max_nb = 0usize;

        let a = points[prev];
        let b = points[cur];

        for nb in nbs {
            let p = points[nb];
            let sign = comparable_angle(p, a, b);
            if sign > max_sign {
                max_sign = sign;
                max_nb = nb;
            }
        }

        prev = cur;
        cur = max_nb;
    }

    result.loop_ = loop_vec;
    result
}

pub fn find_largest_edge_loop(points: &[DVec2], edges: &BTreeSet<(usize, usize)>) -> Loop {
    let l1 = find_outer_loop(points, edges, false);
    let l2 = find_outer_loop(points, edges, true);

    if a_inside_b(points, &l1, &l2) {
        l2
    } else {
        l1
    }
}
