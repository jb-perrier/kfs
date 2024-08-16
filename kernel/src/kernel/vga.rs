use core::arch::asm;

use crate::kernel::asm;

const VGA_ADDR: *const u8 = 0xB8000 as *const u8;
const VGA_ROWS: u32 = 24;
const VGA_COLUMNS: u32 = 80;
const VGA_MAX: u32 = VGA_ROWS * VGA_COLUMNS;
const VGA_BUFFER: u32 = VGA_MAX * 2;

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

pub struct Vga {
    index: isize,
}

impl Vga {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub unsafe fn clear(&mut self) {
        let mut i: isize = 0;
        while i < VGA_BUFFER as isize {
            let cha = VGA_ADDR.offset(i).cast_mut();
            *cha = b' ';
            let col = VGA_ADDR.offset(i + 1).cast_mut();
            *col = Colors::White as u8;
            i += 2;
        }
    }

    pub fn set_index(&mut self, index: isize) {
        self.index = index;
    }

    pub fn get_index(&self) -> isize {
        self.index
    }

    #[inline]
    pub unsafe fn write(&mut self, c: char) {
        self.write_with_colors(c, &Colors::White, &Colors::Black);
    }

    #[inline]
    pub unsafe fn set_cursor_pos(&mut self, pos: isize) {
        let u16pos = pos as u16;
        asm::out_u16(0x3D4, 0x0E);
        asm::out_u16(0x3D5, u16pos >> 8);
        asm::out_u16(0x3D4, 0x0F);
        asm::out_u16(0x3D5, u16pos);
    }

    #[inline]
    pub unsafe fn write_with_colors(&mut self, c: char, fore_color: &Colors, back_color: &Colors) {
        match c {
            '\n' => {
                self.index += VGA_COLUMNS as isize - (self.index % VGA_COLUMNS as isize);
            }
            '\t' => {
                self.index += 4;
            }
            '\r' => {
                self.index -= self.index % VGA_COLUMNS as isize;
            }
            c => {
                let rindex = self.index * 2;
                let cha = VGA_ADDR.offset(rindex).cast_mut();
                *cha = c as u8;
                let col = VGA_ADDR.offset(rindex + 1).cast_mut();
                *col = (*back_color as u8) << 5 | *fore_color as u8;
                self.index += 1;
            }
        }
    }

    pub unsafe fn write_str(&mut self, str: &str) {
        for c in str.chars() {
            self.write(c);
        }
    }

    pub unsafe fn write_str_with_colors(
        &mut self,
        str: &str,
        fore_color: &Colors,
        back_color: &Colors,
    ) {
        for c in str.chars() {
            self.write_with_colors(c, fore_color, back_color);
        }
    }

    #[inline]
    pub unsafe fn write_u8(&mut self, value: u8) {
        self.write_usize(value as usize);
    }

    pub unsafe fn write_usize(&mut self, value: usize) {
        if value == 0 {
            self.write('0');
            return;
        }
        let mut str: [char; 20] = ['0'; 20];
        let mut size = 0;
        let mut num = value;
        while num != 0 {
            let digit = num % 10;
            str[size] = (b'0' + digit as u8) as char;
            size += 1;
            num /= 10;
        }
        while size != 0 {
            self.write(str[size - 1]);
            size -= 1;
        }
    }
}
