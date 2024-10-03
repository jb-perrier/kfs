use core::alloc::Layout;

use multiboot::information::{MemoryType, Multiboot};

use crate::{infinite_loop, vga};

pub mod paging;

// pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    
// }

// pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    
// }

// pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    
// }

// pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    
// }

pub fn init(boot_info: &Multiboot) -> Result<(), MemoryError> {
    if !boot_info.has_memory_map() {
        return Err(MemoryError::NoMemoryMap);
    }

    let Some(region) = choose_region(boot_info) else {
        return Err(MemoryError::NoSuitableMemoryRegionFound);
    };

    if paging::init(region).is_err() {
        return Err(MemoryError::Unknown);
    }
    
    Ok(())
}

fn choose_region(boot_info: &Multiboot) -> Option<(u32, u32)> {
    let mut best = None;
    for (i, mem) in boot_info.memory_regions().unwrap().enumerate() {
        if mem.memory_type() == MemoryType::Available {
            if best.is_none() {
                best = Some((mem.base_address() as u32, mem.base_address() as u32 + mem.length() as u32));
            } else {
                let (best_start, best_end) = best.unwrap();
                let start = mem.base_address() as u32;
                let end = mem.base_address() as u32 + mem.length() as u32;
                if end - start > best_end - best_start {
                    best = Some((start, end));
                }
            }
        }
    }

    best
}

pub enum MemoryError {
    NoMemoryMap,
    NoSuitableMemoryRegionFound,
    Unknown,
}

pub fn print_regions(boot_info: &Multiboot) {
    for mem in boot_info.memory_regions().unwrap() {
        let base = mem.base_address();
        let length = mem.length();
        let end = base + length;
        let typ = match mem.memory_type() {
            MemoryType::Available => "Available",
            MemoryType::ACPI => "ACPI",
            MemoryType::NVS => "NVS",
            MemoryType::Defect => "Defect",
            MemoryType::Reserved => "Reserved",
            _ => "Unknown",
        };
        vga::write_str(typ);
        vga::write_str(":");
        for _ in 0..(12 - typ.len()) {
            vga::write(' ');
        }
        vga::write_num!(base);
        vga::write_str(" - ");
        vga::write_num!(end);
        vga::write_str(" (");
        vga::write_num!(length);
        vga::write_str(" bytes)\n");
    }
}