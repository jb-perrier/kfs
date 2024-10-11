use crate::{asm, error::KernelError, idt::{handler::set_interrupt_handler, registers::Registers}, text};
use layouts::{get_char, Key, QWERTY_MAP};

pub mod layouts;

// Const of the keyboard port
const KEYBOARD_PORT: u16 = 0x60;
const KEYBOARD_INTERRUPT: usize = 33;


pub fn init() -> Result<(), KernelError> {
    set_interrupt_handler(KEYBOARD_INTERRUPT, keyboard_handler);
    Ok(())
}

fn detect_layout() -> &'static [Key; 128] {
    // For simplicity, we assume QWERTY layout is default
    // You can implement a more sophisticated detection mechanism here
    &QWERTY_MAP
}

fn keyboard_handler(reg: Registers) {
    let scancode  = unsafe { asm::in_u8(KEYBOARD_PORT) };
    if scancode >= 128 {
        return; // TODO: handle released keys
    }
    let layout = detect_layout();
    if let Key::Char(c) = get_char(layout, scancode) {
        text::write(c);
    }
    unsafe {
        asm::out_u8(0x20, 0x20);
    }
}