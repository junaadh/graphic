use crate::constants::{HEIGHT, WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct GraphikLine {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub color: u32,
    pub horizontal: bool,
    pub vertical: bool,
    pub center: bool,
}

impl GraphikLine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            x0: 0,
            y0: 0,
            x1: WIDTH as i32,
            y1: HEIGHT as i32,
            color: 0,
            horizontal: false,
            vertical: false,
            center: false,
        }
    }

    pub fn start(&mut self, x0: i32, y0: i32) -> Self {
        self.x0 = x0;
        self.y0 = y0;
        *self
    }

    pub fn end(&mut self, x1: i32, y1: i32) -> Self {
        self.x1 = x1;
        self.y1 = y1;
        *self
    }

    pub fn horizontal(&mut self, y: i32, x0: i32, x1: i32) -> Self {
        self.horizontal = true;
        self.vertical = false;
        self.center = false;
        self.y0 = y;
        self.y1 = y;
        self.x0 = x0;
        self.x1 = x1;
        *self
    }

    pub fn vertical(&mut self, x: i32, y0: i32, y1: i32) -> Self {
        self.vertical = true;
        self.horizontal = false;
        self.center = false;
        self.x0 = x;
        self.x1 = x;
        self.y0 = y0;
        self.y1 = y1;
        *self
    }

    pub fn horizontal_center(&mut self, x0: i32, x1: i32) -> Self {
        self.horizontal = true;
        self.vertical = false;
        self.center = true;
        self.x0 = x0;
        self.x1 = x1;
        *self
    }

    pub fn vertical_center(&mut self, y0: i32, y1: i32) -> Self {
        self.vertical = true;
        self.horizontal = false;
        self.center = true;
        self.y0 = y0;
        self.y1 = y1;
        *self
    }

    pub fn color(&mut self, color: u32) -> Self {
        self.color = color;
        *self
    }
}
