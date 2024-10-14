use core::{arch::asm, ffi::c_void, ptr::addr_of};

use crate::{idt::IDTPointer, mem::paging::directory::PageDirectory};

use super::gdt::GdtDescriptor;

#[inline]
pub unsafe fn out_u16(port: u16, val: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") val);
}

#[inline]
pub unsafe fn out_u8(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val);
    
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
    pub static _KERNEL_START: *const usize;
    pub static _KERNEL_END: *const usize;
    pub static _KERNEL_STACK_TOP: *const c_void;
    pub static _KERNEL_STACK_BOTTOM: *const c_void;
    pub fn _disable_interrupts();
    pub fn _enable_interrupts();
    pub fn halt();
    pub fn _load_gdt(gdt: *const GdtDescriptor);
    pub fn _check_gdt() -> u32;
    pub fn _enable_paging();
    pub fn _set_page_directory(page_directory: *mut PageDirectory);
    pub fn get_stack_top() -> *const u32;
    pub fn get_stack_bottom() -> *const u32;
    pub fn get_stack_ptr() -> *const u32;
    pub fn _idt_flush(idt: usize);
    pub fn _divide_zero();
}

pub fn divide_zero() {
    unsafe { _divide_zero(); }
}

pub fn enable_paging() {
    unsafe { _enable_paging(); }
}

pub fn set_page_directory(page_directory: *mut PageDirectory) {
    unsafe { _set_page_directory(page_directory); }
}

pub fn kernel_start() -> usize {
    unsafe { addr_of!(_KERNEL_START) as usize }
}

pub fn kernel_end() -> usize {
    unsafe { addr_of!(_KERNEL_END) as usize }
}

pub fn disable_interrupts() {
    unsafe { _disable_interrupts(); }
}

pub fn enable_interrupts() {
    unsafe { _enable_interrupts(); }
}

pub fn idt_flush(idt: *const IDTPointer) {
    unsafe { _idt_flush(idt as usize); }
}
pub unsafe fn load_gdt(gdt: *const GdtDescriptor) {
    _load_gdt(gdt);
}

pub fn check_gdt() -> u32 {
    unsafe { _check_gdt() }
}