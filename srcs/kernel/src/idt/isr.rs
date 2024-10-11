use crate::{infinite_loop, text};

use super::{handler::get_interrupt_handler, registers::Registers};

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
pub extern "C" fn isr_handler(regs: Registers) {
    unsafe {
        if let Some(handler) = get_interrupt_handler(regs.int_no as usize) {
            handler(regs);
        }
        print_isr(regs);
    }
}

fn print_isr(regs: Registers) {
    text::write_str("Interrupt: ");
    match regs.int_no {
        0 => text::write_str("Division by zero"),
        1 => text::write_str("Debug"),
        2 => text::write_str("Non-maskable interrupt"),
        3 => text::write_str("Breakpoint"),
        4 => text::write_str("Overflow"),
        5 => text::write_str("Bound range exceeded"),
        6 => text::write_str("Invalid opcode"),
        7 => text::write_str("Device not available"),
        8 => text::write_str("Double fault"),
        9 => text::write_str("Coprocessor segment overrun"),
        10 => text::write_str("Invalid TSS"),
        11 => text::write_str("Segment not present"),
        12 => text::write_str("Stack-segment fault"),
        13 => text::write_str("General protection fault"),
        14 => text::write_str("Page fault"),
        16 => text::write_str("x87 FPU floating-point error"),
        17 => text::write_str("Alignment check"),
        18 => text::write_str("Machine check"),
        19 => text::write_str("SIMD floating-point exception"),
        20 => text::write_str("Virtualization exception"),
        21 => text::write_str("Control protection exception"),
        _ => text::write_str("Unknown interrupt"),
    }
    text::write_str(" received\n");
}