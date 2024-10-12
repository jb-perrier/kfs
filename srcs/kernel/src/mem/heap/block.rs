use core::{alloc::Layout, mem::size_of};

use crate::{infinite_loop, mem::next_aligned_from_addr, text};

const HEAP_SUB_BLOCK_SIZE: usize = 16;

#[repr(C, align(16))]
pub struct HeapBlock {
    start: usize,
    size: usize,
    alloc_count: u8,
    next: Option<*mut HeapBlock>,
}

impl HeapBlock {
    // start and end should be aligned on 16 bytes
    pub fn new(start: usize, size: usize) -> *mut HeapBlock {        
        let heap_addr = start as *mut HeapBlock;
        let heap = unsafe { &mut *heap_addr };
        
        heap.start = start;
        heap.size = size;
        heap.next = None;

        heap.clear();
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
        for i in self.bitmap_start()..self.data_end() {
            unsafe { core::ptr::write_unaligned(i as *mut u8, 0_u8) };
        }
        self.init_bitmap();
    }

    pub fn allocate(&mut self, layout: Layout) -> Result<*mut u8, super::Error> {
        if self.alloc_count == 255 {
            return Err(super::Error::OutOfMemory);
        }
        let size_in_bitmap = size_in_bitmap(layout.size());
        let hole = self.find_hole(size_in_bitmap, layout.align()).ok_or(super::Error::OutOfMemory)?;
        let uid = self.find_free_uid().ok_or(super::Error::OutOfMemory)?;
        self.allocate_in_bitmap(hole, size_in_bitmap, uid);
        let addr = self.data_start() + hole * HEAP_SUB_BLOCK_SIZE;
        self.alloc_count += 1;
        Ok(addr as *mut u8)
    }

    pub fn deallocate(&mut self, addr: *mut u8) -> Result<(), super::Error> {
        if !self.is_allocated(addr) {
            return Err(super::Error::Unallocated);
        }
        let hole = (addr as usize - self.data_start()) / HEAP_SUB_BLOCK_SIZE;
        self.deallocate_in_bitmap(hole)?;
        self.alloc_count -= 1;
        Ok(())
    }

    pub fn is_allocated(&self, addr: *mut u8) -> bool {
        let hole = (addr as usize - self.data_start()) / HEAP_SUB_BLOCK_SIZE;
        let addr = addr as usize;
        if addr < self.data_start() || addr >= self.data_end() {
            return false;
        }
        let bitmap_start = self.bitmap_start();
        unsafe { core::ptr::read((bitmap_start + hole) as *const u8) != 0 }
    }

    pub fn get_size(&self, addr: *mut u8) -> Result<usize, super::Error> {
        let uid = self.get_uid_from_ptr(addr).ok_or(super::Error::Unallocated)?;
        self.get_size_from_uid(uid).ok_or(super::Error::Unallocated)
    }

    fn get_uid_from_ptr(&self, ptr: *mut u8) -> Option<u8> {
        let bitmap_index = (ptr as usize - self.data_start()) / HEAP_SUB_BLOCK_SIZE;
        let uid = self.get_bitmap_value(bitmap_index)?;
        if uid == 0 {
            return None;
        }
        Some(uid)
    }

    fn get_ptr_from_uid(&self, uid: u8) -> Option<*mut u8> {
        for i in 0..self.bitmap_size() {
            let bitmap_value = self.get_bitmap_value(i)?;
            if bitmap_value == uid {
                return Some((self.data_start() + i * HEAP_SUB_BLOCK_SIZE) as *mut u8);
            }
        }
        None
    }

    fn get_size_from_uid(&self, uid: u8) -> Option<usize> {
        let mut size = 0;
        for i in 0..self.bitmap_size() {
            let bitmap_value = self.get_bitmap_value(i)?;
            if bitmap_value == uid {
                size += 1;
            } else if size > 0 {
                break;
            }
        }
        Some(size * HEAP_SUB_BLOCK_SIZE)
    }

    fn find_free_uid(&self) -> Option<u8> {
        for i in 1..=255 {
            let bitmap_value = self.get_ptr_from_uid(i);
            if bitmap_value.is_none() {
                return Some(i);
            }
        }
        None
    }

    fn allocate_in_bitmap(&self, index: usize, size: usize, uid: u8) {
        let bitmap_start = self.bitmap_start();
        for i in index..(index + size) {
            unsafe { core::ptr::write((bitmap_start + i) as *mut u8, uid) };
        }
    }

    fn deallocate_in_bitmap(&self, mut index: usize) -> Result<(), super::Error> {
        let alloc_uid = self.get_bitmap_value(index).ok_or(super::Error::Unallocated)?;
        while index < self.bitmap_size() {
            let uid = self.get_bitmap_value(index).ok_or(super::Error::Unallocated)?;
            if uid != alloc_uid {
                break;
            }
            self.set_bitmap_value(index, 0);
            index += 1;
        }
        Ok(())
    }

    fn set_bitmap_value(&self, index: usize, value: u8) {
        let bitmap_start = self.bitmap_start();
        if index >= self.bitmap_size() {
            return;
        }
        unsafe { core::ptr::write((bitmap_start + index) as *mut u8, value) };
    }

    fn get_bitmap_value(&self, index: usize) -> Option<u8> {
        let bitmap_start = self.bitmap_start();
        if index >= self.bitmap_size() {
            return None;
        }
        Some(unsafe { core::ptr::read((bitmap_start + index) as *const u8) })
    }

    // bitmap space
    fn find_hole(&self, size: usize, alignment: usize) -> Option<usize> {
        let bitmap_size = self.bitmap_size();

        let mut i = 0;
        while i < bitmap_size && bitmap_size - i > size {
            let addr = self.data_start() + i * HEAP_SUB_BLOCK_SIZE;
            if addr % alignment == 0 && self.fit_in_hole(i, size) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    // bitmap space
    fn fit_in_hole(&self, hole: usize, size: usize) -> bool {
        let bitmap_start = self.bitmap_start();

        let mut i = hole;
        while i < self.bitmap_size() {
            let curr_hole = unsafe { core::ptr::read((bitmap_start + i) as *const u8) };
            if curr_hole != 0 {
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
        self.data_start()
    }

    pub fn bitmap_end(&self) -> usize {
        self.bitmap_start() + self.bitmap_size()
    }

    pub fn bitmap_size(&self) -> usize {
        self.data_size() / HEAP_SUB_BLOCK_SIZE
    }

    pub fn data_start(&self) -> usize {
        let addr = self.start + size_of::<HeapBlock>();
        next_aligned_from_addr(addr, 16)
    }

    pub fn data_end(&self) -> usize {
        self.start + self.size
    }

    pub fn data_size(&self) -> usize {
        self.data_end() - self.data_start()
    }
}

fn size_in_bitmap(size: usize) -> usize {
    if size < HEAP_SUB_BLOCK_SIZE {
        1
    } else {
        size / HEAP_SUB_BLOCK_SIZE
    }
}
