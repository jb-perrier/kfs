use core::ops::Deref;

use crate::mem::paging::directory::PageDirectory;

#[derive(Debug, Clone, Copy)]
pub struct PhysAddr(*mut u8);

impl PhysAddr {
    pub fn from_ptr(address: *mut u8) -> Self {
        PhysAddr(address)
    }

    pub const fn from_usize(address: usize) -> Self {
        PhysAddr(address as *mut u8)
    }
}

impl Deref for PhysAddr {
    type Target = *mut u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for PhysAddr {
    fn from(address: usize) -> Self {
        PhysAddr::from_usize(address)
    }
}

impl From<*mut u8> for PhysAddr {
    fn from(address: *mut u8) -> Self {
        PhysAddr(address)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirtAddr(*mut u8);

impl VirtAddr {
    pub fn from_ptr(address: *mut u8) -> Self {
        VirtAddr(address)
    }

    pub const fn from_usize(address: usize) -> Self {
        VirtAddr(address as *mut u8)
    }

    pub fn physical(&self, dir: &PageDirectory) -> PhysAddr {
        let (table_index, entry_index, offset) = self.decompose();
        let entry = dir.table(table_index).entry(entry_index);
        PhysAddr::from_usize(entry.address() + offset)
    }

    pub fn decompose(&self) -> (usize, usize, usize) {
        let dir = (self.0.addr() >> 22) & 0x3FF;
        let table = (self.0.addr() >> 12) & 0x3FF;
        let offset = self.0.addr() & 0xFFF;
        (dir, table, offset)
    }
}

impl Deref for VirtAddr {
    type Target = *mut u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for VirtAddr {
    fn from(address: usize) -> Self {
        VirtAddr::from_usize(address)
    }
}

impl From<*mut u8> for VirtAddr {
    fn from(address: *mut u8) -> Self {
        VirtAddr(address)
    }
}
