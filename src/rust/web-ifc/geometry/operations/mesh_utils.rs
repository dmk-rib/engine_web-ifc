//! Rust port of `web-ifc/geometry/operations/mesh_utils.h`.

use glam::{DMat4, DVec3};

use crate::web_ifc::geometry::nurbs::Nurbs;
use crate::web_ifc::geometry::operations::bim_geometry::utils as bim_utils;
use crate::web_ifc::geometry::operations::bim_geometry::utils::CONST_PI;
use crate::web_ifc::geometry::operations::geometryutils;
use crate::web_ifc::geometry::representation::geometry::{IfcBound3D, IfcProfile, IfcSurface};
use crate::web_ifc::geometry::representation::ifc_geometry::IfcGeometry;

pub fn vector_to_angle_3d(x: f64, y: f64) -> f64 {
    let dd = (x * x + y * y).sqrt();
    if dd.abs() < bim_utils::EPS_MINISCULE {
        return 0.0;
    }
    let xx = x / dd;
    let yy = y / dd;

    let mut angle = xx.asin();
    let mut cosv = angle.cos();
    if (yy - cosv).abs() > 1e-5 {
        angle = yy.acos();
        let mut sinv = angle.sin();
        cosv = angle.cos();
        if (yy - cosv).abs() > 1e-5 || (xx - sinv).abs() > 1e-5 {
            angle = angle + (CONST_PI - angle) * 2.0;
            sinv = angle.sin();
            cosv = angle.cos();
            if (yy - cosv).abs() > 1e-5 || (xx - sinv).abs() > 1e-5 {
                angle += CONST_PI;
            }
        }
    }

    (angle / (2.0 * CONST_PI)) * 360.0
}

pub fn triangulate_revolution(
    geometry: &mut IfcGeometry,
    bounds: &[IfcBound3D],
    surface: &IfcSurface,
    num_rots: f64,
) {
    let mut transform = DMat4::IDENTITY;
    transform.w_axis = surface.revolution_surface.direction.w_axis;
    transform.x_axis = surface.revolution_surface.direction.x_axis.normalize();
    transform.y_axis = surface.revolution_surface.direction.y_axis.normalize();
    transform.z_axis = surface.revolution_surface.direction.z_axis.normalize();

    let mut max_team = 0u16;
    for bound in bounds {
        for idx in &bound.curve.indices {
            max_team = max_team.max(*idx);
        }
    }

    let mut bounding_groups: Vec<Vec<DVec3>> = Vec::new();
    for r in 0..=max_team {
        let mut group = Vec::new();
        for bound in bounds {
            for (j, pt) in bound.curve.base.points.iter().enumerate() {
                if bound.curve.indices.get(j).copied().unwrap_or(0) == r {
                    group.push(*pt);
                }
            }
        }
        bounding_groups.push(group);
    }

    let mut bounding = Vec::new();
    for group in &bounding_groups {
        let mut acc = DVec3::ZERO;
        let mut count = 0.0;
        for (j, pt) in group.iter().enumerate() {
            if j == group.len().saturating_sub(1) {
                if count > 0.0 {
                    acc /= count;
                    bounding.push(acc);
                }
                acc = *pt;
                count = 1.0;
            } else {
                acc += *pt;
                count += 1.0;
            }
        }
    }

    let mut angle_vec = Vec::new();
    let mut angle_dsp = Vec::new();

    for pt in &bounding {
        let xx = pt.x - transform.w_axis.x;
        let yy = pt.y - transform.w_axis.y;
        let zz = pt.z - transform.w_axis.z;
        let dx = transform.x_axis.truncate().dot(DVec3::new(xx, yy, zz));
        let dy = transform.y_axis.truncate().dot(DVec3::new(xx, yy, zz));
        let mut temp = vector_to_angle_3d(dx, dy);
        while temp < 0.0 {
            temp += 360.0;
        }
        while temp > 360.0 {
            temp -= 360.0;
        }
        angle_vec.push(temp);
    }

    for i in 0..angle_vec.len().saturating_sub(1) {
        if angle_vec[i] - angle_vec[i + 1] > 180.0 {
            angle_dsp.push(360.0 - (angle_vec[i] - angle_vec[i + 1]));
        } else if angle_vec[i] - angle_vec[i + 1] < -180.0 {
            angle_dsp.push(-(angle_vec[i] - angle_vec[i + 1] + 360.0));
        } else {
            angle_dsp.push(angle_vec[i + 1] - angle_vec[i]);
        }
    }

    let mut start_degrees = *angle_vec.first().unwrap_or(&0.0);
    let mut end_degrees = start_degrees;
    let mut temp = start_degrees;
    for delta in angle_dsp {
        temp += delta;
        if end_degrees < temp {
            end_degrees = temp;
        }
        if start_degrees > temp {
            start_degrees = temp;
        }
    }

    let geom = bim_utils::revolution(
        transform,
        start_degrees,
        end_degrees,
        surface.revolution_surface.profile.curve.base.points.clone(),
        num_rots,
    );

    for r in 0..geom.num_faces {
        let f = geom.get_face(r as usize);
        geometry
            .base
            .base
            .add_face_points(geom.get_point(f.i0 as usize), geom.get_point(f.i1 as usize), geom.get_point(f.i2 as usize), f.p_id);
    }
}

