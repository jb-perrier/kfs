pub mod asm;
pub mod bits;
pub mod boot;
pub mod cmos;
pub mod dump;
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
use dump::{dump, print_as_hex};

pub static mut KERN: Kernel = Kernel {
    time: Time::new(),
    vga: Vga::new(),
};

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
    time: Time,
    vga: Vga,
}

impl Kernel {
    pub unsafe fn start(&mut self, multiboot: &Multiboot, magic: u32) {
        // Disable interrupts until we setup the IDT
        disable_interrupts();
        self.vga.clear();

        if magic != 0x2BADB002 {
            self.vga.write_str_with_colors(
                "Unknown multiboot ! magic: ",
                &Colors::Red,
                &Colors::Black,
            );
            self.vga.write_usize(magic as usize);
            infinite_loop!();
        }

        // Initialize GDT
        if init_gdt() != 0 {
            self.vga
                .write_str_with_colors("Failed to load GDT !", &Colors::Red, &Colors::Black);
            infinite_loop!();
        }

        self.write_str("KERNEL_START: ");
        let addr = &asm::_KERNEL_START as *const _ as usize;
        print_as_hex(addr, 8);
        self.write_str("\n");
        // Check GDT
        // self.vga.write_str_with_colors(
        //     include_str!("./header_top"),
        //     &Colors::Green,
        //     &Colors::Black,
        // );
        // self.vga.write('\n');
        // self.vga.write_str(include_str!("./header_bottom"));
        // self.vga.write('\n');

        self.write_str("Dump stack:\n");
        let stack_ptr = asm::get_stack_ptr() as *const u8;
        let stack_top = asm::get_stack_top() as *const u8;
        dump(stack_ptr, stack_top);

        self.write_str("\n");
        const STR_BUFFER: &str = "Dump GDT: hello this is a very nice text indeed!\n";
        dump(STR_BUFFER.as_ptr(), STR_BUFFER.as_ptr().add(STR_BUFFER.len()));
        let index = self.vga.get_index();
        self.vga.set_cursor_pos(index);

        
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

    pub unsafe fn write_str(&mut self, s: &str) {
        self.vga.write_str(s);
    }

    pub unsafe fn write_usize(&mut self, n: usize) {
        self.vga.write_usize(n);
    }
}
