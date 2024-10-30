use core::arch::asm;

use crate::asm;

const VGA_ADDR: *const u16 = 0xB8000 as *const u16;
pub const VGA_ROWS: usize = 25;
pub const VGA_COLUMNS: usize = 80;
pub const VGA_BUFFER_SIZE: usize = VGA_ROWS * VGA_COLUMNS;

static mut CURSOR_POS: u32 = 0;
static mut CURSOR_INDEX: u32 = 0;

#[repr(u8)]
#[derive(Default, Copy, Clone)]
pub enum Colors {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Purple = 5,
    Brown = 6,
    Gray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightPurple = 13,
    Yellow = 14,
    #[default]White = 15,
}

impl Colors {
    pub fn from_str(s: &str) -> Colors {
        match s {
            "Black" => Colors::Black,
            "Blue" => Colors::Blue,
            "Green" => Colors::Green,
            "Cyan" => Colors::Cyan,
            "Red" => Colors::Red,
            "Purple" => Colors::Purple,
            "Brown" => Colors::Brown,
            "Gray" => Colors::Gray,
            "DarkGray" => Colors::DarkGray,
            "LightBlue" => Colors::LightBlue,
            "LightGreen" => Colors::LightGreen,
            "LightCyan" => Colors::LightCyan,
            "LightRed" => Colors::LightRed,
            "LightPurple" => Colors::LightPurple,
            "Yellow" => Colors::Yellow,
            "White" => Colors::White,
            _ => Colors::White,
        }
    }

    pub fn from_u8(u: u8) -> Self {
        Colors::from_u32(u as u32)
    }

    pub fn from_u32(u: u32) -> Self {
        match u {
            0 => Colors::Black,
            1 => Colors::Blue,
            2 => Colors::Green,
            3 => Colors::Cyan,
            4 => Colors::Red,
            5 => Colors::Purple,
            6 => Colors::Brown,
            7 => Colors::Gray,
            8 => Colors::DarkGray,
            9 => Colors::LightBlue,
            10 => Colors::LightGreen,
            11 => Colors::LightCyan,
            12 => Colors::LightRed,
            13 => Colors::LightPurple,
            14 => Colors::Yellow,
            15 => Colors::White,
            _ => Colors::White,
        }
    }
}

pub fn init() {
    unsafe {
        clear();
    }
}

pub fn clear() {
    unsafe {
        let mut i = 0;
        while i < VGA_BUFFER_SIZE  {
            let cha = VGA_ADDR.offset(i as isize).cast_mut();
            *cha = encode(' ', Colors::White, Colors::Black);
            i += 1;
        }
        CURSOR_INDEX = 0;
    }
}

pub fn encode(c: char, fore_color: Colors, back_color: Colors) -> u16 {
    let attrib = (((back_color as u8) << 4) | ((fore_color as u8) & 0x0F)) as u16;
    c as u16 | (attrib << 8)
}

pub fn decode(c: u16) -> (char, Colors, Colors) {
    let c = c as u8;
    let attrib = (c >> 8) as u8;
    (c as char, Colors::from_u8(attrib & 0x0F), Colors::from_u8((attrib >> 4) & 0x0F))
}

pub fn empty_char() -> u16 {
    encode(' ', Colors::White, Colors::Black)
}

pub fn pos_to_index(line: u8, col: u8) -> usize {
    let line = line as usize;
    let col = col as usize;
    line * VGA_COLUMNS + col
}
pub fn write(c: char, line: u8, col: u8) {
    unsafe {
        let index = pos_to_index(line, col);
        let cha = VGA_ADDR.offset(index as isize).cast_mut();
        *cha = encode(c, Colors::White, Colors::Black);
    }
}

pub fn read(line: u8, col: u8) -> u16 {
    unsafe {
        let index = pos_to_index(line, col);
        let cha = VGA_ADDR.offset(index as isize).cast();
        *cha
    }
}

pub fn set_cursor_pos(pos: u32) {
    unsafe {
        let u16pos = pos as u16;
        asm::out_u16(0x3D4, 0x0E);
        asm::out_u16(0x3D5, u16pos >> 8);
        asm::out_u16(0x3D4, 0x0F);
        asm::out_u16(0x3D5, u16pos);
    }
}

pub fn get_cursor_index() -> u32 {
    unsafe {
        CURSOR_INDEX
    }
}

pub fn set_cursor_index(index: u32) {
    unsafe {
        CURSOR_INDEX = index;
    }
}
