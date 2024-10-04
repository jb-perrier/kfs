use core::{ffi::c_void, mem::size_of, ptr::addr_of};

use directory::{PageDirectory, PageDirectoryEntryBuilder};
use table::{PageTable, PageTableEntry, PageTableEntryBuilder};

use crate::{asm, vga};

use super::next_aligned_from_addr;

pub mod directory;
pub mod table;

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new();

pub const PAGE_SIZE: u32 = 4096;

pub fn init(memory_block: (u32, u32)) -> Result<(), ()> {
    let start = memory_block.0;
    let end = memory_block.1;
    unsafe {
        let table_size = size_of::<PageTable>() as u32;
        // Manually allocate enough space for the all kernel tables (they are too big to fit in static memory)
        for i in 0..1024 {
            let mut table = (start + i * table_size) as *mut PageTable;
            let mut table = &mut *table;

            // vga::write_num_hex!(start + i * table_size);
            // vga::write_num_hex!(i * 1024 * 4096);
            // vga::write_str("\n");
            let base_addr = i * 1024 * 4096;
            for j in 0..1024 {
                let block_addr = base_addr + j * 4096;
                
                // vga::write_num_hex!(base_addr + j * 4096);
                // vga::write_str(" - ");

                let mut entry = &mut table.entries[j as usize];
                *entry = PageTableEntryBuilder::new()
                    .present(true)
                    .read_write(true)
                    .user(false)
                    .address(block_addr)
                    .build();

                // vga::write_num_hex!((*entry).address());
                // vga::write_str("\n");
            }

            let mut dir_entry = &mut PAGE_DIRECTORY.tables[i as usize];
            *dir_entry = PageDirectoryEntryBuilder::new()
                .present(true)
                .read_write(true)
                .user(false)
                .address(table as *const _ as u32)
                .build();
        }

        // TODO: keep track of allocated pages

        asm::set_page_directory(addr_of!(PAGE_DIRECTORY) as *const _ as *const c_void);
        asm::enable_paging();

        vga::write_str_with_colors("Pagination initialized !", &vga::Colors::Green, &vga::Colors::Black);
        vga::write_str_with_colors(" Kernel virtual space mapped as identity\n", &vga::Colors::DarkGray, &vga::Colors::Black);
    }
    Ok(())
}

// based on the address, get the page that the address is in
pub fn get_page_from_addr(addr: u32) -> u32 {
    if addr % PAGE_SIZE == 0 {
        addr
    } else {
        (addr / PAGE_SIZE + 1) * PAGE_SIZE
    }
}
