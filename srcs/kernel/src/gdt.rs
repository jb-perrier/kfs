#![allow(clippy::too_many_arguments)]

use core::ptr::addr_of;

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

const USER_CODE_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 0, SEGMENT_CODE_EXRD);
const USER_DATA_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 0, SEGMENT_DATA_RDWR);
const USER_STACK_SEGMENT: u8 = build_access(1, 1, 0, 0, 1, 1, 0, SEGMENT_DATA_RDWREXPD);

static mut GDT_DESCRIPTOR_PTR: *mut GdtDescriptor = unsafe { 0x00000800 as *mut GdtDescriptor };

const GDT_SIZE: usize = 7;

static mut GDT: [GdtEntry; GDT_SIZE] = [
    // null
    GdtEntry::new(0, 0, 0, 0),
    // kernel
    GdtEntry::new(0, 0xFFFFF, KERNEL_CODE_SEGMENT, 0xCF),
    GdtEntry::new(0, 0xFFFFF, KERNEL_DATA_SEGMENT, 0xCF),
    GdtEntry::new(0, 0xFFFFF, KERNEL_STACK_SEGMENT, 0xCF),
    // user
    GdtEntry::new(0, 0xFFFFF, USER_CODE_SEGMENT, 0xCF),
    GdtEntry::new(0, 0xFFFFF, USER_DATA_SEGMENT, 0xCF),
    GdtEntry::new(0, 0xFFFFF, USER_STACK_SEGMENT, 0xCF),
];

// LINK : Previous Task Link
// ESP : Stack Pointers used to load the stack when a privilege level change occurs from a lower privilege level to a higher one.
// SS : Stack Segment Selector used to load the stack when a privilege level change occurs from a lower privilege level to a higher one.
// CR3 : Page Directory Base Address
// EIP : Instruction Pointer
// EFLAGS : Flags Register
// IOPB : I/O Permission Bit Map Base Address
// SSP : Shadow Stack pointer
#[repr(C, packed)]
pub struct Tss {
    link: u16,
    _link_reserved: u16,
    esp0: u32,
    ss0: u16, 
    _ss0_reserved: u16,
    esp1: u32,
    ss1: u16,
    _ss1_reserved: u16,
    esp2: u32,
    ss2: u16,
    _ss2_reserved: u16,
    cr3: u32,
    eip: u32,
    eflags: u32,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    es: u16,
    _es_reserved: u16,
    cs: u16,
    _cs_reserved: u16,
    ss: u16,
    _ss_reserved: u16,
    ds: u16,
    _ds_reserved: u16,
    fs: u16,
    _fs_reserved: u16,
    gs: u16,
    _gs_reserved: u16,
    ldt: u16,
    _ldt_reserved: u16,
    _iopb_reserved: u16,
    iomap_base: u16,
    ssp: u32,
}

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

pub fn init() -> Result<(), ()> {
    unsafe {
        *GDT_DESCRIPTOR_PTR = GdtDescriptor {
            size: (core::mem::size_of::<[GdtEntry; GDT_SIZE]>() - 1) as u16,
            offset: addr_of!(GDT) as *const _ as u64,
        };
        asm::load_gdt(GDT_DESCRIPTOR_PTR);
    }
    if asm::check_gdt() != 0 {
        return Err(());
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