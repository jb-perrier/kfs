// since there is no std library for ur kernel
#![no_std]
// we provide ur own !
#![no_main]

mod libc;
mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// we should not mangle the name so the multiboot can find it at link time
#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    //vga::clear();
    vga::write('H', 0);
    // KERNEL LOGIC
    loop {}
}