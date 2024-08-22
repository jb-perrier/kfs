use core::arch::asm;

use super::gdt::GdtDescriptor;

#[inline]
pub unsafe fn out_u16(port: u16, val: u16) {
    asm!("out dx, al", in("dx") port, in("al") val as i8);
}

#[inline]
pub unsafe fn in_u8(port: u16) -> u8 {
    let return_val: u8;
    asm!("in al, dx", in("dx") port, out("al") return_val);
    return_val
}

#[inline]
pub unsafe fn nop(count: usize) {
    let mut i = 0;
    while i < count {
        asm!("nop");
        i += 1;
    }
}

extern "C" {
    pub fn shutdown();
    pub fn disable_interrupts();
    pub fn enable_interrupts();
    pub fn halt();
    pub fn load_gdt(gdt: *const GdtDescriptor);
    pub fn check_gdt() -> u32;
}
