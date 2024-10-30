use core::ptr::addr_of_mut;

use alloc::vec::Vec;
use multiboot::information::Multiboot;

use crate::{infinite_loop, mem::{frame::FrameAllocator, heap::Heap}, process::{scheduler::Scheduler, Process}, text};

/**
 * All these fields CANNOT use the heap in order to initialize themselves because HeapAllocator use kernel()
 */
pub struct Kernel {
    pub frame_allocator: FrameAllocator,
    pub process: Process,
    pub processes:  Vec<Process>,
    pub scheduler: Scheduler,
    // pub shell: Shell
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