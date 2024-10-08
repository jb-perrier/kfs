use core::arch::asm;

const VGA_ADDR: *const u16 = 0xB8000 as *const u16;
const VGA_ROWS: u32 = 25;
const VGA_COLUMNS: u32 = 80;
const VGA_BUFFER_SIZE: u32 = VGA_ROWS * VGA_COLUMNS;

static mut INDEX: u32 = 0;

#[repr(u8)]
#[derive(Copy, Clone)]
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
    White = 15,
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
            *cha = build_char(' ', &Colors::White, &Colors::Black);
            i += 1;
        }
        INDEX = 0;
    }
}

fn build_char(c: char, fore_color: &Colors, back_color: &Colors) -> u16 {
    let attrib = (((*back_color as u8) << 4) | ((*fore_color as u8) & 0x0F)) as u16;
    c as u16 | (attrib << 8)
}

#[inline]
pub fn write(c: char) {
    write_with_colors(c, &Colors::White, &Colors::Black);
}

#[inline]
pub fn set_cursor_pos(pos: isize) {
    unsafe {
        let u16pos = pos as u16;
        asm::out_u16(0x3D4, 0x0E);
        asm::out_u16(0x3D5, u16pos >> 8);
        asm::out_u16(0x3D4, 0x0F);
        asm::out_u16(0x3D5, u16pos);
    }
}

#[inline]
pub fn write_with_colors(c: char, fore_color: &Colors, back_color: &Colors) {
    unsafe {
        if INDEX >= VGA_BUFFER_SIZE {
            clear();
        }
        match c {
            '\n' => {
                INDEX += VGA_COLUMNS - (INDEX % VGA_COLUMNS);
            }
            '\t' => {
                INDEX += 4;
            }
            '\r' => {
                INDEX -= INDEX % VGA_COLUMNS ;
            }
            c => {
                let cha = VGA_ADDR.offset(INDEX as isize).cast_mut();
                *cha = build_char(c, fore_color, back_color);
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

pub fn write_str_with_colors(str: &str, fore_color: &Colors, back_color: &Colors) {
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
