use core::{ffi::c_void, mem::size_of, ptr::addr_of};

use directory::{PageDirectory, PageDirectoryEntryBuilder};
use table::{PageTable, PageTableEntry, PageTableEntryBuilder};

use crate::{asm, error::KernelError, kernel::Kernel, vga};

use super::{
    frame::{self, FrameAllocator},
    next_aligned_from_addr,
};

pub mod directory;
pub mod table;

pub const PAGE_SIZE: usize = 4096;

pub fn init(kernel: &mut Kernel) -> Result<(), KernelError> {
    unsafe {
        
        let addr = PageDirectory::new(kernel.frame_allocator.as_mut().unwrap(), true)?;
        addr.as_mut().unwrap().identity();
        kernel.process.page_directory.insert(addr);
        
        asm::set_page_directory(addr);
        asm::enable_paging();

        vga::write_str_with_colors(
            "Pagination initialized !",
            &vga::Colors::Green,
            &vga::Colors::Black,
        );
        vga::write_str_with_colors(
            " Kernel virtual space mapped as identity\n",
            &vga::Colors::DarkGray,
            &vga::Colors::Black,
        );
    }
    Ok(())
}

// based on the address, get the page (identity) that the address is in
pub fn get_page_from_addr(addr: usize) -> usize {
    if addr % PAGE_SIZE == 0 {
        addr
    } else {
        (addr / PAGE_SIZE + 1) * PAGE_SIZE
    }
}
