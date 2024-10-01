#![no_std]
#![no_main]
#![allow(unused)]
#![allow(static_mut_refs)]
#![feature(strict_provenance)]

mod kernel;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kmain(magic: u32, multiboot: *const kernel::multiboot::Multiboot) -> ! {
    kernel::KERN.start(&*multiboot, magic);

    kernel::infinite_loop!();
}
