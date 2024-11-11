use core::ffi::c_void;

use crate::{error::KernelError, libc::memset, process::address::PhysAddr, text, trace};

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

    pub fn allocate_many(&mut self, count: usize) -> Result<PhysAddr, KernelError> {
        if self.next_free + FRAME_SIZE * count > self.memory_block.1 {
            return Err(KernelError::FrameOutOfMemory);
        }
        let addr = self.next_free;
        self.next_free += FRAME_SIZE * count;

        let ptr_i32 = addr as *mut i32;
        unsafe { memset(ptr_i32, 0, (FRAME_SIZE * count) as isize) };

        Ok(PhysAddr::from_usize(addr))
    }

    pub fn allocate(&mut self) -> Result<PhysAddr, KernelError> {
        self.allocate_many(1)
    }

    pub fn position(&self) -> usize {
        self.next_free
    }

    pub fn capacity(&self) -> usize {
        (self.memory_block.1 - self.memory_block.0) / FRAME_SIZE
    }
}
