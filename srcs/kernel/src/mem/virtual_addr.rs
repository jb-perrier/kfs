pub struct VirtualAddr{
    addr: usize,
}

impl VirtualAddr {
    pub fn new(directory_entry: usize, table_entry: usize, offset: usize) -> VirtualAddr {
        let addr = (directory_entry << 22) | (table_entry << 12) | offset;
        VirtualAddr {
            addr
        }
    }
    pub fn directory_entry(&self) -> usize {
        self.addr >> 22
    }

    pub fn table_entry(&self) -> usize {
        (self.addr >> 12) & 0x3FF
    }

    pub fn offset(&self) -> usize {
        self.addr & 0xFFF
    }
}