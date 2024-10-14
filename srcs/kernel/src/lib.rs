#![no_std]
#![no_main]
#![feature(strict_provenance)]
#![allow(unused)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::vec_init_then_push)]

extern crate alloc;

pub mod asm;
pub mod bits;
pub mod boot;
pub mod cmos;
pub mod dump;
pub mod error;
pub mod gdt;
pub mod idt;
pub mod kernel;
pub mod keyboard;
pub mod libc;
pub mod mem;
pub mod panic;
pub mod process;
pub mod shell;
pub mod text;
pub mod time;

mod kmain;

use self::{cmos::Cmos, time::Time};
use alloc::{format, vec::Vec};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use core::{
    alloc::Layout, ffi::c_void, mem::size_of, panic::{PanicInfo, PanicMessage}
};
use dump::{dump, print_as_hex};
use kernel::Kernel;
use mem::{heap::HEAP, paging::directory::PageDirectory};
use multiboot::information::MemoryType;
use process::Process;
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

    let Ok((mut frame_allocator, page_directory)) = mem::init(&boot_info) else {
        text::write_str_with_colors("Failed to init memory !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    };

    if let Err(_) = keyboard::init() {
        text::write_str_with_colors("Failed to init keyboard !", &Colors::Red, &Colors::Black);
        infinite_loop!();
    }

    unsafe {
        let number = 0_u64;
        let heap_alloc = HEAP.allocate(Layout::for_value(&number)).unwrap();
        text::write_str("Alloc addr: 0x");
        text::write_num_hex!(heap_alloc as usize);
        text::write_str(" size: ");
        text::write_num!(HEAP.get_size(heap_alloc).unwrap());
        text::write_str("\n");

        HEAP.deallocate(heap_alloc);
    }
    let mut v: Vec<isize> = Vec::with_capacity(5);
    v.push(56);
    text::write_str("Value in vec: ");
    text::write_num!(v[0]);
    text::write_str("\n");

    text::write_format!("My age is {}\n", 25);

    text::write_str_with_colors("Kernel initialized !\n", &Colors::Green, &Colors::Black);

    let user_page_directory = PageDirectory::new_from_frame_allocator(&mut frame_allocator, true).unwrap();
    let user_page_directory = unsafe { &mut *user_page_directory };
    let first_user_page = frame_allocator.allocate().unwrap();
    user_page_directory.add_frame_as_page(first_user_page, true);
    text::write_str_with_colors("User space initialized !\n", &Colors::Green, &Colors::Black);

    shell::print_shell();
    infinite_loop!();
}
