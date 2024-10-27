use core::ffi::c_void;

use crate::{error::KernelError, libc::memset, text};

use super::{next_aligned_from_addr, previous_aligned_from_addr};

pub const FRAME_SIZE: usize = 4096;

//TODO: move struct logic to static

#[repr(C, align(4096))]
pub struct Frame {
    data: [u8; FRAME_SIZE],
}

pub struct FrameAllocator {
    memory_block: (usize, usize),
    next_free: usize,
}

impl FrameAllocator {
    pub fn new(mut block: (usize, usize)) -> Self {
        block.0 = next_aligned_from_addr(block.0, FRAME_SIZE);
        block.1 = previous_aligned_from_addr(block.1, FRAME_SIZE);
        FrameAllocator {
            memory_block: block,
            next_free: block.0,
        }
    }

    pub fn allocate(&mut self) -> Result<*mut Frame, KernelError> {
        if self.next_free + FRAME_SIZE > self.memory_block.1 {
            return Err(KernelError::FrameOutOfMemory);
        }
        let frame = self.next_free;
        self.next_free += FRAME_SIZE;

        Ok(frame as *mut Frame)
    }

    pub fn allocate_zeroed(&mut self) -> Result<*mut Frame, KernelError> {
        let frame = self.allocate()?;
        unsafe { memset(frame as *mut i32, 0, FRAME_SIZE as isize) };
        Ok(frame)
    }

    pub fn position(&self) -> usize {
        self.next_free
    }

    pub fn capacity(&self) -> usize {
        (self.memory_block.1 - self.memory_block.0) / FRAME_SIZE
    }
}
