#[derive(Clone, Copy, Debug)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub buttons: u16,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            buttons: 0,
        }
    }
}
