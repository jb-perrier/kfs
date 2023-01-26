// since there is no std library for ur kernel
#![no_std]
// we provide ur own !
#![no_main]
#![allow(unused)]

mod kernel;

#[no_mangle]
pub unsafe extern "C" fn kmain(multiboot: *const kernel::multiboot::Multiboot, magic: u32) -> ! {
    
    kernel::INSTANCE.start(multiboot, magic);
    loop {}
}