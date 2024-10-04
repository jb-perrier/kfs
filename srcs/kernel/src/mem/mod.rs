use core::{alloc::Layout, ptr::addr_of};

use multiboot::information::{MemoryType, Multiboot};
use paging::PAGE_SIZE;

use crate::{asm, infinite_loop, vga};

pub mod paging;

// pub unsafe fn kalloc(layout: Layout) -> *mut u8 {
    
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

    let Some(mut region) = choose_region(boot_info) else {
        return Err(MemoryError::NoSuitableMemoryRegionFound);
    };

    region.0 = next_aligned_from_addr(region.0, PAGE_SIZE);

    vga::write_str("Kernel start: ");
    unsafe {
        let addr = addr_of!(asm::_KERNEL_START) as u32;
        vga::write_num_hex!(addr);
    };
    vga::write_str("\n");

    vga::write_str("Kernel end: ");
    unsafe {
        let addr = addr_of!(asm::_KERNEL_END) as u32;
        vga::write_num_hex!(addr);
    };
    vga::write_str("\n");

    let kernel_end = unsafe { addr_of!(asm::_KERNEL_END) as u32 };
    region.0 = next_aligned_from_addr(kernel_end * 4, PAGE_SIZE);

    // print_regions(boot_info);
    
    vga::write_str("Region for pages allocation: ");
    vga::write_num_hex!(region.0);
    vga::write_str(" - ");
    vga::write_num_hex!(region.1);
    vga::write_str("\n");

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

// get next aligned address starting from addr
pub fn next_aligned_from_addr(addr: u32, align: u32) -> u32 {
    if addr % align == 0 {
        addr
    } else {
        (addr / align + 1) * align
    }
}

// get previous aligned address starting from addr
pub fn previous_aligned_from_addr(addr: u32, align: u32) -> u32 {
    if addr % align == 0 {
        addr
    } else {
        (addr / align) * align
    }
}

fn build_virtual(physical: u32, directory_index: u32, table_index: u32, offset: u32) -> u32 {
    (directory_index << 22) | (table_index << 12) | offset
}