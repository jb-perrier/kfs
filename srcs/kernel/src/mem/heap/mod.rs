use core::alloc::{GlobalAlloc, Layout};

use block::HeapBlock;

use crate::{debug, error::KernelError, infinite_loop, kernel, text, trace};

use super::frame::FrameAllocator;

mod block;
pub use block::*;

mod error;
pub use error::*;

#[global_allocator]
static mut HEAP_ALLOCATOR: HeapAllocator = HeapAllocator {};
pub struct HeapAllocator {}

pub struct Heap {
    blocks: *mut HeapBlock,
}

impl Heap {
    pub fn new(start: usize, size: usize) -> Self {
        Heap {
            blocks: HeapBlock::new(start, size),
        }
    }

    pub const fn empty() -> Self {
        Heap {
            blocks: core::ptr::null_mut(),
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

    pub fn allocate(&mut self, mut layout: Layout) -> Result<*mut u8, Error> {
        trace!();

        debug! {
            text::write_str("Allocating: ");
            text::write_num!(layout.size());
            text::write_str(" bytes, align: ");
            text::write_num!(layout.align());
            text::write_str("\n");
        }
        
        let mut current = self.blocks;
        loop {
            let block = unsafe { &mut *current };
            match block.allocate(layout) {
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
        trace!();
        debug! {
            text::write_str("Deallocating size: ");
            text::write_num!(self.get_size_from_ptr(ptr).unwrap());
            text::write_str("\n");
        }
        
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

    pub fn get_size_from_ptr(&self, ptr: *mut u8) -> Result<usize, Error> {
        let mut current = self.blocks;
        loop {
            let block = unsafe { &*current };
            if let Ok(size) = block.get_size_from_ptr(ptr) {
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

    pub fn get_free_size(&self) -> usize {
        let mut current = self.blocks;
        let mut size = 0;
        loop {
            let block = unsafe { &*current };
            size += block.get_free_size();
            if let Some(next) = block.next() {
                current = next;
            } else {
                break;
            }
        }
        size
    }
}

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let heap = &mut kernel().process.heap;
        match heap.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => {
                panic!("Failed to allocate memory\n");
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let heap = &mut kernel().process.heap;
        if let Err(_) = heap.deallocate(ptr) {
            panic!("Failed to deallocate memory");
        }
    }
}