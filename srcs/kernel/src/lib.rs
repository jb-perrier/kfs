#![no_std]
#![no_main]
#![allow(unused)]
#![feature(strict_provenance)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]

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
pub mod panic;
pub mod process;
pub mod kernel;
pub mod error;

mod kmain;

use self::{cmos::Cmos, time::Time};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use kernel::Kernel;
use process::Process;
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
    vga::clear();
    vga::write_str_with_colors("Kernel panic !\n", &Colors::Red, &Colors::Black);
    if let Some(location) = _info.location() {
        vga::write_str("  File: ");
        vga::write_str(location.file());
        vga::write_str(":");
        vga::write_num!(location.line());
        vga::write_str(":");
        vga::write_num!(location.column());
        vga::write_str("\n");
    }
    infinite_loop!()
}

pub fn start(multiboot: usize, magic: usize) {
    disable_interrupts();
    vga::clear();

    let mut kernel = Kernel::new();

    vga::write_str("Kernel addr: 0x");
    vga::write_num_hex!(asm::kernel_start());
    vga::write_str(" - 0x");
    vga::write_num_hex!(asm::kernel_end());
    vga::write_str("\n");

    vga::write_str("Kernel size: 0x");
    vga::write_num_hex!(asm::kernel_end() - asm::kernel_start());
    vga::write_str(" bytes\n");

    let Some(boot_info) = boot::init(magic, multiboot) else {
        vga::write_str_with_colors("Failed to parse multiboot !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    };
    kernel.multiboot.insert(boot_info);
    
    if gdt::init().is_err() {
        vga::write_str_with_colors("Failed to load GDT !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    }
    
    if mem::init(&mut kernel).is_err() {
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

    // let heap_alloc = kernel.process.heap_mut().allocate(16).unwrap();
    // vga::write_str("Alloc addr: 0x");
    // vga::write_num_hex!(heap_alloc as usize);
    // vga::write_str("\n");

    vga::write_str_with_colors("Kernel initialized !", &Colors::Green, &Colors::Black);
    infinite_loop!();
}
