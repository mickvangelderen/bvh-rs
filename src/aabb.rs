use crate::vector::*;
use crate::axis::*;

#[derive(Debug, Clone, Copy)]
pub struct AABB3 {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB3 {
    pub fn from_point(point: Vector3<f32>) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn include_point(&mut self, point: Vector3<f32>) {
        self.min.ew_min_assign(point);
        self.max.ew_max_assign(point);
    }

    pub fn contains_point(&self, point: Vector3<f32>) -> bool {
        (point.x >= self.min.x && point.x <= self.max.x)
            && (point.y >= self.min.y && point.y <= self.max.y)
            && (point.z >= self.min.z && point.z <= self.max.z)
    }

    pub fn split(&self) -> (Self, Self) {
        let delta = self.max - self.min;

        let largest_component_index = if delta.x > delta.y {
            if delta.x > delta.z {
                Axis3::X
            } else {
                Axis3::Z
            }
        } else {
            if delta.y > delta.z {
                Axis3::Y
            } else {
                Axis3::Z
            }
        };

        let split = (self.max + self.min)[largest_component_index] * 0.5;

        (
            Self {
                min: self.min,
                max: {
                    let mut max = self.max;
                    max[largest_component_index] = split;
                    max
                },
            },
            Self {
                min: {
                    let mut min = self.min;
                    min[largest_component_index] = split;
                    min
                },
                max: self.max,
            },
        )
    }
}
