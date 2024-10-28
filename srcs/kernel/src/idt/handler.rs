use crate::{idt::get_interrupt_name, infinite_loop, text};

use super::registers::Registers;

pub enum ControlFlow {
    Continue,
    Halt,
}

pub type InterruptHandler = fn(Registers, u32, u32); 

static mut INTERRUPT_HANDLERS: [InterruptHandler; 256] = [unhandled_interrupt; 256];

pub fn fatal_handler(regs: Registers, int_no: u32, err_code: u32) {
    text::write_str("Fatal interrupt: ");
    match get_interrupt_name(int_no, err_code) {
        Some(name) => text::write_str(name),
        None => text::write_num!(int_no),
    }
    text::write_str("\n");
    infinite_loop!();
}

pub fn unhandled_interrupt(regs: Registers, int_no: u32, err_code: u32) {
    text::write_str("Unhandled interrupt: ");
    match get_interrupt_name(int_no, err_code) {
        Some(name) => text::write_str(name),
        None => text::write_num!(int_no),
    }
    text::write_str("\n");
    // infinite_loop!();
}

pub fn get_interrupt_handler(index: usize) -> Option<InterruptHandler> {
    unsafe {
        INTERRUPT_HANDLERS.get(index).copied()
    }
}

pub fn set_interrupt_handler(index: usize, handler: InterruptHandler) {
    unsafe {
        INTERRUPT_HANDLERS[index] = handler;
    }
}