use core::mem::size_of;

use crate::{
    bits::{get_bit_at, set_bit_at},
    error::KernelError,
    mem::{
        frame::{Frame, FrameAllocator}, next_aligned_from_addr, paging::get_directory_entry_from_phys_addr, previous_aligned_from_addr, virtual_addr::VirtualAddr
    },
    text,
};

use super::{
    get_page_index_from_phys_addr, get_table_entry_from_phys_addr,
    table::{PageTable, PageTableEntryBuilder},
};

#[repr(C, align(4096))]
#[derive(Clone, Copy)]
pub struct PageDirectory {
    pub tables: [PageDirectoryEntry; 1024],
}

impl PageDirectory {
    pub fn new_from_frame_allocator(
        frame_allocator: &mut FrameAllocator,
        is_user: bool,
    ) -> Result<*mut PageDirectory, KernelError> {
        let page_directory_addr = frame_allocator.allocate_zeroed()? as *mut PageDirectory;
        let page_directory = unsafe { &mut *page_directory_addr };
        let table_size = size_of::<PageTable>();

        for (i, dir_entry) in &mut page_directory.tables.iter_mut().enumerate() {
            let table_addr = frame_allocator.allocate_zeroed()? as *mut PageTable;
            let mut table = unsafe { &mut *table_addr };

            for (j, table_entry) in table.entries.iter_mut().enumerate() {
                *table_entry = PageTableEntryBuilder::new().build(); // filled later
            }

            *dir_entry = PageDirectoryEntryBuilder::new()
                .present(false)
                .read_write(false)
                .user(is_user)
                .address(table as *const _ as usize)
                .build();
        }
        Ok(page_directory_addr)
    }
    #[allow(clippy::never_loop)]
    pub fn identity(&mut self, max_addr: usize) {
        let is_user = self.tables[0].user();
        let mut active_dir = 0;
        for (i, dir_entry) in &mut self.tables.iter_mut().enumerate() {
            let mut table = unsafe { &mut *dir_entry.table() };

            let base_addr = i * 1024 * 4096;
            for (j, table_entry) in table.entries.iter_mut().enumerate() {
                let block_addr = base_addr + j * 4096;
                *table_entry = PageTableEntryBuilder::new()
                    .address(block_addr)
                    .present(false)
                    .read_write(true)
                    .user(is_user)
                    .build();
            }
            *dir_entry = PageDirectoryEntryBuilder::new()
                .present(false)
                .read_write(true)
                .user(is_user)
                .address(table as *const _ as usize)
                .build();
        }
    }

    pub fn map_range_as_identity(&mut self, start: usize, end: usize, is_user: bool) {
        let block_start = previous_aligned_from_addr(start, 4096);
        let block_end = next_aligned_from_addr(end, 4096);

        text::write_str("Mapping range as identity: 0x");
        text::write_num_hex!(block_start);
        text::write_str(" - 0x");
        text::write_num_hex!(block_end);
        text::write_str("\n");

        let mut curr_block = block_start;
        while curr_block < block_end {
            let directory_entry = get_directory_entry_from_phys_addr(self, curr_block);
            directory_entry.set_present(true);
            let table_entry = get_table_entry_from_phys_addr(self, curr_block);
            *table_entry = PageTableEntryBuilder::new()
                    .address(curr_block)
                    .present(true)
                    .read_write(true)
                    .user(is_user)
                    .build();
            curr_block += 4096;
        }
    }

    pub fn clear(&mut self) {
        for i in 0..1024 {
            self.tables[i] = PageDirectoryEntry(0);
        }
    }

    pub fn add_new_page(&mut self, frame: *mut Frame, is_user: bool) -> Result<(), ()> {
        let frame = frame as usize;
        let entry = PageDirectoryEntryBuilder::new()
            .address(frame)
            .present(true)
            .read_write(true)
            .build();
        for entry in &self.tables {
            let table = unsafe { &mut *entry.table() };
            for mut entry in &mut table.entries {
                if !entry.present() {
                    *entry = PageTableEntryBuilder::new()
                        .address(frame)
                        .present(true)
                        .read_write(true)
                        .user(is_user)
                        .build();
                    return Ok(());
                }
            }
        }
        // TODO: Out of memory (not enough space in the page directory)
        Err(())
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageDirectoryEntry(pub usize);

impl PageDirectoryEntry {
    pub fn flags(&self) -> usize {
        self.0 & 0xFFF
    }

    pub fn address(&self) -> usize {
        self.0 & 0xFFFFF000
    }

    pub fn set_address(&mut self, address: usize) {
        let shifted_address = address & 0xFFFFF000;
        let flags = self.flags();
        self.0 = shifted_address | flags;
    }

    pub fn present(&self) -> bool {
        get_bit_at(self.0, 0)
    }

    pub fn set_present(&mut self, present: bool) {
        set_bit_at(&mut self.0, 0, present);
    }

    pub fn read_write(&self) -> bool {
        get_bit_at(self.0, 1)
    }

    pub fn set_read_write(&mut self, read_write: bool) {
        set_bit_at(&mut self.0, 1, read_write);
    }

    pub fn user(&self) -> bool {
        get_bit_at(self.0, 2)
    }

    pub fn set_user(&mut self, user: bool) {
        set_bit_at(&mut self.0, 2, true);
    }

    pub fn table(&self) -> *mut PageTable {
        (self.0 & 0xFFFFF000) as *mut PageTable
    }
}

impl Default for PageDirectoryEntry {
    fn default() -> Self {
        PageDirectoryEntryBuilder::new()
            .read_write(true)
            .user(false)
            .build()
    }
}

pub struct PageDirectoryEntryBuilder {
    entry: PageDirectoryEntry,
}

impl PageDirectoryEntryBuilder {
    pub fn new() -> Self {
        PageDirectoryEntryBuilder {
            entry: PageDirectoryEntry(0),
        }
    }

    pub fn present(mut self, present: bool) -> Self {
        self.entry.set_present(present);
        self
    }

    pub fn read_write(mut self, read_write: bool) -> Self {
        self.entry.set_read_write(read_write);
        self
    }

    pub fn user(mut self, user: bool) -> Self {
        self.entry.set_user(user);
        self
    }

    pub fn address(mut self, address: usize) -> Self {
        self.entry.set_address(address);
        self
    }

    pub fn unallocated(mut self) -> Self {
        self.address(0).present(false).read_write(false).user(false)
    }

    pub fn build(self) -> PageDirectoryEntry {
        self.entry
    }
}
