use core::{alloc::Layout, ptr::addr_of};
use frame::{FrameAllocator, FRAME_SIZE};
use heap::Heap;
use multiboot::information::{MemoryType, Multiboot};
use paging::{directory::PageDirectory, PAGE_SIZE};

use crate::{
    asm,
    debug::print_from_process,
    error::KernelError,
    infinite_loop,
    kernel::{self, Kernel},
    process::address::VirtAddr,
    text, Colors,
};

pub mod frame;
pub mod heap;
pub mod paging;

pub fn init(
    multiboot: &Multiboot,
) -> Result<(FrameAllocator, *mut PageDirectory, Heap), KernelError> {
    if !multiboot.has_memory_map() {
        return Err(KernelError::NoMemoryMap);
    }

    let Some(mut region) = choose_region(multiboot) else {
        return Err(KernelError::NoSuitableMemoryRegionFound);
    };

    let kernel_end = asm::kernel_end();
    let old_start = region.0;
    region.0 = next_aligned_from_addr(kernel_end + FRAME_SIZE, FRAME_SIZE);
    region.1 = previous_aligned_from_addr(region.1, FRAME_SIZE);

    let mut frame_allocator = FrameAllocator::new(region);
    // print_regions(multiboot);

    let Ok(page_directory_addr) = paging::init(&mut frame_allocator, multiboot) else {
        return Err(KernelError::Unknown);
    };

    const FRAME_COUNT: usize = 16;
    let Ok(block) = frame_allocator.allocate_many(FRAME_COUNT) else {
        text::write_str_with_colors("Failed to allocate blocks !", Colors::Red, Colors::Black);
        infinite_loop!();
    };

    let heap = Heap::new_from_range(block.addr(), FRAME_COUNT * FRAME_SIZE);
    Ok((frame_allocator, page_directory_addr, heap))
}

fn find_max_addr(boot_info: &Multiboot) -> usize {
    let mut max_addr = 0;
    for mem in boot_info.memory_regions().unwrap() {
        let end = mem.base_address() + mem.length();
        if end > max_addr {
            max_addr = end;
        }
    }
    if max_addr > usize::MAX as u64 {
        usize::MAX
    } else {
        max_addr as usize
    }
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
        text::write_str(typ);
        text::write_str(":");
        for _ in 0..(12 - typ.len()) {
            text::write(' ');
        }
        text::write_str(" - 0x");
        text::write_num_hex!(base);
        text::write_str(" - 0x");
        text::write_num_hex!(end);
        text::write_str(" (");
        text::write_num!(length);
        text::write_str(" bytes)\n");
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

fn build_virtual(directory_index: usize, table_index: usize, offset: usize) -> usize {
    (directory_index << 22) | (table_index << 12) | offset
}

pub fn decompose_virtual(vaddr: VirtAddr) -> (usize, usize, usize) {
    let vaddr = vaddr.addr();
    let dir = (vaddr >> 22) & 0x3FF;
    let table = (vaddr >> 12) & 0x3FF;
    let offset = vaddr & 0xFFF;
    (dir, table, offset)
}
