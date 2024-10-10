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
pub mod text;
pub mod panic;
pub mod process;
pub mod kernel;
pub mod error;
pub mod idt;

mod kmain;

use self::{cmos::Cmos, time::Time};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use kernel::Kernel;
use process::Process;
use core::{ffi::c_void, mem::size_of, panic::PanicInfo, panic::PanicMessage};
use dump::{dump, print_as_hex};
use multiboot::information::MemoryType;
use text::*;

#[macro_export]
macro_rules! infinite_loop {
    () => {
        loop {}
    };
}

pub fn start(multiboot: usize, magic: usize) {
    disable_interrupts();
    text::clear();

    text::write_str("Kernel addr: 0x");
    text::write_num_hex!(asm::kernel_start());
    text::write_str(" - 0x");
    text::write_num_hex!(asm::kernel_end());
    text::write_str("\n");

    text::write_str("Kernel size: 0x");
    text::write_num_hex!(asm::kernel_end() - asm::kernel_start());
    text::write_str(" bytes\n");

    let Some(boot_info) = boot::init(magic, multiboot) else {
        text::write_str_with_colors("Failed to parse multiboot !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    };
    
    if gdt::init().is_err() {
        text::write_str_with_colors("Failed to init GDT !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    }
    
    if idt::init().is_err() {
        text::write_str_with_colors("Failed to init IDT !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    }
    asm::enable_interrupts();

    let Ok((frame_allocator, page_directory, mut heap)) = mem::init(&boot_info) else {
        text::write_str_with_colors("Failed to init memory !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    };

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
    
    let heap_alloc = heap.allocate(16).unwrap();
    text::write_str("Alloc addr: 0x");
    text::write_num_hex!(heap_alloc as usize);
    text::write_str("\n");

    heap.deallocate(heap_alloc, 16);

    text::write_str_with_colors("Kernel initialized !\n", &Colors::Green, &Colors::Black);
    infinite_loop!();
}
