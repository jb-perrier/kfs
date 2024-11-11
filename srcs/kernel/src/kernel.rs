use core::ptr::addr_of_mut;

use alloc::vec::Vec;
use multiboot::information::Multiboot;

use crate::{
    infinite_loop,
    mem::{frame::FrameAllocator, heap::Heap, paging::directory::PageDirectory},
    process::{scheduler::Scheduler, Process, ProcessId},
    socket::Socket,
    text,
};

/**
 * All these fields CANNOT use the heap in order to initialize themselves because HeapAllocator use kernel()
 */
pub struct Kernel {
    pub frame_allocator: FrameAllocator,
    pub processes: Vec<Process>,
    pub scheduler: Scheduler,

    pub heap: Heap,
    pub page_directory: *mut PageDirectory,

    pub sockets: Vec<Socket>,
}

impl Kernel {
    pub fn get_process(&self, pid: ProcessId) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid() == pid)
    }

    pub fn get_process_mut(&mut self, pid: ProcessId) -> Option<&mut Process> {
        self.processes
            .iter_mut()
            .find(|p: &&mut Process| p.pid() == pid)
    }

    pub fn get_current_process(&mut self) -> Option<&mut Process> {
        self.processes.get_mut(self.scheduler.current)
    }

    pub fn get_current_process_index(&self) -> usize {
        self.scheduler.current
    }
}

static mut KERNEL: Option<Kernel> = None;

pub fn kernel_option() -> Option<&'static mut Kernel> {
    unsafe { KERNEL.as_mut() }
}

pub fn kernel() -> &'static mut Kernel {
    unsafe { KERNEL.as_mut().unwrap() }
}

pub fn set_kernel(kernel: Kernel) {
    unsafe { KERNEL = Some(kernel) }
}
