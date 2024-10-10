use core::mem::size_of;

use crate::{error::KernelError, infinite_loop, text};

const HEAP_SUB_BLOCK_SIZE: usize = 16;

#[repr(C, align(16))]
pub struct HeapBlock {
    start: usize,
    size: usize,
    next: Option<*mut HeapBlock>,
}

impl HeapBlock {
    // start and end should be aligned on 16 bytes
    pub fn new(start: usize, size: usize) -> *mut HeapBlock {
        text::write_str("HeapBlock: 0x");
        text::write_num_hex!(start);
        text::write_str(" - 0x");
        text::write_num_hex!(start + size);
        text::write_str("\n");
        
        let heap_addr = start as *mut HeapBlock;
        let heap = unsafe { &mut *heap_addr };
        
        heap.start = start;
        heap.size = size;
        heap.next = None;
        
        // heap.clear();
        // heap.init_bitmap();
        heap_addr
    }

    pub fn next(&self) -> Option<*mut HeapBlock> {
        self.next
    }

    pub fn set_next(&mut self, next: *mut HeapBlock) {
        self.next = Some(next);
    }

    fn init_bitmap(&mut self) {
        let bitmap_size_in_bitmap = size_in_bitmap(self.bitmap_size());
        for i in self.bitmap_start()..(self.bitmap_start() + bitmap_size_in_bitmap) {
            unsafe { core::ptr::write(i as *mut u8, 1) };
        }
    }

    pub fn clear(&mut self) {
        text::write_str("Clear: 0x");
        text::write_num_hex!(self.bitmap_start());
        text::write_str(" - 0x");
        text::write_num_hex!(self.data_end());
        text::write_str(" => ");
        text::write_num!(self.data_end() - self.bitmap_start());
        text::write_str("\n");

        for i in self.bitmap_start()..self.data_end() {
            unsafe { core::ptr::write_unaligned(i as *mut u8, 0_u8) };
        }
        // self.init_bitmap();
    }

    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, KernelError> {
        let size_in_bitmap = size_in_bitmap(size);
        let hole = self.find_hole(size_in_bitmap)?;
        self.allocate_in_bitmap(hole, size_in_bitmap);
        let addr = self.data_start() + hole * HEAP_SUB_BLOCK_SIZE;

        Ok(addr as *mut u8)
    }

    pub fn deallocate(&mut self, addr: *mut u8, size: usize) -> Result<(), KernelError> {
        let hole = (addr as usize - self.data_start()) / HEAP_SUB_BLOCK_SIZE;
        let size_in_bitmap = size_in_bitmap(size);
        self.deallocate_in_bitmap(hole, size)?;
        Ok(())
    }

    // bitmap space
    fn allocate_in_bitmap(&self, hole: usize, size: usize) {
        let bitmap_start = self.bitmap_start();
        for i in hole..(hole + size) {
            unsafe { core::ptr::write((bitmap_start + i) as *mut u8, 1) };
        }
    }

    // bitmap space
    fn deallocate_in_bitmap(&self, hole: usize, size: usize) -> Result<(), KernelError> {
        let bitmap_start = self.bitmap_start();
        for i in hole..(hole + size) {
            if unsafe { core::ptr::read((bitmap_start + i) as *mut u8) == 0 } {
                return Err(KernelError::FreeUnallocated);
            }
            unsafe { core::ptr::write((bitmap_start + i) as *mut u8, 0) };
        }
        Ok(())
    }

    // bitmap space
    fn find_hole(&self, size: usize) -> Result<usize, KernelError> {
        let bitmap_size = self.bitmap_size();

        let mut i = 0;
        while i < bitmap_size && bitmap_size - i > size {
            if self.fit_in_hole(i, size) {
                return Ok(i);
            }
            i += 1;
        }
        Err(KernelError::HeapOutOfMemory)
    }

    // bitmap space
    fn fit_in_hole(&self, hole: usize, size: usize) -> bool {
        let bitmap_start = self.bitmap_start();

        let mut i = hole;
        while i < self.bitmap_size() {
            let curr_hole = unsafe { core::ptr::read((bitmap_start + i) as *const u8) };
            if curr_hole == 1 {
                return false;
            }
            let curr_size = i - hole;
            if curr_size == size {
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn bitmap_start(&self) -> usize {
        self.start + size_of::<HeapBlock>()
    }

    pub fn bitmap_end(&self) -> usize {
        self.bitmap_start() + self.bitmap_size()
    }

    pub fn bitmap_size(&self) -> usize {
        self.data_size() / HEAP_SUB_BLOCK_SIZE
    }

    pub fn data_start(&self) -> usize {
        self.bitmap_start() + self.bitmap_size()
    }

    pub fn data_end(&self) -> usize {
        self.start + self.size
    }

    pub fn data_size(&self) -> usize {
        self.size - size_of::<HeapBlock>()
    }
}

fn size_in_bitmap(size: usize) -> usize {
    if size < HEAP_SUB_BLOCK_SIZE {
        HEAP_SUB_BLOCK_SIZE
    } else {
        size / HEAP_SUB_BLOCK_SIZE
    }
}
