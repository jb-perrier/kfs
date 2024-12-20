use crate::{bits::{get_bit_at, set_bit_at}, mem::frame::FRAME_SIZE};



#[repr(C, align(4096))]
#[derive(Clone, Copy)]
pub struct PageTable {
    pub entries: [PageTableEntry; 1024]
}

impl PageTable {
    pub const fn new() -> Self {
        PageTable { entries: [PageTableEntry(0); 1024] }
    }

    pub fn clear(&mut self) {
        for i in 0..1024 {
            self.entries[i] = PageTableEntry(0);
        }
    }

    pub fn entry(&self, index: usize) -> &PageTableEntry {
        &self.entries[index]
    }

    pub fn entry_mut(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub usize);

impl PageTableEntry {

    pub fn flags(&self) -> usize {
        self.0 & 0xFFF
    }

    pub fn address(&self) -> usize {
        self.0 & 0xFFFFF000
    }

    pub fn set_address(&mut self, address: usize) {
        assert_eq!(address % FRAME_SIZE, 0);
        let masked_address = address & 0xFFFFF000;
        let flags = self.flags();
        self.0 = masked_address | flags;
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
}

pub struct PageTableEntryBuilder {
    entry: PageTableEntry
}

impl PageTableEntryBuilder {
    pub fn new() -> Self {
        PageTableEntryBuilder { entry: PageTableEntry(0) }
    }

    pub fn unallocated(mut self) -> Self {
        self.address(0).present(false).read_write(false).user(false)
    }

    pub fn address(mut self, address: usize) -> Self {
        self.entry.set_address(address);
        self
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

    pub fn build(self) -> PageTableEntry {
        self.entry
    }
}