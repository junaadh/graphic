use std::{
    fs::{File, OpenOptions},
    io::Write,
    mem,
};

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

#[derive(Debug)]
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

    pub fn fill(mut self, color: u32) -> Self {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
        self
    }

    pub fn rect_fill(mut self, rect: &mut GraphikRect) -> Self {
        if rect.center {
            let x0 = get_center(self.width, rect.width);
            let y0 = get_center(self.height, rect.height);
            rect.origin(x0, y0);
        }

        for dy in 0..rect.height {
            let y = rect.y0 as usize + dy;
            if y < self.height {
                for dx in 0..rect.width {
                    let x = rect.x0 as usize + dx;
                    if x < self.width {
                        self.buffer[y * self.width + x] = rect.color;
                    }
                }
            }
        }
        self
    }

    pub fn circle_fill(mut self, circle: &mut GraphikCircle) -> Self {
        if circle.center {
            let x0 = (self.width / 2) as i32;
            let y0 = (self.height / 2) as i32;
            circle.origin(x0, y0);
        }

        let x1 = circle.x0 - circle.radius as i32;
        let y1 = circle.y0 - circle.radius as i32;
        let x2 = circle.x0 + circle.radius as i32;
        let y2 = circle.y0 + circle.radius as i32;
        for y in y1..y2 {
            if y < self.height as i32 {
                for x in x1..x2 {
                    if x < self.width as i32 {
                        let dx = x - circle.x0;
                        let dy = y - circle.y0;
                        if (dx * dx + dy * dy) <= (circle.radius * circle.radius) as i32 {
                            self.buffer[y as usize * self.width + x as usize] = circle.color;
                        }
                    }
                }
            }
        }

        self
    }

    pub fn save_as_ppm(self, file_path: &str) -> Result<(), Error> {
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
        self.write_header(&mut file)?;

        for pixel in self.buffer.iter() {
            let bytes = [
                (*pixel & 0xff) as u8,
                ((*pixel >> 8) & 0xff) as u8,
                ((*pixel >> 16) & 0xff) as u8,
            ];
            file.write_all(&bytes).map_err(|_| Error::FileWriteError)?;
        }
        Ok(())
    }

    fn write_header(&self, file: &mut File) -> Result<(), Error> {
        let header = format!("P6\n{} {} 255\n", self.width, self.height);
        file.write_all(header.as_bytes())
            .map_err(|_| Error::FileWriteError)?;
        Ok(())
    }
}

pub fn get_center(canvas: usize, object: usize) -> i32 {
    ((canvas - object) / 2) as i32
}

// TODO: to be removed. currently here to compare
pub fn graphik_fill(buffer: &mut [u32], color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

pub fn graphik_save_to_ppm(
    buffer: &[u32],
    width: usize,
    height: usize,
    file_path: String,
) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .map_err(|err| {
            eprintln!("Failed to open file {}: {}", &file_path, err);
            Error::FileOpenError
        })?;
    write_header(&mut file, width, height)?;

    for pixel in buffer.iter() {
        let bytes = [
            (*pixel & 0xff) as u8,
            ((*pixel >> 8) & 0xff) as u8,
            ((*pixel >> 16) & 0xff) as u8,
        ];
        file.write_all(&bytes).map_err(|_| Error::FileWriteError)?;
    }

    Ok(())
}

fn write_header(file: &mut File, width: usize, height: usize) -> Result<(), Error> {
    let header = format!("P6\n{} {} 255\n", width, height);
    file.write_all(header.as_bytes())
        .map_err(|_| Error::FileWriteError)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn graphik_draw_line(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    color: u32,
) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx != 0 {
        let c = y1 - dy * x1 / dx;
        if x1 > x2 {
            mem::swap(&mut x1, &mut x2);
        }
        for x in x1..x2 {
            if 0 <= x && x < width as i32 {
                let mut sy1 = dy * x / dx + c;
                let mut sy2 = dy * (x + 1) / dx + c;
                if sy1 > sy2 {
                    mem::swap(&mut sy1, &mut sy2);
                }
                for y in sy1..sy2 {
                    if 0 <= y && y < height as i32 {
                        buffer[y as usize * width + x as usize] = color;
                    }
                }
            }
        }
    } else {
        let x = x1;
        if 0 <= x && x < width as i32 {
            if y1 > y2 {
                mem::swap(&mut y1, &mut y2);
            }
            for y in y1 as usize..height {
                buffer[y * width + x as usize] = color;
            }
        }
    }
}

// fn swap_int(a: &mut i32, b: &mut i32) {
//     let t = *a;
//     *a = *b;
//     *b = t;
// }
