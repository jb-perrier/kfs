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
pub mod debug;
pub mod user_api;

mod kmain;

use self::{cmos::Cmos, time::Time};
use alloc::{boxed::Box, format, vec::Vec};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use debug::{disable_debug, enable_debug, set_debug_log, set_tracing};
use idt::handler::set_interrupt_handler;
use shell::style::{parse_style_in_str, style};
use core::{
    alloc::Layout,
    ffi::c_void,
    mem::size_of,
    panic::{PanicInfo, PanicMessage},
};
use dump::{dump, print_as_hex};
use kernel::{kernel, set_kernel, Kernel};
use mem::{
    frame::{self, FRAME_SIZE},
    heap::{self},
    paging::directory::PageDirectory,
};
use multiboot::information::MemoryType;
use process::{scheduler::Scheduler, Process};
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
    asm::enable_interrupts();

    let Ok((mut frame_allocator, page_directory, heap)) = mem::init(&boot_info) else {
        text::write_str_with_colors("Failed to init memory !", Colors::Red, Colors::Black);
        infinite_loop!();
    };

    set_kernel(Kernel {
        frame_allocator,
        process: Process::new(page_directory, heap, 0),
        scheduler: Scheduler::new(),
        processes: Vec::new(),
        debug_log: true,
    });


    text::write_str_with_colors(include_str!("./header_top"), Colors::LightPurple, Colors::Black);
    text::write_str("\n");
    text::write_str_with_colors(include_str!("./header_bottom"), Colors::LightGreen, Colors::Black);
    text::write_str("\n\n");

    text::write_str_with_colors(
        "Initialized !\n",
        text::Colors::Green,
        text::Colors::Black,
    );

    if let Err(_) = keyboard::init() {
        text::write_str_with_colors("Failed to init keyboard !", Colors::Red, Colors::Black);
        infinite_loop!();
    }

    const FRAME_COUNT: usize = 16;
    let Ok(block) = kernel().frame_allocator.allocate_many(FRAME_COUNT) else {
        text::write_str_with_colors("Failed to allocate blocks !", Colors::Red, Colors::Black);
        infinite_loop!();
    };

    let heap = &mut kernel().process.heap;
    heap.add_block(block.addr(), FRAME_COUNT * FRAME_SIZE);

    let styled_string = format!("{}Hello, world! {}This is a test\n", style(Colors::Green, Colors::Black), style(Colors::Purple, Colors::Black));
    let strs = parse_style_in_str(styled_string.as_str()).unwrap();
    for s in strs {
        text::write_str_with_colors(&s.string, s.fore_color, s.back_color);
    }
    
    kernel().process.signal_callback = Some(Box::new(|sig| {
        text::write_str("Debug4\n");
        enable_debug();
        match sig {
            signal::Signal::Echo(msg) => {
                text::write_format!("Echo received: {}\n", msg);
            }
            sig => {
                text::write_format!("Signal received: {:?}\n", sig);
            }
        }
        disable_debug();
    }));
    shell::print_shell();
    infinite_loop!();
}




    // let number = 0_u64;
    // let heap_alloc = heap.allocate(Layout::for_value(&number)).unwrap();
    // text::write_str("Alloc addr: 0x");
    // text::write_num_hex!(heap_alloc as usize);
    // text::write_str(" size: ");
    // text::write_num!(heap.get_size_from_ptr(heap_alloc).unwrap());
    // text::write_str("\n");

    // heap.deallocate(heap_alloc);

    // let mut v: Vec<isize> = Vec::with_capacity(5);
    // v.push(56);
    // text::write_str("Value in vec: ");
    // text::write_num!(v[0]);
    // text::write_str("\n");

    // let kernel_start = asm::kernel_start();
    // let kernel_end = asm::kernel_end();
    // text::write_format!("Kernel end {kernel_start:#X}\n");
    // text::write_format!("Kernel end {kernel_end:#X}\n");

    // text::write_str_with_colors("Kernel initialized !\n", &Colors::Green, &Colors::Black);

    // let frame_allocator = &mut kernel().frame_allocator;
    // let user_page_directory =
    //     PageDirectory::new_from_frame_allocator(frame_allocator, true).unwrap();
    // let user_page_directory = unsafe { &mut *user_page_directory };
    // let first_user_page = frame_allocator.allocate().unwrap();
    // user_page_directory.add_frame_as_page(first_user_page, true);
    // text::write_str_with_colors("User space initialized !\n", &Colors::Green, &Colors::Black);

    // const STR_BUFFER: &str = "Dump GDT: hello this is a very nice text indeed!\n";
    // unsafe {
    //     dump(
    //         STR_BUFFER.as_ptr(),
    //         STR_BUFFER.as_ptr().add(STR_BUFFER.len()),
    //     )
    // };