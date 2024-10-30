use alloc::{boxed::Box, vec::Vec};

use crate::{
    idt::registers::Registers,
    mem::{frame::FrameAllocator, heap::Heap, paging::directory::PageDirectory},
    signal::Signal,
};

pub mod scheduler;

#[repr(C)]
pub struct Process {
    pub page_directory: *mut PageDirectory,
    pub heap: Heap,
    pub signals: Vec<Signal>,
    pub signal_callback: Option<Box<dyn Fn(Signal)>>,
    pub stack: *mut u8,
    pub registers: Registers,
    pub pid: usize,
}

impl Process {
    pub fn new(page_directory: *mut PageDirectory, heap: Heap, pid: usize) -> Self {
        Self {
            page_directory,
            heap,
            signals: Vec::new(),
            signal_callback: None,
            stack: core::ptr::null_mut(),
            registers: Registers::new(),
            pid,
        }
    }

    pub fn push_signal(&mut self, signal: Signal) {
        self.signals.push(signal);
    }

    pub fn execute_signals(&mut self) {
        if let Some(callback) = &self.signal_callback {
            while let Some(signal) = self.signals.pop() {
                callback(signal);
            }
        }
    }

    pub fn pop_signal(&mut self) -> Option<Signal> {
        self.signals.pop()
    }
}
