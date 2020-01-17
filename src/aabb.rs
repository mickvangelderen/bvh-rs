use crate::vector::*;
use crate::axis::*;

// pub trait Infinity {
//     const INFINITY: Self;
// }

// impl Infinity for f32 {
//     const INFINITY: Self = std::f32::INFINITY;
// }

// impl Infinity for f64 {
//     const INFINITY: Self = std::f64::INFINITY;
// }

#[derive(Debug, Clone, Copy)]
pub struct AABB3 {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl Default for AABB3 {
    fn default() -> Self {
        Self {
            min: Vector3::from_scalar(std::f32::INFINITY),
            max: Vector3::from_scalar(-std::f32::INFINITY),
        }
    }
}

impl AABB3 {
    pub fn merge(mut self, other: Self) -> Self {
        self.min.ew_min_assign(other.min);
        self.max.ew_max_assign(other.max);
        self
    }

    pub fn from_point(point: Vector3<f32>) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn from_points<I>(points: I) -> Option<Self> where I: IntoIterator<Item = Vector3<f32>> {
        let mut points = points.into_iter();
        points.next().map(|first| {
            let mut aabb = Self::from_point(first);
            for point in points {
                aabb.include_point(point);
            }
            aabb
        })
    }

    pub fn include_point(&mut self, point: Vector3<f32>) {
        self.min.ew_min_assign(point);
        self.max.ew_max_assign(point);
    }

    pub fn contains_point(&self, point: Vector3<f32>) -> bool {
        (point.x >= self.min.x && point.x < self.max.x)
            && (point.y >= self.min.y && point.y < self.max.y)
            && (point.z >= self.min.z && point.z < self.max.z)
    }

    pub fn split(&self, axis: Axis3) -> (Self, Self) {
        let split = (self.max + self.min)[axis] * 0.5;

        (
            Self {
                min: self.min,
                max: {
                    let mut max = self.max;
                    max[axis] = split;
                    max
                },
            },
            Self {
                min: {
                    let mut min = self.min;
                    min[axis] = split;
                    min
                },
                max: self.max,
            },
        )
    }
}
