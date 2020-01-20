use cgmath::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct RayPrecomputed {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    pub inv_direction: Vector3<f32>,
}

impl From<Ray> for RayPrecomputed {
    fn from(ray: Ray) -> Self {
        let Ray { origin, direction } = ray;
        RayPrecomputed {
            origin,
            direction,
            inv_direction: 1.0 / direction,
            // inv_direction: Vector3 {
            //     x: 1.0 / direction.x,
            //     y: 1.0 / direction.y,
            //     z: 1.0 / direction.z,
            // },
        }
    }
}
