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

#[derive(Debug, Clone, Copy)]
pub struct Heap {
    blocks: Option<*mut HeapBlock>,
}

impl Heap {
    pub const fn empty() -> Self {
        Heap {
            blocks: None,
        }
    }

    pub fn new_from_range(start: usize, size: usize) -> Self {
        Heap {
            blocks: Some(HeapBlock::new(start, size)),
        }
    }

    pub fn add_block(&mut self, start: usize, size: usize) {
        let new_block = HeapBlock::new(start, size);
        match self.blocks {
            None => self.blocks = Some(new_block),
            Some(first_block) => add_block_to_chain(first_block, new_block),
        }
        
    }

    pub fn allocate(&mut self, mut layout: Layout) -> Result<*mut u8, HeapError> {
        trace!();

        debug! {
            text::write_str("Allocating: ");
            text::write_num!(layout.size());
            text::write_str(" bytes, align: ");
            text::write_num!(layout.align());
            text::write_str("\n");
        }

        let Some(mut current) = self.get_first_block_mut() else {
            return Err(HeapError::OutOfMemory);
        };
        current.allocate_in_chain(layout).map_err(|_| HeapError::OutOfMemory)
    }

    pub fn deallocate(&mut self, ptr: *mut u8) -> Result<(), HeapError> {
        trace!();
        debug! {
            text::write_str("Deallocating size: ");
            text::write_num!(self.get_size_from_ptr(ptr).unwrap());
            text::write_str("\n");
        }

        let Some(mut current) = self.get_first_block_mut() else {
            return Err(HeapError::Unallocated);
        };
        current.deallocate(ptr).map_err(|_| HeapError::Unallocated)
    }

    pub fn is_allocated(&self, ptr: *mut u8) -> bool {
        self.get_size_from_ptr(ptr).is_some()
    }

    pub fn get_size_from_ptr(&self, ptr: *mut u8) -> Option<usize> {
        let Some(current) = self.get_first_block() else {
            return None;
        };
        current.get_size_from_ptr_in_chain(ptr)
    }

    pub fn get_free_size(&self) -> usize {
        let Some(current) = self.get_first_block() else {
            return 0;
        };
        current.get_free_size_in_chain()
    }

    fn get_first_block(&self) -> Option<&HeapBlock> {
        match self.blocks {
            Some(ptr) => Some(unsafe { &*ptr }),
            None => None,
        }
    }

    fn get_first_block_mut(&self) -> Option<&mut HeapBlock> {
        match self.blocks {
            Some(ptr) => Some(unsafe { &mut *ptr }),
            None => None,
        }
    }
}

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let heap = &mut kernel().heap;
        match heap.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => {
                panic!("Failed to allocate memory\n");
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let heap = &mut kernel().heap;
        if let Err(_) = heap.deallocate(ptr) {
            panic!("Failed to deallocate memory");
        }
    }
}

fn add_block_to_chain(first_block: *mut HeapBlock, new_block: *mut HeapBlock) {
    let mut current = first_block;
    loop {
        let heap = unsafe { &mut *current };
        if heap.next().is_none() {
            heap.set_next(new_block);
            break;
        }
        current = heap.next().unwrap();
    }
}