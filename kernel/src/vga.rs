const VGA_ADDR: *const u8 = 0xB8000 as *const u8;
const VGA_ROWS: u32 = 24;
const VGA_COLUMNS: u32 = 80;
const VGA_MAX: u32 = VGA_ROWS * VGA_COLUMNS;
const VGA_BUFFER: u32 = VGA_MAX * 2;

pub struct VGA {
    index: isize,
}

impl VGA {
    pub fn new() -> Self {
        Self {
            index: 0,
        }
    }

    pub unsafe fn clear(&mut self) {
        let mut i: isize = 0;
        while i < VGA_BUFFER as isize {
            let value = VGA_ADDR.offset(i).cast_mut();
            *value = 0x0F as u8;
            i += 1;
        }
    }
    
    #[inline]
    pub unsafe fn write(&mut self, c: char, color: u8) {
        let rindex = self.index * 2;
        let cha = VGA_ADDR.offset(rindex).cast_mut();
        *cha = c as u8;
        let col = VGA_ADDR.offset(rindex + 1).cast_mut();
        *col = 0x07;
        self.index += 1;
    }
    
    pub unsafe fn write_str(&mut self, str: &str, color: u8) {
        for c in str.chars() {
            self.write(c, color);
        }
    }
}
