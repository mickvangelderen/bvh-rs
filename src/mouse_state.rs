#[derive(Debug, Default)]
pub struct MouseState {
    pub dx: f64,
    pub dy: f64,
    pub dscroll: f64,
}

impl MouseState {
    pub fn update(&mut self, axis: u32, value: f64) {
        match axis {
            0 => self.dx += value,
            1 => self.dy += value,
            3 => self.dscroll += value,
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }
}
