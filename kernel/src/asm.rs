use core::arch::asm;

#[inline]
pub unsafe fn outb(port: u16, val: u16) {
    asm!("out dx, al", in("dx") port as u16, in("al") val as i8);
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let return_val: u8;
    asm!("in al, dx", in("dx") port as u16, out("al") return_val);
    return return_val;
}

extern "C" {
    pub fn shutdown();
    pub fn disable_interrupts();
    pub fn enable_interrupts();
    pub fn halt();
    pub fn nop();
}
