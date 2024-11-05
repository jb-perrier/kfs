use core::arch::asm;

const VGA_ADDR: *const u16 = 0xB8000 as *const u16;
const VGA_ROWS: u32 = 25;
const VGA_COLUMNS: u32 = 80;
const VGA_BUFFER_SIZE: u32 = VGA_ROWS * VGA_COLUMNS;

static mut INDEX: u32 = 0;
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

    pub fn from_u32(u: u32) -> Colors {
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
            *cha = build_char(' ', Colors::White, Colors::Black);
            i += 1;
        }
        INDEX = 0;
        CURSOR_INDEX = 0;
    }
}

fn build_char(c: char, fore_color: Colors, back_color: Colors) -> u16 {
    let attrib = (((back_color as u8) << 4) | ((fore_color as u8) & 0x0F)) as u16;
    c as u16 | (attrib << 8)
}

pub fn write(c: char) {
    write_with_colors(c, Colors::White, Colors::Black);
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

pub fn get_index() -> u32 {
    unsafe {
        INDEX
    }
}

pub fn set_index(index: u32) {
    unsafe {
        INDEX = index;
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

pub fn erase() {
    unsafe {
        if CURSOR_INDEX != 0 {
            if CURSOR_INDEX != INDEX {
                for i in CURSOR_INDEX..INDEX {
                    let cha = VGA_ADDR.offset((i - 1) as isize).cast_mut();
                    *cha = *VGA_ADDR.offset(i as isize);
                }
            }

            let cha = VGA_ADDR.offset((INDEX - 1) as isize).cast_mut();
            *cha = build_char(' ', Colors::White, Colors::Black);
            CURSOR_INDEX -= 1;
            INDEX -= 1;
        }
    }
}

pub fn write_with_colors(c: char, fore_color: Colors, back_color: Colors) {
    unsafe {
        if INDEX >= VGA_BUFFER_SIZE {
            // Move all lines up
            for i in 0..VGA_BUFFER_SIZE - VGA_COLUMNS {
                let cha = VGA_ADDR.offset(i as isize).cast_mut();
                *cha = *VGA_ADDR.offset((i + VGA_COLUMNS) as isize);
            }
            // Clear last line
            for i in (VGA_BUFFER_SIZE - VGA_COLUMNS..VGA_BUFFER_SIZE).rev() {
                let cha = VGA_ADDR.offset(i as isize).cast_mut();
                *cha = build_char(' ', fore_color, back_color);
            }
            INDEX -= VGA_COLUMNS;
            CURSOR_INDEX -= VGA_COLUMNS;
        }
        match c {
            '\n' => {
                INDEX += VGA_COLUMNS - (INDEX % VGA_COLUMNS);
                CURSOR_INDEX = INDEX;
            }
            '\t' => {
                INDEX += 4;
                CURSOR_INDEX = INDEX;
            }
            '\r' => {
                INDEX -= INDEX % VGA_COLUMNS;
                CURSOR_INDEX = INDEX;
            }
            c => {
                if CURSOR_INDEX != INDEX {
                    for i in (CURSOR_INDEX..INDEX).rev() {
                        let cha = VGA_ADDR.offset((i + 1) as isize).cast_mut();
                        *cha = *VGA_ADDR.offset(i as isize);
                    }
                }
                let cha = VGA_ADDR.offset(CURSOR_INDEX as isize).cast_mut();
                *cha = build_char(c, fore_color, back_color);
                CURSOR_INDEX += 1;
                INDEX += 1;
            }
        }
    }
}

pub fn write_str(str: &str) {
    for c in str.chars() {
        write(c);
    }
}

#[macro_export]
macro_rules! write_format {
    ($($arg:tt)*) => {
        {
            let str = alloc::format!($($arg)*);
            $crate::text::write_str(str.as_str());
        }
    };
}
pub use write_format;

pub fn write_str_with_colors(str: &str, fore_color: Colors, back_color: Colors) {
    for c in str.chars() {
        write_with_colors(c, fore_color, back_color);
    }
}

#[macro_export]
macro_rules! write_num {
    ($value:expr) => {{
        if $value == 0 {
            $crate::text::write('0');
        } else {
            let mut str: [char; 20] = ['0'; 20];
            let mut size = 0;
            let mut num = $value;
            while num != 0 {
                let digit = num % 10;
                str[size] = (b'0' + digit as u8) as char;
                size += 1;
                num /= 10;
            }
            while size != 0 {
                $crate::text::write(str[size - 1]);
                size -= 1;
            }
        }
    }};
}
pub use write_num;

#[macro_export]
macro_rules! write_num_hex {
    ($value:expr) => {{
        if $value == 0 {
            $crate::text::write('0');
        } else {
            unsafe { $crate::dump::print_as_hex($value as usize, 8) };
        }
    }};
}
pub use write_num_hex;

use crate::asm;


