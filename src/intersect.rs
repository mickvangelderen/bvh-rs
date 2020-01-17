use cgmath::*;

use crate::aabb::*;
use crate::ray::*;

pub fn ray_versus_aabb(ray: Ray, aabb: AABB3) -> Option<f32> {
    let frac_1_direction = [
        1.0 / ray.direction.x,
        1.0 / ray.direction.y,
        1.0 / ray.direction.z,
    ];

    let t1 = {
        let (n, f) = if ray.direction.x >= 0.0 {
            (aabb.min.x, aabb.max.x)
        } else {
            (aabb.max.x, aabb.min.x)
        };
        (
            (n - ray.origin.x) * frac_1_direction[0],
            (f - ray.origin.x) * frac_1_direction[0],
        )
    };

    let t2 = {
        let (n, f) = if ray.direction.y >= 0.0 {
            (aabb.min.y, aabb.max.y)
        } else {
            (aabb.max.y, aabb.min.y)
        };
        (
            (n - ray.origin.y) * frac_1_direction[1],
            (f - ray.origin.y) * frac_1_direction[1],
        )
    };

    if t1.0 > t2.1 || t2.0 > t1.1 {
        return None;
    }

    let t1 = (
        if t1.0 < t2.0 { t1.0 } else { t2.0 },
        if t1.1 > t2.1 { t1.1 } else { t2.1 },
    );

    let t2 = {
        let (n, f) = if ray.direction.z >= 0.0 {
            (aabb.min.z, aabb.max.z)
        } else {
            (aabb.max.z, aabb.min.z)
        };
        (
            (n - ray.origin.z) * frac_1_direction[2],
            (f - ray.origin.z) * frac_1_direction[2],
        )
    };

    if t1.0 > t2.1 || t2.0 > t1.1 {
        return None;
    }

    let t1 = (
        if t1.0 < t2.0 { t1.0 } else { t2.0 },
        if t1.1 > t2.1 { t1.1 } else { t2.1 },
    );

    if t1.0 >= 0.0 {
        return Some(t1.0);
    }

    if t1.1 >= 0.0 {
        return Some(t1.1);
    }

    None
}

pub struct TriangleIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

pub fn ray_versus_triangle(ray: Ray, triangle: [Point3<f32>; 3]) -> Option<TriangleIntersection> {
    const E: f32 = 0.0000001;

    let e01 = triangle[1] - triangle[0];
    let e02 = triangle[2] - triangle[0];

    let pvec = ray.direction.cross(e02);
    let det = e01.dot(pvec);

    if det < E {
        // If the det is negative, the triangle is back-facing.
        // If the det is close to zero, the ray is close to parallel.
        // Parallel.
        return None;
    }

    let frac_1_det = 1.0 / det;

    let tvec = ray.origin - triangle[0];
    let u = tvec.dot(pvec) * frac_1_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = tvec.cross(e01);
    let v = ray.direction.dot(qvec) * frac_1_det;
    if v < 0.0 || v > 1.0 {
        return None;
    }

    let w = 1.0 - (u + v);
    if w < 0.0 {
        return None;
    }

    let t = e02.dot(qvec) * frac_1_det;

    Some(TriangleIntersection { t, u, v, w })
}
