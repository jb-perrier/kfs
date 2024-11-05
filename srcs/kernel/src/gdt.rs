#![allow(clippy::too_many_arguments)]

use core::ptr::addr_of;

use crate::error::KernelError;

use super::asm;

const SEGMENT_DATA_RD: u8 = 0x00; // Read-Only
const SEGMENT_DATA_RDA: u8 = 0x01; // Read-Only, accessed
const SEGMENT_DATA_RDWR: u8 = 0x02; // Read/Write
const SEGMENT_DATA_RDWRA: u8 = 0x03; // Read/Write, accessed
const SEGMENT_DATA_RDEXPD: u8 = 0x04; // Read-Only, expand-down
const SEGMENT_DATA_RDEXPDA: u8 = 0x05; // Read-Only, expand-down, accessed
const SEGMENT_DATA_RDWREXPD: u8 = 0x06; // Read/Write, expand-down
const SEGMENT_DATA_RDWREXPDA: u8 = 0x07; // Read/Write, expand-down, accessed
const SEGMENT_CODE_EX: u8 = 0x08; // Execute-Only
const SEGMENT_CODE_EXA: u8 = 0x09; // Execute-Only, accessed
const SEGMENT_CODE_EXRD: u8 = 0x0A; // Execute/Read
const SEGMENT_CODE_EXRDA: u8 = 0x0B; // Execute/Read, accessed
const SEGMENT_CODE_EXC: u8 = 0x0C; // Execute-Only, conforming
const SEGMENT_CODE_EXCA: u8 = 0x0D; // Execute-Only, conforming, accessed
const SEGMENT_CODE_EXRDC: u8 = 0x0E; // Execute/Read, conforming
const SEGMENT_CODE_EXRDCA: u8 = 0x0F; // Execute/Read, conforming, accessed

const KERNEL_CODE_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 0, SEGMENT_CODE_EXRD);
const KERNEL_DATA_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 0, SEGMENT_DATA_RDWR);
const KERNEL_STACK_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 0, SEGMENT_DATA_RDWREXPD);

const USER_CODE_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 3, SEGMENT_CODE_EXRD);
const USER_DATA_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 3, SEGMENT_DATA_RDWR);
const USER_STACK_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 3, SEGMENT_DATA_RDWREXPD);

static mut GDT_DESCRIPTOR_PTR: *mut GdtDescriptor = unsafe { 0x00000800 as *mut GdtDescriptor };
// static mut TSS: Tss = Tss::new();

const GDT_SIZE: usize = 7;

static mut GDT: [GdtEntry; GDT_SIZE] = [GdtEntry::new(0,0,0,0); GDT_SIZE];

#[repr(C, packed)]
#[derive(Clone, Copy)]
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

pub fn init() -> Result<(), KernelError> {
    unsafe {
        GDT[0] = GdtEntry::new(0, 0, 0, 0);
        GDT[1] = GdtEntry::new(0, 0xFFFFFFFF, KERNEL_CODE_SEGMENT, 0xCF);
        GDT[2] = GdtEntry::new(0, 0xFFFFFFFF, KERNEL_DATA_SEGMENT, 0xCF);
        GDT[3] = GdtEntry::new(0, 0xFFFFFFFF, KERNEL_STACK_SEGMENT, 0xCF);
        GDT[4] = GdtEntry::new(0, 0xFFFFFFFF, USER_CODE_SEGMENT, 0xCF);
        GDT[5] = GdtEntry::new(0, 0xFFFFFFFF, USER_DATA_SEGMENT, 0xCF);
        GDT[6] = GdtEntry::new(0, 0xFFFFFFFF, USER_STACK_SEGMENT, 0xCF);
        // GDT[7] = GdtEntry::new(addr_of!(TSS) as u32, core::mem::size_of::<Tss>() as u32, 0xE9, 0x00);
    }

    unsafe {
        *GDT_DESCRIPTOR_PTR = GdtDescriptor {
            size: (core::mem::size_of::<[GdtEntry; GDT_SIZE]>() - 1) as u16,
            offset: addr_of!(GDT) as *const _ as u64,
        };
        asm::load_gdt(GDT_DESCRIPTOR_PTR);
    }
    if asm::check_gdt() != 0 {
        return Err(KernelError::InvalidGDT);
    }
    Ok(())
}

const fn build_access(
    descriptor_type: u16,
    present: u16,
    system_available: u16,
    long_mode: u16,
    size: u16,
    granularity: u16,
    privilege: u16,
    access: u8,
) -> u8 {
    ((descriptor_type << 0x04)
        | (present << 0x07)
        | (system_available << 0x0C)
        | (long_mode << 0x0D)
        | (size << 0x0E)
        | (granularity << 0x0F)
        | ((privilege & 0x03) << 0x05)
        | access as u16) as u8
}
