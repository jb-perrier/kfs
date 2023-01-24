// since there is no std library for ur kernel
#![no_std]
// we provide ur own !
#![no_main]
#![allow(unused)]

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
        // vga.write_str("Hello default colors");
    // vga.write_str_with_colors("That's cool with other colors !", &Colors::Green, &Colors::Black);
    vga.write_str_with_colors("   ___             _        _      ___     ___  \n\r", &Colors::Green, &Colors::Black);
    vga.write_str_with_colors("  | _ \\   __ _    | |_     (_)    / _ \\   / __|  \n\r", &Colors::Green, &Colors::Black);
    vga.write_str_with_colors("  |   /  / _` |   |  _|    | |   | (_) |  \\__ \\  \n\r", &Colors::Green, &Colors::Black);
    vga.write_str_with_colors("  |_|_\\  \\__,_|   _\\__|   _|_|_   \\___/   |___/ \n\r", &Colors::Green, &Colors::Black);
    vga.write_str("_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"| \n\r");
    vga.write_str("\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-' \n\r");


    vga.write_str("\n\r\n\r>");
    // KERNEL LOGIC
    loop {}
}