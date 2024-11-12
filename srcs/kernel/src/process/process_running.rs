use alloc::{boxed::Box, vec::Vec};

use crate::{mem::{heap::Heap, paging::directory::PageDirectory}, signal::Signal, text};

use super::{address::PhysAddr, ProcessId, StackAddr};

pub struct ProcessRunning {
    pub page_directory: *mut PageDirectory,
    pub heap: Heap,
    pub heap_bottom: PhysAddr,

    pub signals: Vec<Signal>,
    pub signal_callback: Option<Box<dyn Fn(Signal)>>,

    pub stack: StackAddr,

    pub pid: ProcessId,
    pub owner: usize,

    pub parent: ProcessId,
    pub children: Vec<ProcessId>,

    pub func: fn() -> (),

    pub fork: usize,

    pub exit: bool,
}

impl ProcessRunning {
    pub fn init_from_user() {}

    pub fn execute_signals(&mut self) {
        if let Some(callback) = &self.signal_callback {
            while let Some(signal) = self.signals.pop() {
                callback(signal);
            }
        }
    }
}