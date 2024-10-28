use core::ptr::addr_of_mut;

use multiboot::information::Multiboot;

use crate::{infinite_loop, mem::{frame::FrameAllocator, heap::Heap}, process::Process, text};

pub struct Kernel {
    pub frame_allocator: FrameAllocator,
    pub process: Process,
}

impl Kernel {

}

static mut KERNEL: Option<Kernel> = None;

pub fn kernel() -> &'static mut Kernel {
    unsafe { KERNEL.as_mut().unwrap() }
}

pub fn set_kernel(kernel: Kernel) {
    unsafe { KERNEL = Some(kernel) }
}