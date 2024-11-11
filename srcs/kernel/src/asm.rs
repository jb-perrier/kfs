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
pub fn nop(count: usize) {
    let mut i = 0;
    while i < count {
        unsafe { asm!("nop"); }
        i += 1;
    }
}

#[no_mangle] static mut PROC_ESP: u32 = 0;

pub fn set_proc_esp(esp: u32) {
    unsafe {
        PROC_ESP = esp;
    }
}

pub fn get_proc_esp() -> u32 {
    unsafe { PROC_ESP }
}

#[no_mangle] static mut PROC_PAGE_DIR: u32 = 0;

pub fn set_proc_page_dir(page_dir: u32) {
    unsafe {
        PROC_PAGE_DIR = page_dir;
    }
}

pub fn get_proc_page_dir() -> u32 {
    unsafe { PROC_PAGE_DIR }
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
    pub fn _get_stack_top() -> *const u32;
    pub fn _get_stack_bottom() -> *const u32;
    pub fn _get_stack_ptr() -> *const u32;
    pub fn _idt_flush(idt: usize);
    pub fn _divide_zero();
    pub fn _clean_registers();
    pub fn _switch_tcb(tcb: InterruptRegisters);
    pub fn _update_stack_pointers(esp: u32, ebp: u32);
}

pub fn switch_tcb(tcb: InterruptRegisters) {
    unsafe {
        _switch_tcb(tcb);
    }
}

pub fn update_stack_pointers(esp: u32, ebp: u32) {
    unsafe {
        _update_stack_pointers(esp, ebp);
    }
}
pub fn clean_registers() {
    unsafe {
        _clean_registers();
    }
}

pub fn get_stack_top() -> *const u8 {
    unsafe { _get_stack_top() as *const u8 }
}

pub fn get_stack_bottom() -> *const u8 {
    unsafe { _get_stack_bottom() as *const u8 }
}

pub fn get_stack_ptr() -> *const u8 {
    unsafe { _get_stack_ptr() as *const u8 }
}

pub fn divide_zero() {
    unsafe {
        _divide_zero();
    }
}

pub fn enable_paging() {
    unsafe {
        _enable_paging();
    }
}

pub fn set_page_directory(page_directory: *mut PageDirectory) {
    unsafe {
        _set_page_directory(page_directory);
    }
}

pub fn kernel_start() -> usize {
    unsafe { addr_of!(_KERNEL_START) as usize }
}

pub fn kernel_end() -> usize {
    unsafe { addr_of!(_KERNEL_END) as usize }
}

pub fn disable_interrupts() {
    unsafe {
        _disable_interrupts();
    }
}

pub fn enable_interrupts() {
    unsafe {
        _enable_interrupts();
    }
}

pub fn idt_flush(idt: *const IDTPointer) {
    unsafe {
        _idt_flush(idt as usize);
    }
}
pub unsafe fn load_gdt(gdt: *const GdtDescriptor) {
    _load_gdt(gdt);
}

pub fn check_gdt() -> u32 {
    unsafe { _check_gdt() }
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy, Debug)]
pub struct GeneralRegisters {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
}

impl GeneralRegisters {
    pub fn new() -> Self {
        Default::default()
    }
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy, Debug)]
pub struct InterruptRegisters {
    pub int_no: u32,
    pub err: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
}

impl InterruptRegisters {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy, Debug)]
pub struct HandlerRegisters {
    pub general: GeneralRegisters,
    pub interrupt: InterruptRegisters,
}

impl HandlerRegisters {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn jump_in_new_process(esp: u32, ebp: u32, eip: u32, eflags: u32, cr3: u32) {
    unsafe {
        asm!(
            "mov cr3, {4}",
            "mov esp, {0}",
            "mov ebp, {1}",
            "push {3}",
            "popfd",
            "push {2}", // push eip, since call can use the reg in which there is eip
            "call process_start",
            "sti",
            "pop eax",
            "call eax",
            "call process_end",
            in(reg) esp,
            in(reg) ebp,
            in(reg) eip,
            in(reg) eflags,
            in(reg) cr3,
            options(noreturn)
        );
    }
}
