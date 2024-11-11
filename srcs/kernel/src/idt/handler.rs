use crate::{asm::{HandlerRegisters, InterruptRegisters}, idt::get_interrupt_name, infinite_loop, text};

use super::asm::GeneralRegisters;

pub enum ControlFlow {
    Continue,
    Halt,
}

pub type InterruptHandler = fn(HandlerRegisters); // return esp

static mut INTERRUPT_HANDLERS: [InterruptHandler; 256] = [unhandled_interrupt; 256];

pub fn fatal_handler(regs: HandlerRegisters) {
    text::write_str("Fatal interrupt: ");
    match get_interrupt_name(regs.interrupt.int_no, regs.interrupt.err) {
        Some(name) => text::write_str(name),
        None => text::write_num!(regs.interrupt.int_no),
    }
    text::write_str("\n");
    infinite_loop!();
}

pub fn unhandled_interrupt(regs: HandlerRegisters) {
    text::write_str("Unhandled interrupt: ");
    match get_interrupt_name(regs.interrupt.int_no, regs.interrupt.err) {
        Some(name) => text::write_str(name),
        None => text::write_num!(regs.interrupt.int_no),
    }
    text::write_str("\n");
    infinite_loop!();
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