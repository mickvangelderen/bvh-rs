use crate::axis::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: num_traits::identities::Zero,
{
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T> From<[T; 3]> for Vector3<T>
where
    T: Copy,
{
    fn from(array: [T; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<T> Into<[T; 3]> for Vector3<T>
where
    T: Copy,
{
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> std::ops::Add for Vector3<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> std::ops::AddAssign for Vector3<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> std::ops::Sub for Vector3<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> std::ops::SubAssign for Vector3<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> std::ops::Mul<T> for Vector3<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T> std::ops::MulAssign<T> for Vector3<T>
where
    T: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T> std::ops::Div<T> for Vector3<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T> std::ops::DivAssign<T> for Vector3<T>
where
    T: std::ops::DivAssign + Copy,
{
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl<T> Vector3<T>
where
    T: PartialOrd,
{
    pub fn ew_min_assign(&mut self, other: Vector3<T>) {
        if other.x < self.x {
            self.x = other.x
        }
        if other.y < self.y {
            self.y = other.y
        }
        if other.z < self.z {
            self.z = other.z
        }
    }

    pub fn ew_max_assign(&mut self, other: Vector3<T>) {
        if other.x > self.x {
            self.x = other.x
        }
        if other.y > self.y {
            self.y = other.y
        }
        if other.z > self.z {
            self.z = other.z
        }
    }
}

impl<T> std::ops::Index<Axis3> for Vector3<T> {
    type Output = T;

    fn index(&self, index: Axis3) -> &Self::Output {
        match index {
            Axis3::X => &self.x,
            Axis3::Y => &self.y,
            Axis3::Z => &self.z,
        }
    }
}

impl<T> std::ops::IndexMut<Axis3> for Vector3<T> {
    fn index_mut(&mut self, index: Axis3) -> &mut Self::Output {
        match index {
            Axis3::X => &mut self.x,
            Axis3::Y => &mut self.y,
            Axis3::Z => &mut self.z,
        }
    }
}
