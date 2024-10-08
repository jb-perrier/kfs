use core::{alloc::Layout, ptr::addr_of};
use frame::{FrameAllocator, FRAME_SIZE};
use heap::Heap;
use multiboot::information::{MemoryType, Multiboot};
use paging::PAGE_SIZE;

use crate::{
    asm, error::KernelError, infinite_loop, kernel::{self, Kernel}, vga
};

pub mod frame;
pub mod heap;
pub mod paging;
pub mod virtual_addr;

pub fn init(kernel: &mut Kernel) -> Result<(), KernelError> {
    if !kernel.multiboot.as_ref().unwrap().has_memory_map() {
        return Err(KernelError::NoMemoryMap);
    }
    
    let Some(mut region) = choose_region(kernel.multiboot.as_ref().unwrap()) else {
        return Err(KernelError::NoSuitableMemoryRegionFound);
    };

    let kernel_end = asm::kernel_end();
    // let some space between kernel and page tables
    // for some reasons it crashes if we put the tables right after the kernel
    // something use memory just after the kernel memory but i don't know why
    region.0 = next_aligned_from_addr(kernel_end + 16 * FRAME_SIZE, FRAME_SIZE);
    region.1 = previous_aligned_from_addr(region.1, FRAME_SIZE);

    kernel.frame_allocator = Some(FrameAllocator::new(region));
    let mut frame_allocator = kernel.frame_allocator.as_mut().unwrap();
    //print_regions(boot_info);

    vga::write_str("Frames allocation block: 0x");
    vga::write_num_hex!(region.0);
    vga::write_str(" - 0x");
    vga::write_num_hex!(region.1);
    vga::write_str("\n");

    let pages_count = (region.1 - region.0) / PAGE_SIZE;
    vga::write_str("Memory available: ");
    vga::write_num!(pages_count);
    vga::write_str(" pages / ");
    vga::write_num!(pages_count * PAGE_SIZE);
    vga::write_str(" bytes\n");

    if paging::init(kernel).is_err() {
        return Err(KernelError::Unknown);
    }

    let heap = Heap::new(region.0, region.1 - region.0);
    // kernel.process.heap.insert(heap);
    Ok(())
}

fn choose_region(boot_info: &Multiboot) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize)> = None;
    for (i, mem) in boot_info.memory_regions().unwrap().enumerate() {
        if mem.memory_type() == MemoryType::Available {
            if best.is_none() {
                best = Some((
                    mem.base_address() as usize,
                    mem.base_address() as usize + mem.length() as usize,
                ));
            } else {
                let (best_start, best_end) = best.unwrap();
                let start = mem.base_address() as usize;
                let end = mem.base_address() as usize + mem.length() as usize;
                if end - start > best_end - best_start {
                    best = Some((start, end));
                }
            }
        }
    }

    best
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
pub fn next_aligned_from_addr(addr: usize, align: usize) -> usize {
    if addr % align == 0 {
        addr
    } else {
        (addr / align + 1) * align
    }
}

// get previous aligned address starting from addr
pub fn previous_aligned_from_addr(addr: usize, align: usize) -> usize {
    if addr % align == 0 {
        addr
    } else {
        (addr / align) * align
    }
}

fn build_virtual(
    physical: usize,
    directory_index: usize,
    table_index: usize,
    offset: usize,
) -> usize {
    (directory_index << 22) | (table_index << 12) | offset
}
