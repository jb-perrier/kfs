pub mod asm;
pub mod bits;
pub mod boot;
pub mod cmos;
pub mod gdt;
pub mod libc;
pub mod multiboot;
pub mod paging;
pub mod shell;
pub mod time;
pub mod vga;

use crate::kernel::multiboot::MmapEntry;

use self::{cmos::Cmos, multiboot::Multiboot, time::Time};
use asm::{check_gdt, disable_interrupts, enable_interrupts, load_gdt};
use core::{mem::size_of, panic::PanicInfo};
use gdt::init_gdt;
use paging::init_paging;
use vga::*;

pub static mut INSTANCE: Kernel = Kernel { time: Time::new() };

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

pub struct Kernel {
    time: time::Time,
}

impl Kernel {
    pub unsafe fn start(&mut self, multiboot: &Multiboot, magic: u32) {
        // Disable interrupts until we setp the IDT
        disable_interrupts();
        let mut vga = Vga::new();
        vga.clear();

        if magic != 0x2BADB002 {
            vga.write_str_with_colors("Unknown multiboot ! magic: ", &Colors::Red, &Colors::Black);
            vga.write_usize(magic as usize);
            infinite_loop!();
        }

        // Initialize GDT
        if init_gdt() != 0 {
            vga.write_str_with_colors("Failed to load GDT !", &Colors::Red, &Colors::Black);
            infinite_loop!();
        }

        // Init Paging
        init_paging();

        // Initialize IDT
        //enable_interrupts();

        // Handle multiboot data
        // vga.write_str("Multiboot found ! magic: ");
        // vga.write_usize(magic as usize);
        // vga.write('\n');

        // vga.clear();
        // vga.set_index(0);

        // if !bits::get_bit_at(multiboot.flags, 6) {
        //     vga.write_str("Invalid memory map from multiboot info !\n");
        //     infinite_loop!();
        // }

        // vga.write_str("mmap_addr = ");
        // vga.write_usize(multiboot.mmap_addr as usize);
        // vga.write_str(" mmap_length = ");
        // vga.write_usize(multiboot.mmap_length as usize);
        // vga.write('\n');
        // let mut total_mem = 0 as usize;
        // let mut sector_count = 0;
        // let mmap_ptr = multiboot.mmap_addr as *const MmapEntry;
        // let mut offset = 0;
        // while offset < multiboot.mmap_length {
        //     let entry = &*mmap_ptr.byte_offset(offset as isize);
        //     offset += entry.size + 4; // 4 = sizeof(entry.size)
        //     if entry.ty == multiboot::MEMORY_AVAILABLE {
        //         total_mem += entry.len as usize;
        //         sector_count += 1;
        //         vga.write_str("sector size = ");
        //         vga.write_usize(entry.len as usize);
        //         vga.write_str(" index = ");
        //         vga.write_usize(sector_count as usize);
        //         vga.write('\n');
        //     }
        // }

        // vga.write_str("total ram = ");
        // vga.write_usize(total_mem);
        // vga.write_str(" sector count = ");
        // vga.write_usize(sector_count as usize);
        // vga.write('\n');

        // self.update_time();
        // let time = self.get_time();
        // vga.write_str("Time UTC = ");
        // vga.write_u8(time.hour);
        // vga.write_str(":");
        // vga.write_u8(time.minute);
        // vga.write_str(":");
        // vga.write_u8(time.second);
        // vga.write('\n');

        vga.write_str(include_str!("./header_42"));
        vga.write('\n');

        // vga.write_str_with_colors(include_str!("./header_top"), &Colors::Green, &Colors::Black);
        // vga.write('\n');
        // vga.write_str(include_str!("./header_bottom"));
        // vga.write('\n');
        // vga.write_str("\n\rkernel>");
        let index = vga.get_index();
        vga.set_cursor_pos(index);

        infinite_loop!();
    }

    pub unsafe fn shutdown(&self) {
        asm::shutdown();
    }

    pub fn get_time(&self) -> &time::Time {
        &self.time
    }

    unsafe fn update_time(&mut self) {
        let cmos = Cmos::read(true);
        self.time = Time::from_cmos(cmos)
    }
}
