use std::{
    cell::RefCell,
    fs::{File, OpenOptions},
    io::Write,
    rc::Rc,
};

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

#[derive(Debug)]
pub enum Error {
    FileOpenError,
    FileWriteError,
}

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

#[derive(Debug, Clone)]
pub struct GraphikBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl GraphikBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
}

pub fn get_center(canvas: usize, object: usize) -> i32 {
    ((canvas - object) / 2) as i32
}

pub fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[derive(Debug)]
pub struct GraphikBuilder {
    pub buffer: Rc<RefCell<GraphikBuffer>>,
}

impl GraphikBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: Rc::new(RefCell::new(GraphikBuffer::new(width, height))),
        }
    }

    pub fn fill(&mut self, color: u32) {
        self.buffer
            .borrow_mut()
            .buffer
            .iter_mut()
            .for_each(|pixel| {
                *pixel = color;
            });
    }

    pub fn rect_fill(&mut self, rect: &mut GraphikRect) {
        let mut buffer = self.buffer.borrow_mut();
        if rect.center {
            let x0 = get_center(buffer.width, rect.width);
            let y0 = get_center(buffer.height, rect.height);
            rect.origin(x0, y0);
        }

        for dy in 0..rect.height {
            let y = rect.y0 as usize + dy;
            if y < buffer.height {
                for dx in 0..rect.width {
                    let x = rect.x0 as usize + dx;
                    if x < buffer.width {
                        let bufwid = buffer.width;
                        buffer.buffer[y * bufwid + x] = rect.color;
                    }
                }
            }
        }
    }

    pub fn circle_fill(&mut self, circle: &mut GraphikCircle) {
        let mut buffer = self.buffer.borrow_mut();
        if circle.center {
            let x0 = (buffer.width / 2) as i32;
            let y0 = (buffer.height / 2) as i32;
            circle.origin(x0, y0);
        }

        let x1 = circle.x0 - circle.radius as i32;
        let y1 = circle.y0 - circle.radius as i32;
        let x2 = circle.x0 + circle.radius as i32;
        let y2 = circle.y0 + circle.radius as i32;
        for y in y1..y2 {
            if 0 <= y && y < buffer.height as i32 {
                for x in x1..x2 {
                    if 0 <= x && x < buffer.width as i32 {
                        let dx = x - circle.x0;
                        let dy = y - circle.y0;
                        if (dx * dx + dy * dy) <= (circle.radius * circle.radius) as i32 {
                            let bufwid = buffer.width;
                            buffer.buffer[y as usize * bufwid + x as usize] = circle.color;
                        }
                    }
                }
            }
        }
    }

    pub fn line_draw(&mut self, line: &mut GraphikLine) {
        let mut buffer = self.buffer.borrow_mut();
        self.process_line_vertices(line, buffer.width, buffer.height);

        let mut x0 = line.x0;
        let mut y0 = line.y0;
        let x1 = line.x1;
        let y1 = line.y1;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        while x0 != x1 || y0 != y1 {
            if 0 <= x0 && x0 < buffer.width as i32 && 0 <= y0 && y0 < buffer.height as i32 {
                let bufwid = buffer.width;
                buffer.buffer[y0 as usize * bufwid + x0 as usize] = line.color;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn save_as_ppm(&self, file_path: &str) -> Result<(), Error> {
        let buffer = self.buffer.borrow();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|err| {
                eprintln!("Failed to open file {}: {}", &file_path, err);
                Error::FileOpenError
            })?;
        self.write_header(&mut file, buffer.width, buffer.height)?;

        for pixel in buffer.buffer.iter() {
            let bytes = [
                (*pixel & 0xff) as u8,
                ((*pixel >> 8) & 0xff) as u8,
                ((*pixel >> 16) & 0xff) as u8,
            ];
            file.write_all(&bytes).map_err(|_| Error::FileWriteError)?;
        }
        Ok(())
    }

    fn write_header(&self, file: &mut File, width: usize, height: usize) -> Result<(), Error> {
        let header = format!("P6\n{} {} 255\n", width, height);
        file.write_all(header.as_bytes())
            .map_err(|_| Error::FileWriteError)?;
        Ok(())
    }

    fn process_line_vertices(&self, line: &mut GraphikLine, width: usize, height: usize) {
        if line.center {
            if line.vertical {
                let center_x = (width / 2) as i32;
                line.vertical(center_x, line.y0, line.y1);
            } else if line.horizontal {
                let center_y = (height / 2) as i32;
                line.horizontal(center_y, line.x0, line.x1);
            }
        }
        // let x1 = self.width as i32 - line.x1;
        // let y1 = self.height as i32 - line.y1;
        // line.end(x1, y1);
    }
}
