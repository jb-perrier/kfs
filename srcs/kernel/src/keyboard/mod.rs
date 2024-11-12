use crate::{
    asm::{self, GeneralRegisters, HandlerRegisters, InterruptRegisters},
    error::KernelError,
    idt::handler::set_interrupt_handler,
    set_index, shell, text,
};
use layouts::{get_char, Key, AZERTY_MAP, AZERTY_MAP_MAJ, QWERTY_MAP, QWERTY_MAP_MAJ};

pub mod layouts;

// Const of the keyboard port
const KEYBOARD_PORT: u16 = 0x60;
const KEYBOARD_INTERRUPT: usize = 33;

static mut SHIFT_PRESSED: bool = false;
static mut CTRL_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;

static mut LAYOUT: (&[Key; 128], &[Key; 128]) = (&AZERTY_MAP, &AZERTY_MAP_MAJ);
// static mut LAYOUT: (&[Key; 128], &[Key; 128]) = (&QWERTY_MAP, &QWERTY_MAP_MAJ);

pub fn init() -> Result<(), KernelError> {
    // set_interrupt_handler(KEYBOARD_INTERRUPT, keyboard_handler);
    Ok(())
}

pub fn set_azerty() {
    unsafe {
        LAYOUT = (&AZERTY_MAP, &AZERTY_MAP_MAJ);
    }
}

pub fn set_qwerty() {
    unsafe {
        LAYOUT = (&QWERTY_MAP, &QWERTY_MAP_MAJ);
    }
}

fn detect_layout() -> &'static [Key; 128] {
    unsafe {
        if SHIFT_PRESSED || CAPS_LOCK {
            LAYOUT.1
        } else {
            LAYOUT.0
        }
    }
}

fn keyboard_handler(regs: HandlerRegisters) {
    let scancode = unsafe { asm::in_u8(KEYBOARD_PORT) };
    let layout = detect_layout();
    if scancode >= 128 {
        let key = get_char(layout, scancode - 128);
        match key {
            Key::Shift => unsafe {
                SHIFT_PRESSED = false;
            },
            Key::Ctrl => unsafe {
                CTRL_PRESSED = false;
            },
            _ => {}
        }
    } else {
        let key = get_char(layout, scancode);
        match key {
            Key::Char(c) => {
                shell::add_char(c);
            }
            Key::Backspace => {
                shell::remove_char();
            }
            Key::LeftArrow => {
                shell::move_left();
            }
            Key::RightArrow => {
                shell::move_right();
            }
            Key::UpArrow => {
                //TODO: cmd historic
            }
            Key::DownArrow => {
                //TODO: cmd historic
            }
            Key::Shift => unsafe {
                SHIFT_PRESSED = true;
            },
            Key::Ctrl => unsafe {
                CTRL_PRESSED = true;
            },
            Key::CapsLock => unsafe {
                CAPS_LOCK = !CAPS_LOCK;
            },
            Key::Enter => {
                shell::execute();
            }
            _ => {}
        }
    }
    unsafe {
        asm::out_u8(0x20, 0x20);
    }
}
