const IDT_SIZE: usize = 256;
static mut IDT: [IDTEntry; IDT_SIZE] = [IDTEntry::new(0, 0, 0); IDT_SIZE];

#[repr(C, packed)]
pub struct IDTEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_high: u16,
}

impl IDTEntry {
    pub const fn new(offset: u32, selector: u16, type_attr: u8) -> IDTEntry {
        IDTEntry {
            offset_low: (offset & 0xFFFF) as u16,
            selector,
            zero: 0,
            type_attr,
            offset_high: ((offset >> 16) & 0xFFFF) as u16,
        }
    }
}

#[repr(C, packed)]
pub struct IDTPointer {
    limit: u16,
    base: u32,
}