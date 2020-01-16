use num_traits::{cast, Float};

#[derive(Debug, Copy, Clone)]
pub struct Range3<T> {
    pub x0: T,
    pub x1: T,
    pub y0: T,
    pub y1: T,
    pub z0: T,
    pub z1: T,
}

impl<T: Float> Range3<T> {
    #[inline]
    pub fn dx(&self) -> T {
        self.x1 - self.x0
    }

    #[inline]
    pub fn dy(&self) -> T {
        self.y1 - self.y0
    }

    #[inline]
    pub fn dz(&self) -> T {
        self.z1 - self.z0
    }

    #[inline]
    pub fn cast<U>(self) -> Option<Range3<U>>
    where
        U: Float,
    {
        Some(Range3 {
            x0: cast(self.x0)?,
            x1: cast(self.x1)?,
            y0: cast(self.y0)?,
            y1: cast(self.y1)?,
            z0: cast(self.z0)?,
            z1: cast(self.z1)?,
        })
    }
}
