use core::ptr::addr_of;

use directory::{PageDirectory, PageDirectoryEntryBuilder};
use table::{PageTable, PageTableEntry};

use crate::asm;

pub mod directory;
pub mod table;

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new();

const PAGE_SIZE: u32 = 4096;

pub fn init(memory_block: (u32, u32)) -> Result<(), ()> {
    let start = memory_block.0;
    let end = memory_block.1;
    unsafe {
        for i in 0..1024 {
            let mut table = &mut PAGE_DIRECTORY.tables[i];
            *table = PageDirectoryEntryBuilder::new()
                .unallocated()
                .build();
        }

        let kernel_end_aligned = next_aligned_from_addr(asm::_KERNEL_END as u32, PAGE_SIZE);

        // Add few pages de kernel directory
        for i in 0..10 {
            let mut table = &mut PAGE_DIRECTORY.tables[i];
            let page_addr = kernel_end_aligned + i as u32 * PAGE_SIZE;
            
            table.set_address(page_addr);
            table.set_present(true);
            table.set_read_write(true);
        }

        // TODO: keep track of allocated pages
        
        asm::set_page_directory(addr_of!(PAGE_DIRECTORY) as *const _ as *const u32);
        asm::enable_paging();
    }
    Ok(())
}

// get next aligned address starting from addr
pub fn next_aligned_from_addr(addr: u32, align: u32) -> u32 {
    if addr % align == 0 {
        addr
    } else {
        (addr / PAGE_SIZE + 1) * PAGE_SIZE
    }
}

// based on the address, get the page that the address is in
pub fn get_page_from_addr(addr: u32) -> u32 {
    if addr % PAGE_SIZE == 0 {
        addr
    } else {
        (addr / PAGE_SIZE + 1) * PAGE_SIZE
    }
}