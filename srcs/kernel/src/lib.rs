#![no_std]
#![no_main]
#![allow(unused)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::vec_init_then_push)]
#![allow(static_mut_refs)]

extern crate alloc;

pub mod asm;
pub mod bits;
pub mod boot;
pub mod cmos;
pub mod debug;
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
pub mod signal;
pub mod text;
pub mod time;
pub mod user_api;
pub mod socket;
// pub mod vga_wip;

mod kmain;

use self::{cmos::Cmos, time::Time};
use alloc::{boxed::Box, format, vec::Vec};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use user_api::fork;
use core::{
    alloc::Layout,
    ffi::c_void,
    mem::size_of,
    panic::{PanicInfo, PanicMessage},
};
use debug::{disable_debug, enable_debug, set_debug_log, set_tracing};
use dump::{dump, print_as_hex};
use idt::handler::set_interrupt_handler;
use kernel::{kernel, set_kernel, Kernel};
use mem::{
    frame::{self, FRAME_SIZE},
    heap::{self},
    paging::directory::PageDirectory,
};
use multiboot::information::MemoryType;
use process::{scheduler::Scheduler, Process};
use shell::style::{parse_style_in_str, style};
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

    let Some(boot_info) = boot::init(magic, multiboot) else {
        text::write_str_with_colors("Failed to parse multiboot !", Colors::Red, Colors::Black);
        infinite_loop!();
    };

    if gdt::init().is_err() {
        text::write_str_with_colors("Failed to init GDT !", Colors::Red, Colors::Black);
        infinite_loop!();
    }

    if idt::init().is_err() {
        text::write_str_with_colors("Failed to init IDT !", Colors::Red, Colors::Black);
        infinite_loop!();
    }

    let Ok((mut frame_allocator, page_directory, heap)) = mem::init(&boot_info) else {
        text::write_str_with_colors("Failed to init memory !", Colors::Red, Colors::Black);
        infinite_loop!();
    };

    set_kernel(Kernel {
        frame_allocator,
        scheduler: Scheduler::new(),
        processes: Vec::new(),
        heap,
        page_directory,
        sockets: Vec::new(),
    });

    kernel().scheduler.current = 0;

    text::write_str_with_colors(
        include_str!("./header_top"),
        Colors::LightPurple,
        Colors::Black,
    );
    text::write_str("\n");
    text::write_str_with_colors(
        include_str!("./header_bottom"),
        Colors::LightGreen,
        Colors::Black,
    );
    text::write_str("\n\n");

    text::write_str_with_colors("Initialized !\n", text::Colors::Green, text::Colors::Black);

    if let Err(_) = keyboard::init() {
        text::write_str_with_colors("Failed to init keyboard !", Colors::Red, Colors::Black);
        infinite_loop!();
    }

    const FRAME_COUNT: usize = 16;
    let Ok(block) = kernel().frame_allocator.allocate_many(FRAME_COUNT) else {
        text::write_str_with_colors("Failed to allocate blocks !", Colors::Red, Colors::Black);
        infinite_loop!();
    };

    let heap = &mut kernel().heap;
    heap.add_block(block.addr(), FRAME_COUNT * FRAME_SIZE);

    pub fn user_proc_main() {
        trace!();
        fork();
        let mut i = 0_u32;
        let pid = kernel().get_current_process().unwrap().pid().0;
        loop {
            asm::disable_interrupts();
            text::write_num!(pid);
            text::write_str(":");
            text::write_num!(i);
            text::write_str("\n");
            asm::enable_interrupts();
            i += 1;
            if i > 10 {
                // i = 0;
                return;
            }
            for _ in 0..3000000 {
                unsafe { core::arch::asm!("nop") }
            }
        }
    }

    let process1 = Process::new(0, user_proc_main);
    kernel().processes.push(process1);

    asm::enable_interrupts();
    kernel().scheduler.run();
    
    infinite_loop!();
}