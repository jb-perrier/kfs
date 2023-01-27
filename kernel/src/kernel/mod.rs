pub mod asm;
pub mod libc;
pub mod multiboot;
pub mod shell;
pub mod vga;

use self::multiboot::Multiboot;
use core::panic::PanicInfo;
use vga::*;

pub static mut INSTANCE: Kernel = Kernel {};

macro_rules! infinite_loop {
    () => {
        loop {}
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    infinite_loop!()
}

// wait for allocator impl
pub trait Driver {
    fn new() -> Self;
    fn update();
    fn destroy();
}

pub struct Kernel {}

impl Kernel {
    pub unsafe fn start(&mut self, multiboot: *const Multiboot, magic: u32) {
        let mut vga = VGA::new();
        vga.clear();

        if magic != 0x2BADB002 {
            vga.write_str_with_colors("Unknown multiboot ! magic: ", &Colors::Red, &Colors::Black);
            vga.write_usize(magic as usize);
            infinite_loop!();
        }

        // Handle multiboot data
        vga.write_str("Multiboot found ! magic: ");
        vga.write_usize(magic as usize);
        vga.write('\n');

        vga.clear();
        vga.set_index(0);

        /*if ((*multiboot).flags >> 6 & 0x1) == 0 {
            vga.write_str("Invalid memory map from multiboot info !\n");
            infinite_loop!();
        }*/

        // GDT
        // IDT
        // Paging

        vga.write_str_with_colors(include_str!("./header_top"), &Colors::Green, &Colors::Black);
        vga.write('\n');
        vga.write_str(include_str!("./header_bottom"));

        vga.write_str("\n\r\n\rkernel >");
        let index = vga.get_index();
        vga.set_cursor_pos(index);

        infinite_loop!();
    }

    pub unsafe fn shutdown(&self) {
        asm::shutdown();
    }
}
