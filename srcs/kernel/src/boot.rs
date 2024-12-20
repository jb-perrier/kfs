use multiboot::information::{MemoryManagement, MemoryType, Multiboot, PAddr};

use crate::{infinite_loop, text};

struct Mem;

impl MemoryManagement for Mem {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let ptr = core::mem::transmute(addr as u32);
        Some(core::slice::from_raw_parts(ptr, size))
    }
    
    // If you only want to read fields, you can simply return `None`.
    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }
    
    unsafe fn deallocate(&mut self, addr: PAddr) {
        if addr != 0 {
            unimplemented!()
        }
    }
}

static mut MEM: Mem = Mem;

pub fn init<'a, 'b>(magic: usize, multiboot: usize) -> Option<Multiboot<'a, 'b>> {
    if magic != 0x2BADB002 {
        text::write_str_with_colors("Unknown multiboot ! magic: ", text::Colors::Red, text::Colors::Black);
        text::write_num!(magic);
        infinite_loop!();
    }

    let multi = unsafe { Multiboot::from_ptr(multiboot as u64, &mut MEM) };
    
    multi
}