#[derive(Debug, Default)]
pub struct MouseState {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub dscroll: f64,
}

impl MouseState {
    pub fn update_motion(&mut self, axis: u32, value: f64) {
        match axis {
            0 => self.dx += value,
            1 => self.dy += value,
            3 => self.dscroll += value,
            _ => (),
        }
    }

    pub fn clear_motion(&mut self) {
        self.dx = 0.0;
        self.dy = 0.0;
        self.dscroll = 0.0;
    }
}
