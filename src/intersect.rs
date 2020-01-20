use cgmath::*;

use crate::aabb::*;
use crate::ray::*;

pub fn ray_versus_aabb(ray: Ray, aabb: AABB3) -> Option<f32> {
    #[inline]
    fn min(a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn max(a: f32, b: f32) -> f32 {
        if a > b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn compute_t(ray: Ray, aabb: AABB3, axis: usize) -> (f32, f32) {
        let (p0, p1) = if ray.direction[axis] >= 0.0 {
            (aabb.min[axis], aabb.max[axis])
        } else {
            (aabb.max[axis], aabb.min[axis])
        };
        (
            (p0 - ray.origin[axis]) / ray.direction[axis],
            (p1 - ray.origin[axis]) / ray.direction[axis],
        )
    }

    let (mut t0, mut t1) = compute_t(ray, aabb, 0);

    for axis in 1..3 {
        let (n0, n1) = compute_t(ray, aabb, axis);

        t0 = max(t0, n0);
        t1 = min(t1, n1);

        if t0 > t1 {
            return None;
        }
    }

    if t0 > 0.0 {
        Some(t0)
    } else if t1 > 0.0 {
        Some(t1)
    } else {
        None
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_vs_aabb() {
        let aabb = AABB3 {
            min: crate::vector::Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            max: crate::vector::Vector3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            },
        };

        let ray = Ray {
            origin: Point3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            .normalize(),
        };

        assert_eq!(Some(0.0), ray_versus_aabb(ray, aabb));
    }
}
