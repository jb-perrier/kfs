use alloc::vec::Vec;

use crate::{asm, debug::{disable_debug, enable_debug}, kernel::kernel, mem::{frame::FRAME_SIZE, heap::Heap, paging::directory::PageDirectory}, process::{find_free_pid, PROCESS_USER_STACK_FRAME_SIZE}, text, trace};

use super::{address::{PhysAddr, VirtAddr}, process_running::ProcessRunning, Process, ProcessId, StackAddr, PROCESS_USER_HEAP_FRAME_SIZE, PROCESS_USER_HEAP_SIZE, PROCESS_USER_STACK_SIZE, PROCESS_USER_VIRTUAL_START};

pub struct ProcessStart {
    pub page_directory: *mut PageDirectory,
    pub heap: VirtAddr,
    pub heap_bottom: PhysAddr,
    pub stack: StackAddr,
    pub pid: ProcessId,
    pub owner: usize,
    pub parent: ProcessId,
    pub func: fn() -> (),
}

impl ProcessStart {
    pub fn user(owner: usize, func: fn() -> ()) -> Self {
        trace!();
        let pid = find_free_pid();
        let frame_alloc = &mut kernel().frame_allocator;
        let page_directory = PageDirectory::new_from_frame_allocator(frame_alloc, true).unwrap();
        // text::write_format!("Page directory: {:p}\n", page_directory);
        let mut dir = unsafe { &mut (*page_directory) };
        // dir.identity();

        let pstack = kernel()
            .frame_allocator
            .allocate_many(PROCESS_USER_STACK_FRAME_SIZE)
            .unwrap();
        let pheap = kernel()
            .frame_allocator
            .allocate_many(PROCESS_USER_HEAP_FRAME_SIZE)
            .unwrap();

        // text::write_format!("Physical stack: {:x}\n", pstack.addr());
        // text::write_format!("Physical Heap: {:x}\n", pheap.addr());
        trace!();

        let (stack, heap, heap_phys) = unsafe {

            // map stack in virtual memory
            let vstack = PROCESS_USER_VIRTUAL_START;
            for i in 0..PROCESS_USER_STACK_FRAME_SIZE {
                let paddr = pstack.add(i * FRAME_SIZE).into();
                let vaddr = vstack.add(i * FRAME_SIZE).into();
                dir.set_page(paddr, vaddr);
            }            
            // map heap in virtual memory
            let vheap = vstack.add(PROCESS_USER_STACK_SIZE);
            for i in 0..PROCESS_USER_HEAP_FRAME_SIZE {
                let paddr = pheap.add(i * FRAME_SIZE).into();
                let vaddr = vheap.add(i * FRAME_SIZE).into();
                dir.set_page(paddr, vaddr);
            }
            // text::write_format!("Phys {:x} {:x} mapped to Virt {:x} {:x}\n", pheap.addr(), pheap.add(PROCESS_USER_HEAP_SIZE).addr() - 1, vheap.addr(), vheap.add(PROCESS_USER_HEAP_SIZE).addr() - 1);
            // text::write_format!("Phys {:x} {:x} mapped to Virt {:x} {:x}\n", pstack.addr(), pstack.add(PROCESS_USER_STACK_SIZE).addr() - 1, vstack.addr(), vstack.add(PROCESS_USER_STACK_SIZE).addr() - 1);

            let top = vstack.add(PROCESS_USER_STACK_SIZE);
            let stack = StackAddr {
                bottom: vstack,
                top: top.into(),
                ptr: top.into(),
                bottom_phys: pstack,
            };

            (stack, vheap.into(), pheap)
        };

        trace!();

        Self {
            page_directory,
            heap,
            heap_bottom: heap_phys,
            stack,
            pid,
            owner,
            parent: ProcessId(0),
            func,
        }
    }

    pub fn start(self) -> ProcessRunning {
        let start = self.heap.addr();
        let size = PROCESS_USER_HEAP_SIZE;
        let heap = Heap::new_from_range(start, size);
        // let heap = Heap::empty();

        ProcessRunning {
            page_directory: self.page_directory,
            heap,
            heap_bottom: self.heap_bottom,
            signals: Vec::new(),
            signal_callback: None,
            stack: self.stack,
            pid: self.pid,
            owner: self.owner,
            parent: self.parent,
            children: Vec::new(),
            func: self.func,
            fork: 0,
            exit: false,
        }
    }
}

// always clear interrupt flag before calling this function
#[no_mangle] extern "C" fn process_start() {
    trace!();
    let index = kernel().get_current_process_index();
    let proc = kernel().processes.remove(index);
    let mut proc = match proc {
        Process::Start(p) => p,
        _ => panic!("process_start called on a non-starting process"),
    };
    // we init the process here, so he can use the virtual paging to init heap, ..
    let proc = proc.start();
    kernel().processes.insert(index, Process::Running(proc));
}