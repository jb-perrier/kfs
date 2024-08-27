// since there is no std library for our kernel
#![no_std]
// we provide ur own !
#![no_main]
#![allow(unused)]

mod kernel;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kmain(magic: u32, multiboot: *const kernel::multiboot::Multiboot) {
    kernel::INSTANCE.start(&*multiboot, magic);
}
