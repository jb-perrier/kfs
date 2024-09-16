use core::ptr::addr_of;

use super::asm;

const GDT_SIZE: usize = 3;

static mut GDT: [GdtEntry; GDT_SIZE] = [
    GdtEntry::new(0, 0, 0, 0),             // Null
    GdtEntry::new(0, 0xFFFFF, 0x9A, 0xCF), // Code
    GdtEntry::new(0, 0xFFFFF, 0x92, 0xCF), // Data
];

#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    size: u16,
    offset: u64,
}

impl GdtEntry {
    pub const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> GdtEntry {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (granularity & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

pub fn init_gdt() -> u32 {
    unsafe {
        let descriptor = GdtDescriptor {
            size: (core::mem::size_of::<[GdtEntry; GDT_SIZE]>() - 1) as u16,
            offset: addr_of!(GDT) as *const _ as u64,
        };
        asm::load_gdt(&descriptor);
        asm::check_gdt()
    }
}
