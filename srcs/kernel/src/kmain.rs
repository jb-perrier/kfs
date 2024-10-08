use core::ffi::c_void;

use crate::kernel::Kernel;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kmain(magic: usize, multiboot: usize) -> ! {
    crate::start(multiboot, magic);

    #[allow(clippy::empty_loop)]
    loop {}
}
