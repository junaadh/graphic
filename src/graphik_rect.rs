#[derive(Debug, Clone, Copy)]
pub struct GraphikRect {
    pub x0: i32,
    pub y0: i32,
    pub width: usize,
    pub height: usize,
    pub color: u32,
    pub center: bool,
}

impl GraphikRect {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            x0: 0,
            y0: 0,
            width,
            height,
            color: 0xffffffff,
            center: false,
        }
    }

    pub fn origin(&mut self, x0: i32, y0: i32) -> Self {
        self.x0 = x0;
        self.y0 = y0;
        *self
    }

    pub fn center(&mut self, bool: bool) -> Self {
        self.center = bool;
        *self
    }

    pub fn color(&mut self, color: u32) -> Self {
        self.color = color;
        *self
    }
}
