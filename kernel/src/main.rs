// since there is no std library for ur kernel
#![no_std]
// we provide ur own !
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}

// we should not mangle the name so the multiboot can find it at link time
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // KERNEL LOGIC
    loop {}
}

