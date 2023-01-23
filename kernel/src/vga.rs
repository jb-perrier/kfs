const VGA_ADDR: *const u8 = 0xB8000 as *const u8;
const VGA_ROWS: u32 = 24;
const VGA_COLUMNS: u32 = 80;
const VGA_MAX: u32 = VGA_ROWS * VGA_COLUMNS;
const VGA_BUFFER: u32 = VGA_MAX * 2;

pub struct VGA {

}

pub unsafe fn clear() {
    let mut i: isize = 0;
    while i < VGA_BUFFER as isize {
        let value = VGA_ADDR.offset(i).cast_mut();
        *value = 0x0F as u8;
        i += 1;
    }
}

#[inline]
pub unsafe fn write(c: char, color: u8) {
    let cha = VGA_ADDR.offset(0).cast_mut();
    *cha = c as u8;
    let col = VGA_ADDR.offset(1).cast_mut();
    *col = 0x07;
}