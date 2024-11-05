use core::alloc::Layout;

use alloc::{boxed::Box, vec::Vec};

use crate::{
    asm::{GeneralRegisters, HandlerRegisters, InterruptRegisters},
    kernel::kernel,
    mem::{
        frame::{FrameAllocator, FRAME_SIZE},
        heap::Heap,
        paging::directory::PageDirectory,
    },
    signal::Signal,
    text,
};

pub mod scheduler;

const PROCESS_USER_STACK_SIZE: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Start,
    Running,
    Thread,
    Stopped,
}

pub struct Process {
    pub page_directory: *mut PageDirectory,
    pub heap: Heap,

    pub signals: Vec<Signal>,
    pub signal_callback: Option<Box<dyn Fn(Signal)>>,

    pub stack_bottom: *mut u8,
    pub stack_ptr: *mut u8,
    pub stack_top: *mut u8,

    pub pid: usize,
    pub owner: usize,
    pub state: ProcessState,

    pub func: fn() -> (),
}

impl Process {
    pub fn kernel(page_directory: *mut PageDirectory, heap: Heap, pid: usize) -> Self {
        Self {
            page_directory,
            heap,
            signals: Vec::new(),
            signal_callback: None,
            stack_bottom: core::ptr::null_mut(),
            stack_ptr: core::ptr::null_mut(),
            stack_top: core::ptr::null_mut(),
            pid,
            owner: 0,
            state: ProcessState::Running,
            func: || (),
        }
    }

    pub fn user(pid: usize, owner: usize, func: fn() -> ()) -> Self {
        let page_directory = kernel().page_directory;
        let frame = kernel().frame_allocator.allocate_many(16).unwrap();
        let mut heap = Heap::new(frame.addr(), FRAME_SIZE * 16);

        let layout = Layout::from_size_align(PROCESS_USER_STACK_SIZE, 16).unwrap();
        let stack = heap.allocate(layout).unwrap();
        let stack_top = unsafe { stack.add(PROCESS_USER_STACK_SIZE) };

        Self {
            page_directory,
            heap,
            signals: Vec::new(),
            signal_callback: None,
            stack_bottom: stack,
            stack_ptr: stack_top,
            stack_top,
            pid,
            owner,
            state: ProcessState::Start,
            func,
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