pub fn triangulate_cylindrical_surface(
    geometry: &mut IfcGeometry,
    bounds: &[IfcBound3D],
    surface: &IfcSurface,
    num_rots: i32,
) {
    let cent = surface.transformation.w_axis.truncate();
    let vec_x = surface.transformation.x_axis.truncate().normalize();
    let vec_y = surface.transformation.y_axis.truncate().normalize();
    let vec_z = surface.transformation.z_axis.truncate().normalize();

    let mut max_team = 0u16;
    for bound in bounds {
        for idx in &bound.curve.indices {
            max_team = max_team.max(*idx);
        }
    }

    let mut bounding_groups: Vec<Vec<DVec3>> = Vec::new();
    for r in 0..=max_team {
        let mut group = Vec::new();
        for bound in bounds {
            for (j, pt) in bound.curve.base.points.iter().enumerate() {
                if bound.curve.indices.get(j).copied().unwrap_or(0) == r {
                    group.push(*pt);
                }
            }
        }
        bounding_groups.push(group);
    }

    let mut bounding = Vec::new();
    for group in &bounding_groups {
        let mut acc = DVec3::ZERO;
        let mut count = 0.0;
        for (j, pt) in group.iter().enumerate() {
            if j == group.len().saturating_sub(1) {
                if count > 0.0 {
                    acc /= count;
                    bounding.push(acc);
                }
                acc = *pt;
                count = 1.0;
            } else {
                acc += *pt;
                count += 1.0;
            }
        }
    }

    let mut angle_vec = Vec::new();
    let mut angle_dsp = Vec::new();
    for pt in &bounding {
        let xx = pt.x - cent.x;
        let yy = pt.y - cent.y;
        let zz = pt.z - cent.z;
        let dx = vec_x.dot(DVec3::new(xx, yy, zz));
        let dy = vec_y.dot(DVec3::new(xx, yy, zz));
        let mut temp = vector_to_angle_3d(dx, dy);
        while temp < 0.0 {
            temp += 360.0;
        }
        while temp > 360.0 {
            temp -= 360.0;
        }
        angle_vec.push(temp);
    }

    for i in 0..angle_vec.len().saturating_sub(1) {
        if angle_vec[i] - angle_vec[i + 1] > 180.0 {
            angle_dsp.push(360.0 - (angle_vec[i] - angle_vec[i + 1]));
        } else if angle_vec[i] - angle_vec[i + 1] < -180.0 {
            angle_dsp.push(-(angle_vec[i] - angle_vec[i + 1] + 360.0));
        } else {
            angle_dsp.push(angle_vec[i + 1] - angle_vec[i]);
        }
    }

    let mut start_degrees = *angle_vec.first().unwrap_or(&0.0);
    let mut end_degrees = start_degrees;
    let mut temp = start_degrees;
    for delta in angle_dsp {
        temp += delta;
        if end_degrees < temp {
            end_degrees = temp;
        }
        if start_degrees > temp {
            start_degrees = temp;
        }
    }

    let geom = bim_utils::revolve_cylinder(
        DMat4::from_cols(vec_x.extend(0.0), vec_y.extend(0.0), vec_z.extend(0.0), cent.extend(1.0)),
        start_degrees,
        end_degrees,
        -surface.cylinder_surface.radius,
        surface.cylinder_surface.radius,
        num_rots,
        surface.cylinder_surface.radius,
    );

    for r in 0..geom.num_faces {
        let f = geom.get_face(r as usize);
        geometry
            .base
            .base
            .add_face_points(geom.get_point(f.i0 as usize), geom.get_point(f.i1 as usize), geom.get_point(f.i2 as usize), f.p_id);
    }
}

pub fn triangulate_extrusion(geometry: &mut IfcGeometry, bounds: &[IfcBound3D], surface: &IfcSurface) {
    let mut profile = IfcProfile::default();
    if let Some(bound) = bounds.first() {
        profile.curve.base.points = bound.curve.base.points.clone();
    }
    let geom = geometryutils::extrude(
        profile,
        surface.extrusion_surface.direction,
        surface.extrusion_surface.length,
        DVec3::ZERO,
        DVec3::ZERO,
    );
    geometry.merge_geometry(geom.base.base);
}

pub fn triangulate_bspline(geometry: &mut IfcGeometry, bounds: &[IfcBound3D], surface: &IfcSurface, scaling: f64) {
    let mut nurbs = Nurbs::new(geometry, bounds, surface, scaling);
    nurbs.fill_geometry();
}
