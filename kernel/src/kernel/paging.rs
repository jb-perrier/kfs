use core::ptr::addr_of;

use super::asm;

#[repr(C, align(4096))]
struct PageDirectory([u32; 1024]);

#[repr(C, align(4096))]
struct PageTable([u32; 1024]);

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory([0; 1024]);
static mut FIRST_PAGE_TABLE: PageTable = PageTable([0; 1024]);

pub fn init_paging() {
    unsafe {
        for i in 0..1024 {
            FIRST_PAGE_TABLE.0[i] = (i * 0x1000) as u32 | 0x3;
        }

        PAGE_DIRECTORY.0[0] = (&FIRST_PAGE_TABLE.0 as *const _ as u32) | 0x3; // Present + Read/Write

        asm::set_page_directory(addr_of!(PAGE_DIRECTORY) as *const _ as *const u32);
        asm::enable_paging();
    }
}
