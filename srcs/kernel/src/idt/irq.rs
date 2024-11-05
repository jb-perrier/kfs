use crate::{asm::{out_u16, out_u8, HandlerRegisters, InterruptRegisters}, text};

use super::{handler::get_interrupt_handler, asm::GeneralRegisters};

const IRQ0: u8 = 32;
const IRQ1: u8 = 33;
const IRQ2: u8 = 34;
const IRQ3: u8 = 35;
const IRQ4: u8 = 36;
const IRQ5: u8 = 37;
const IRQ6: u8 = 38;
const IRQ7: u8 = 39;
const IRQ8: u8 = 40;
const IRQ9: u8 = 41;
const IRQ10: u8 = 42;
const IRQ11: u8 = 43;
const IRQ12: u8 = 44;
const IRQ13: u8 = 45;
const IRQ14: u8 = 46;
const IRQ15: u8 = 47;

extern "C" {
    pub fn _irq0();
    pub fn _irq1();
    pub fn _irq2();
    pub fn _irq3();
    pub fn _irq4();
    pub fn _irq5();
    pub fn _irq6();
    pub fn _irq7();
    pub fn _irq8();
    pub fn _irq9();
    pub fn _irq10();
    pub fn _irq11();
    pub fn _irq12();
    pub fn _irq13();
    pub fn _irq14();
    pub fn _irq15();
}

pub fn remap_irq_table() {
    unsafe {
        out_u8(0x20, 0x11);
        out_u8(0xA0, 0x11);
        out_u8(0x21, 0x20);
        out_u8(0xA1, 0x28);
        out_u8(0x21, 0x04);
        out_u8(0xA1, 0x02);
        out_u8(0x21, 0x01);
        out_u8(0xA1, 0x01);
        out_u8(0x21, 0x0);
        out_u8(0xA1, 0x0);
    }
}

#[no_mangle]
pub extern "C" fn irq_handler(regs: HandlerRegisters) -> u32 {
    unsafe {
        let int_no = regs.interrupt.int_no as usize;
        if int_no >= 40 {
            out_u8(0xA0, 0x20);
        }
        out_u8(0x20, 0x20);

        if let Some(handler) = get_interrupt_handler(int_no) {
            return handler(regs);
        }
        0
    }
}
