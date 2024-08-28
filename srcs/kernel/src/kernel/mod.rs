pub mod asm;
pub mod libc;
pub mod multiboot;
pub mod vga;

use self::multiboot::Multiboot;
use asm::disable_interrupts;
use core::{mem::size_of, panic::PanicInfo};
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

pub struct Kernel {}

impl Kernel {
    pub unsafe fn start(&mut self, multiboot: &Multiboot, magic: u32) {
        disable_interrupts();

        let mut vga = Vga::new();
        vga.clear();

        if magic != 0x2BADB002 {
            vga.write_str_with_colors("Unknown multiboot ! magic: ", &Colors::Red, &Colors::Black);
            vga.write_usize(magic as usize);
            infinite_loop!();
        }

        vga.write_str(include_str!("./header_42"));
        vga.write('\n');

        let index = vga.get_index();
        vga.set_cursor_pos(index);

        infinite_loop!();
    }
}
