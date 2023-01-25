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

pub struct VGA {
    index: isize,
}

impl VGA {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub unsafe fn clear(&mut self) {
        let mut i: isize = 0;
        while i < VGA_BUFFER as isize {
            let cha = VGA_ADDR.offset(i).cast_mut();
            *cha = ' ' as u8;
            let col = VGA_ADDR.offset(i + 1).cast_mut();
            *col = (0 as u8) << 5 | 0 as u8;
            i += 2;
        }
    }

    #[inline]
    pub unsafe fn write(&mut self, c: char) {
        self.write_with_colors(c, &Colors::White, &Colors::Black);
    }

    #[inline]
    pub unsafe fn set_cursor_pos(&mut self, pos: u16) {
        asm::outb(0x3D4, 0x0E);
        asm::outb(0x3D5, pos >> 8);
        asm::outb(0x3D4, 0x0F);
        asm::outb(0x3D5, pos);
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
        self.set_cursor_pos(self.index as u16);
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
}
