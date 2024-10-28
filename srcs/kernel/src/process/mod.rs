use alloc::vec::Vec;

use crate::{idt::registers::Registers, mem::{frame::FrameAllocator, heap::Heap, paging::directory::PageDirectory}, signal::Signal};

#[repr(C)]
pub struct Process {
    pub page_directory: *mut PageDirectory,
    pub heap: Heap,
    pub signals: Vec<Signal>,
    pub stack: *mut u8,
    pub registers: Registers,
}

impl Process {
    pub fn new(page_directory: *mut PageDirectory, heap: Heap) -> Self {
        Self {
            page_directory,
            heap,
            signals: Vec::new(),
            stack: core::ptr::null_mut(),
            registers: Registers::new(),
        }
    }
}