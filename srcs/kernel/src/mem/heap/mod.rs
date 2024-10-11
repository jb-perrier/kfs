use block::HeapBlock;

use crate::{error::KernelError, infinite_loop};

use super::frame::FrameAllocator;

mod block;
pub use block::*;

mod error;
pub use error::*;

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

    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, Error> {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &mut *current };
            match block.allocate(size) {
                Ok(ptr) => return Ok(ptr),
                Err(Error::OutOfMemory) => { /* moving to next block */ }
                Err(e) => return Err(e),
            }
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        Err(Error::OutOfMemory)
    }

    pub fn deallocate(&mut self, ptr: *mut u8) -> Result<(), Error> {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &mut *current };
            match block.deallocate(ptr) {
                Ok(()) => return Ok(()),
                Err(Error::Unallocated) => {}
                Err(e) => return Err(e),
            }
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        Err(Error::Unallocated)
    }

    pub fn is_allocated(&self, ptr: *mut u8) -> bool {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &*current };
            if block.is_allocated(ptr) {
                return true;
            }
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        false
    }

    pub fn get_size(&self, ptr: *mut u8) -> Result<usize, Error> {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &*current };
            if let Ok(size) = block.get_size(ptr) {
                return Ok(size);
            }
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        Err(Error::Unallocated)
    }
}
