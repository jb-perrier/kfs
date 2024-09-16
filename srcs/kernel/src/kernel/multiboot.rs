#[repr(C)]
pub struct Multiboot {
    pub flags: u32,
    // MEM
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: Symbol,
    // MMAP
    pub mmap_length: u32,
    pub mmap_addr: u32,
    // DRIVES
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    // VBE
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    // FRAME
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ElfSection {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AOutSymbol {
    pub tabsize: u32,
    pub strsize: u32,
    pub addr: u32,
    pub reserved: u32,
}

#[repr(C)]
pub union Symbol {
    pub aout_sym: AOutSymbol,
    pub elf_sec: ElfSection,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MmapEntry {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub ty: u32,
}

pub const MEMORY_AVAILABLE: u32 = 1;
pub const MEMORY_RESERVED: u32 = 2;
pub const MEMORY_ACPI_RECLAIMABLE: u32 = 3;
pub const MEMORY_NVS: u32 = 4;
pub const MEMORY_BADRAM: u32 = 5;
