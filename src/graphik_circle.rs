#[derive(Debug, Clone, Copy)]
pub struct GraphikCircle {
    pub x0: i32,
    pub y0: i32,
    pub radius: usize,
    pub color: u32,
    pub center: bool,
}

impl GraphikCircle {
    pub fn new(radius: usize) -> Self {
        Self {
            x0: 0,
            y0: 0,
            radius,
            color: 0xffffffff,
            center: false,
        }
    }

    pub fn radius(&mut self, radius: usize) -> Self {
        self.radius = radius;
        *self
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

