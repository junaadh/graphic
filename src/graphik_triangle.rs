use std::mem;

use crate::constants::Colors;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GraphikTriangle {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32,
    pub color: u32,
}

impl GraphikTriangle {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            x3: 0,
            y3: 0,
            color: Colors::Background as u32,
        }
    }

    pub fn first(&mut self, x: i32, y: i32) -> Self {
        self.x1 = x;
        self.y1 = y;
        *self
    }

    pub fn second(&mut self, x: i32, y: i32) -> Self {
        self.x2 = x;
        self.y2 = y;
        *self
    }

    pub fn third(&mut self, x: i32, y: i32) -> Self {
        self.x3 = x;
        self.y3 = y;
        *self
    }

    pub fn color(&mut self, color: u32) -> Self {
        self.color = color;
        *self
    }

    pub fn sort(&mut self) -> Self {
        if self.y1 > self.y2 {
            mem::swap(&mut self.y1, &mut self.y2);
            mem::swap(&mut self.x1, &mut self.x2);
        }

        if self.y2 > self.y3 {
            mem::swap(&mut self.y2, &mut self.y3);
            mem::swap(&mut self.x2, &mut self.x3);
        }

        if self.y1 > self.y2 {
            mem::swap(&mut self.y1, &mut self.y2);
            mem::swap(&mut self.x1, &mut self.x2);
        }
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::GraphikTriangle;

    #[test]
    fn sort_1() {
        let triangle = GraphikTriangle::new()
            .first(69, 1)
            .second(690, 2)
            .third(6900, 3);
        let triangle2 = triangle.clone().sort();
        assert_eq!(triangle, triangle2);
        assert_eq!(triangle.y1, triangle2.y1);
        assert_eq!(triangle.y2, triangle2.y2);
        assert_eq!(triangle.y3, triangle2.y3);
    }

    #[test]
    fn sort_2() {
        let triangle = GraphikTriangle::new()
            .first(69, 2)
            .second(690, 1)
            .third(6900, 3);
        let triangle2 = triangle.clone().sort();
        assert_ne!(triangle, triangle2);
        assert_ne!(triangle.y1, triangle2.y1);
        assert_ne!(triangle.y2, triangle2.y2);
        assert_eq!(triangle.y3, triangle2.y3);
        assert_eq!(1, triangle2.y1);
        assert_eq!(2, triangle2.y2);
    }

    #[test]
    fn sort_3() {
        let triangle = GraphikTriangle::new()
            .first(69, 3)
            .second(690, 1)
            .third(6900, 2);
        let triangle2 = triangle.clone().sort();
        assert_ne!(triangle, triangle2);
        assert_ne!(triangle.y1, triangle2.y1);
        assert_ne!(triangle.y2, triangle2.y2);
        assert_ne!(triangle.y3, triangle2.y3);
        assert_eq!(1, triangle2.y1);
        assert_eq!(2, triangle2.y2);
        assert_eq!(3, triangle2.y3);
    }
}
