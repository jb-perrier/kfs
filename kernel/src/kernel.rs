// since there is no std library for ur kernel
#![no_std]
// we provide ur own !
#![no_main]

mod libc;
mod vga;

use vga::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// we should not mangle the name so the multiboot can find it at link time
#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    let mut vga = VGA::new();
    //vga::clear();
    vga.write_str("Hello default colors");
    vga.write_str_with_colors("That's cool with other colors !", &Colors::Green, &Colors::Black);
    // KERNEL LOGIC
    loop {}
}