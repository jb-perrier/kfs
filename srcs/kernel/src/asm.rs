use core::{arch::asm, ffi::c_void};

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
    pub static _KERNEL_START: *const c_void;
    pub static _KERNEL_END: *const c_void;
    pub static _KERNEL_STACK_TOP: *const c_void;
    pub static _KERNEL_STACK_BOTTOM: *const c_void;
    pub fn shutdown();
    pub fn _disable_interrupts();
    pub fn _enable_interrupts();
    pub fn halt();
    pub fn _load_gdt(gdt: *const GdtDescriptor);
    pub fn _check_gdt() -> u32;
    pub fn enable_paging();
    pub fn set_page_directory(page_directory: *const c_void);
    pub fn get_stack_top() -> *const u32;
    pub fn get_stack_bottom() -> *const u32;
    pub fn get_stack_ptr() -> *const u32;
}

pub fn disable_interrupts() {
    unsafe { _disable_interrupts(); }
}

pub fn enable_interrupts() {
    unsafe { _enable_interrupts(); }
}

pub unsafe fn load_gdt(gdt: *const GdtDescriptor) {
    _load_gdt(gdt);
}

pub fn check_gdt() -> u32 {
    unsafe { _check_gdt() }
}