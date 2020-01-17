use cgmath::*;

use crate::ray::*;
use crate::aabb::*;

pub fn ray_versus_aabb(ray: Ray, aabb: AABB3) -> Option<Point3<f32>> {
    let planes = Point3 {
        x: if ray.direction.x > 0.0 { aabb.min.x } else { aabb.max.x },
        y: if ray.direction.y > 0.0 { aabb.min.y } else { aabb.max.y },
        z: if ray.direction.z > 0.0 { aabb.min.z } else { aabb.max.z },
    };

    for &(a0, a1, a2) in [
        (0, 1, 2),
        (1, 2, 0),
        (2, 0, 1),
    ].iter() {
        let t = (planes[a0] - ray.origin[a0]) / ray.direction[a0];
        let p1 = ray.origin[a1] + t * ray.direction[a1];
        let p2 = ray.origin[a2] + t * ray.direction[a2];
        if p1 >= aabb.min[a1] && p1 < aabb.max[a1] && p2 >= aabb.min[a2] && p2 < aabb.max[a2] {
            // Intersect plane
            let mut p = Point3::origin();
            p[a0] = planes[a0];
            p[a1] = p1;
            p[a2] = p2;
            return Some(p);
        }
    }

    None
}

pub fn ray_versus_triangle(ray: Ray, triangle: [Point3<f32>; 3]) -> Option<Point3<f32>> {
   unimplemented!()
}
