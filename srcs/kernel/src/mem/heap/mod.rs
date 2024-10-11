use core::alloc::GlobalAlloc;

use block::HeapBlock;

use crate::{error::KernelError, infinite_loop, text};

use super::frame::FrameAllocator;

mod block;
pub use block::*;

mod error;
pub use error::*;

static mut HEAP: Heap = Heap::empty();

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

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        text::write_str("Allocating ");
        text::write_num!(layout.size());
        text::write_str(" bytes\n");
        let heap = &mut HEAP;
        match heap.allocate(layout.size()) {
            Ok(ptr) => ptr,
            Err(_) => {
                panic!("Failed to allocate memory\n");
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let heap = &mut HEAP;
        if let Err(_) = heap.deallocate(ptr) {
            panic!("Failed to deallocate memory");
        }
    }
    
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        // SAFETY: the safety contract for `alloc` must be upheld by the caller.
        let ptr = unsafe { self.alloc(layout) };
        if !ptr.is_null() {
            // SAFETY: as allocation succeeded, the region from `ptr`
            // of size `size` is guaranteed to be valid for writes.
            unsafe { core::ptr::write_bytes(ptr, 0, size) };
        }
        ptr
    }
    
    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller must ensure that the `new_size` does not overflow.
        // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
        let new_layout = unsafe { core::alloc::Layout::from_size_align_unchecked(new_size, layout.align()) };
        // SAFETY: the caller must ensure that `new_layout` is greater than zero.
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            // SAFETY: the previously allocated block cannot overlap the newly allocated block.
            // The safety contract for `dealloc` must be upheld by the caller.
            unsafe {
                core::ptr::copy_nonoverlapping(ptr, new_ptr, core::cmp::min(layout.size(), new_size));
                self.dealloc(ptr, layout);
            }
        }
        new_ptr
    }
}