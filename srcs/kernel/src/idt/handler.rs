use super::registers::Registers;

pub type InterruptHandler = fn(Registers) -> ();

static mut INTERRUPT_HANDLERS: [Option<InterruptHandler>; 256] = [None; 256];

pub fn get_interrupt_handler(index: usize) -> Option<InterruptHandler> {
    unsafe {
        INTERRUPT_HANDLERS[index]
    }
}

pub fn set_interrupt_handler(index: usize, handler: InterruptHandler) {
    unsafe {
        INTERRUPT_HANDLERS[index] = Some(handler);
    }
}