use core::ffi::c_void;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kmain(magic: u32, multiboot: u32) -> ! {
    crate::start(multiboot, magic);

    #[allow(clippy::empty_loop)]
    loop {}
}
