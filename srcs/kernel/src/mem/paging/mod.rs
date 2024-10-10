use core::{ffi::c_void, mem::size_of, ptr::addr_of};

use directory::{PageDirectory, PageDirectoryEntry, PageDirectoryEntryBuilder};
use multiboot::information::Multiboot;
use table::{PageTable, PageTableEntry, PageTableEntryBuilder};

use crate::{asm, error::KernelError, infinite_loop, kernel::Kernel, text};

use super::{
    frame::{self, FrameAllocator},
    next_aligned_from_addr,
};

pub mod directory;
pub mod table;

pub const PAGE_SIZE: usize = 4096;

pub fn init(
    frame_allocator: &mut FrameAllocator,
    multiboot: &Multiboot,
) -> Result<*mut PageDirectory, KernelError> {
    let addr = PageDirectory::new_from_frame_allocator(frame_allocator, true)?;

    for mem in multiboot.memory_regions().unwrap() {
        if (mem.base_address() + mem.length() > usize::MAX as u64) {
            continue;
        }
        let start = mem.base_address() as usize;
        let end = mem.base_address() as usize + mem.length() as usize;
        unsafe {
            (*addr).map_range_as_identity(mem.base_address() as usize, end, false);
        }
    }
    // infinite_loop!();
    asm::set_page_directory(addr);
    asm::enable_paging();

    
    text::write_str_with_colors(
        "Pagination initialized !",
        &text::Colors::Green,
        &text::Colors::Black,
    );

    text::write_str_with_colors(
        " Kernel virtual space mapped as identity\n",
        &text::Colors::DarkGray,
        &text::Colors::Black,
    );

    Ok(addr)
}

// based on the address, get the page (identity) that the address is in
pub fn get_page_from_addr(addr: usize) -> usize {
    if addr % PAGE_SIZE == 0 {
        addr
    } else {
        (addr / PAGE_SIZE + 1) * PAGE_SIZE
    }
}

pub fn get_page_index_from_phys_addr(addr: usize) -> (usize, usize, usize) {
    let index_in_pages = addr / 4096;
    let table_entry_index = index_in_pages / 1024;
    let directory_entry_index = table_entry_index / 1024;
    let offset = addr % 4096;

    (directory_entry_index, table_entry_index, offset)
}

pub fn get_directory_entry_from_phys_addr<'a>(
    page_directory: &'a mut PageDirectory,
    addr: usize,
) -> &'a mut PageDirectoryEntry {
    let (directory_index, table_index, offset) = get_page_index_from_phys_addr(addr);
    unsafe { &mut page_directory.tables[directory_index] }
}

pub fn get_table_entry_from_phys_addr<'a>(
    page_directory: &'a mut PageDirectory,
    addr: usize,
) -> &'a mut PageTableEntry {
    let (directory_index, table_index, offset) = get_page_index_from_phys_addr(addr);
    let table = unsafe { &mut *page_directory.tables[directory_index].table() };
    &mut table.entries[table_index]
}
