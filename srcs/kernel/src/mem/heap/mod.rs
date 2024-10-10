use block::HeapBlock;

use crate::{error::KernelError, infinite_loop};

use super::frame::FrameAllocator;

pub mod block;

pub struct Heap {
    blocks: *mut HeapBlock,
}

impl Heap {
    pub fn new(start: usize, size: usize) -> Self {
        Heap {
            blocks: HeapBlock::new(start, size),
        }
    }

    pub fn add_block(&mut self, start: usize, size: usize) {
        let block = HeapBlock::new(start, size);
        let mut current = self.blocks;
        loop {
            let heap = unsafe { &mut *current };
            if heap.next().is_none() {
                heap.set_next(block);
                break;
            }
            current = heap.next().unwrap();
        }
    }

    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, KernelError> {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &mut *current };
            match block.allocate(size) {
                Ok(ptr) => return Ok(ptr),
                // try other blocks before asking for a new page
                Err(KernelError::HeapOutOfMemory) => {}
                Err(e) => return Err(e),
            }
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        Err(KernelError::HeapOutOfMemory)
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) -> Result<(), KernelError> {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &mut *current };
            match block.deallocate(ptr, size) {
                Ok(()) => return Ok(()),
                Err(KernelError::InvalidPointer) => {}
                Err(e) => return Err(e),
            }
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        Err(KernelError::InvalidPointer)
    }
}
