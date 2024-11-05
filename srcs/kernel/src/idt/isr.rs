use crate::{asm::{HandlerRegisters, InterruptRegisters}, infinite_loop, text};

use super::{handler::{get_interrupt_handler, ControlFlow}, asm::GeneralRegisters,};

extern "C" {
    pub fn _isr0();
    pub fn _isr1();
    pub fn _isr2();
    pub fn _isr3();
    pub fn _isr4();
    pub fn _isr5();
    pub fn _isr6();
    pub fn _isr7();
    pub fn _isr8();
    pub fn _isr9();
    pub fn _isr10();
    pub fn _isr11();
    pub fn _isr12();
    pub fn _isr13();
    pub fn _isr14();
    pub fn _isr15();
    pub fn _isr16();
    pub fn _isr17();
    pub fn _isr18();
    pub fn _isr19();
    pub fn _isr20();
    pub fn _isr21();
    pub fn _isr22();
    pub fn _isr23();
    pub fn _isr24();
    pub fn _isr25();
    pub fn _isr26();
    pub fn _isr27();
    pub fn _isr28();
    pub fn _isr29();
    pub fn _isr30();
    pub fn _isr31();
}

#[no_mangle]
pub extern "C" fn isr_handler(regs: HandlerRegisters) -> u32 {
    if let Some(handler) = get_interrupt_handler(regs.interrupt.int_no as usize) {
        return handler(regs);
    };
    0
}