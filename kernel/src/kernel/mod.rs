pub mod libc;
pub mod vga;
pub mod asm;
pub mod shell;
pub mod multiboot;

use vga::*;
use core::panic::PanicInfo;
use self::multiboot::Multiboot;

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
    pub unsafe fn start(&mut self, multiboot: *const Multiboot, magic: u32) {
        let mut vga = VGA::new();
        vga.clear();

        if magic != 0x2BADB002 {
            vga.write_str_with_colors("Unknown multiboot ! magic: ", &Colors::Red, &Colors::Black);
            vga.write_usize(magic as usize);
            loop {}
        }

        // Handle multiboot data
        vga.write_str("Multiboot found ! magic: ");
        vga.write_usize((*multiboot).flags as usize);

        // GDT
        // IDT
        // Paging
        
    
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

    pub unsafe fn shutdown(&self) {
        asm::shutdown();
    }
}