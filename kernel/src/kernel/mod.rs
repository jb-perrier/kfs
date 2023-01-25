mod libc;
mod vga;
mod asm;
mod shell;

use vga::*;
use core::panic::PanicInfo;

pub static mut INSTANCE: Kernel = Kernel{};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// wait for allocator impl
pub trait Driver {
    fn new() -> Self;
    fn update();
    fn destroy();
}

pub struct Kernel {

}

impl Kernel {
    pub unsafe fn start(&mut self) {

        // GDT
        // IDT
        // Paging
        let mut vga = VGA::new();
        vga.clear();
    
        vga.write_str_with_colors("   ___             _        _      ___     ___  \n\r", &Colors::Green, &Colors::Black);
        vga.write_str_with_colors("  | _ \\   __ _    | |_     (_)    / _ \\   / __|  \n\r", &Colors::Green, &Colors::Black);
        vga.write_str_with_colors("  |   /  / _` |   |  _|    | |   | (_) |  \\__ \\  \n\r", &Colors::Green, &Colors::Black);
        vga.write_str_with_colors("  |_|_\\  \\__,_|   _\\__|   _|_|_   \\___/   |___/ \n\r", &Colors::Green, &Colors::Black);
        vga.write_str("_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"| \n\r");
        vga.write_str("\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-' \n\r");

        vga.write_str("\n\r\n\r>");
        vga.set_cursor_pos(8);
        loop {}
    }

    pub unsafe fn shutdown() {
        asm::shutdown();
    }
}