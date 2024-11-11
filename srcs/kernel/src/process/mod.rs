use core::alloc::Layout;

use address::{PhysAddr, VirtAddr};
use alloc::{boxed::Box, vec::Vec};
use process_running::ProcessRunning;
use process_start::ProcessStart;
use process_stopped::ProcessStopped;

use crate::{
    asm::{GeneralRegisters, HandlerRegisters, InterruptRegisters},
    kernel::kernel,
    mem::{
        frame::{FrameAllocator, FRAME_SIZE},
        heap::{self, Heap},
        paging::directory::PageDirectory,
    },
    signal::Signal,
    text,
};

pub mod address;
pub mod scheduler;
pub mod process_running;
pub mod process_start;
pub mod process_stopped;
pub mod fork;

const PROCESS_USER_STACK_FRAME_SIZE: usize = 4;
const PROCESS_USER_STACK_SIZE: usize = PROCESS_USER_STACK_FRAME_SIZE * FRAME_SIZE;
const PROCESS_USER_VIRTUAL_START: VirtAddr = VirtAddr::from_usize(0x40000000);
const PROCESS_USER_HEAP_FRAME_SIZE: usize = 16;
const PROCESS_USER_HEAP_SIZE: usize = PROCESS_USER_HEAP_FRAME_SIZE * FRAME_SIZE;

pub enum Process {
    Start(ProcessStart),
    Running(ProcessRunning),
    Stopped(ProcessStopped),
}

impl Process {
    pub fn new(owner: usize, func: fn() -> ()) -> Process {
        Process::Start(ProcessStart::user(owner, func))
    }

    pub fn state(&self) -> ProcessState {
        match self {
            Process::Start(_) => ProcessState::Start,
            Process::Running(_) => ProcessState::Running,
            Process::Stopped(_) => ProcessState::Stopped,
        }
    }

    pub fn pid(&self) -> ProcessId {
        match self {
            Process::Start(p) => p.pid,
            Process::Running(p) => p.pid,
            Process::Stopped(p) => p.pid,
        }
    }

    pub fn as_start(&self) -> Option<&ProcessStart> {
        match self {
            Process::Start(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_start_mut(&mut self) -> Option<&mut ProcessStart> {
        match self {
            Process::Start(p) => Some(p),
            _ => None,
        }
    }
    
    pub fn as_running(&self) -> Option<&ProcessRunning> {
        match self {
            Process::Running(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_running_mut(&mut self) -> Option<&mut ProcessRunning> {
        match self {
            Process::Running(p) => Some(p),
            _ => None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.as_running().is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Start,
    Running,
    Stopped,
}

#[derive(Debug, Clone, Copy)]
pub struct StackAddr {
    pub bottom: VirtAddr,
    pub top: VirtAddr,
    pub ptr: VirtAddr,

    pub bottom_phys: PhysAddr,
}

pub fn find_free_pid() -> ProcessId {
    let mut pid = 1;
    while kernel().processes.iter().any(|p| p.pid() == ProcessId(pid)) {
        pid += 1;
    }
    ProcessId(pid)
}