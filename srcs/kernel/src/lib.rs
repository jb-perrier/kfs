#![no_std]
#![no_main]
#![allow(unused)]
#![feature(strict_provenance)]

pub mod asm;
pub mod bits;
pub mod boot;
pub mod cmos;
pub mod dump;
pub mod gdt;
pub mod libc;
pub mod mem;
pub mod time;
pub mod vga;

mod kmain;

use self::{cmos::Cmos, time::Time};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use core::{ffi::c_void, mem::size_of, panic::PanicInfo};
use dump::{dump, print_as_hex};
use multiboot::information::MemoryType;
use vga::*;

#[macro_export]
macro_rules! infinite_loop {
    () => {
        loop {}
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    infinite_loop!()
}

pub fn start(multiboot: u32, magic: u32) {
    disable_interrupts();
    vga::clear();

    let Some(boot_info) = boot::init(magic, multiboot) else {
        vga::write_str_with_colors("Failed to parse multiboot !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    };

    if gdt::init().is_err() {
        vga::write_str_with_colors("Failed to load GDT !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    }

    if mem::init(&boot_info).is_err() {
        vga::write_str_with_colors("Failed to init memory !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    }

        // Check GDT
    // vga::write_str_with_colors(
    //     include_str!("./header_top"),
    //     &Colors::Green,
    //     &Colors::Black,
    // );
    // vga::write('\n');
    // vga::write_str(include_str!("./header_bottom"));
    // vga::write('\n');

    /*self.write_str("Dump stack:\n");
    let stack_ptr = asm::get_stack_ptr() as *const u8;
    let stack_top = asm::get_stack_top() as *const u8;
    dump(stack_ptr, stack_top);

    self.write_str("\n");
    const STR_BUFFER: &str = "Dump GDT: hello this is a very nice text indeed!\n";
    dump(STR_BUFFER.as_ptr(), STR_BUFFER.as_ptr().add(STR_BUFFER.len()));
    let index = vga::get_index();
    vga::set_cursor_pos(index);*/

    vga::write_str("We are using pagination !!\n");
    let num = 55652 + 522;
    vga::write_num!(num);
    infinite_loop!();
}
