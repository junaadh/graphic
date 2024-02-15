use std::fmt;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub const COLS: usize = 8 * 2;
pub const ROWS: usize = 6 * 2;

pub const CELL_WIDTH: usize = WIDTH / COLS;
pub const CELL_HEIGHT: usize = HEIGHT / ROWS;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Colors {
    Red = 0x000000ff,
    Green = 0x0000ff00,
    Blue = 0x00ff0000,
    Black = 0x00000000,
    White = 0xffffffff,
    Background = 0xff202020,
}

impl fmt::LowerHex for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08X}", *self as u32)
    }
}
