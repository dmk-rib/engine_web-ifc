//! Rust port of `web-ifc/geometry/operations/boolean-utils/is-inside-boundary.h`.

use std::collections::BTreeSet;

use glam::DVec2;

pub fn is_inside_boundary(
    t1: DVec2,
    t2: DVec2,
    t3: DVec2,
    edges: &BTreeSet<(usize, usize)>,
    projected_points: &[DVec2],
) -> bool {
    let centroid = (t1 + t2 + t3) / 3.0;

    let mut crossings = 0;
    for (start, end) in edges {
        let p1 = projected_points[*start];
        let p2 = projected_points[*end];

        if (p1.y > centroid.y) != (p2.y > centroid.y) {
            let x_intersection = p1.x + (p2.x - p1.x) * (centroid.y - p1.y) / (p2.y - p1.y);
            if x_intersection > centroid.x {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}
