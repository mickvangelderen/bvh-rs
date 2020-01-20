use cgmath::*;

use crate::aabb::*;
use crate::ray::*;

/// https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/
pub fn ray_versus_aabb(ray: RayPrecomputed, aabb: AABB3) -> bool {
    #[inline]
    fn min(a: f32, b: f32) -> f32 {
        if a < b { a } else { b }
    }

    #[inline]
    fn max(a: f32, b: f32) -> f32 {
        if a > b { a } else { b }
    }

    let t1 = (aabb.min.x - ray.origin.x)*ray.inv_direction.x;
    let t2 = (aabb.max.x - ray.origin.x)*ray.inv_direction.x;

    let mut tmin = min(t1, t2);
    let mut tmax = max(t1, t2);

    for axis in 1..3 {
        let t1 = (aabb.min[axis] - ray.origin[axis])*ray.inv_direction[axis];
        let t2 = (aabb.max[axis] - ray.origin[axis])*ray.inv_direction[axis];

        tmin = max(tmin, min(t1, t2));
        tmax = min(tmax, max(t1, t2));
    }

    tmax > max(tmin, 0.0)
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
