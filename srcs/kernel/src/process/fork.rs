use alloc::vec::Vec;

use crate::{mem::{frame::FRAME_SIZE, heap::Heap}, process::{PROCESS_USER_STACK_FRAME_SIZE, PROCESS_USER_VIRTUAL_START}, text};

use super::{find_free_pid, process_running::ProcessRunning, process_start::ProcessStart, Process, StackAddr, PROCESS_USER_HEAP_SIZE, PROCESS_USER_STACK_SIZE};

fn dummy_func() {
}

pub fn fork(original_proc: &ProcessRunning) -> ProcessRunning {
    let mut proc_start = ProcessStart::user(0, dummy_func);

    text::write_format!("Forking process {} to {}\n", original_proc.pid.0, proc_start.pid.0);
    unsafe {
        // copy stack
        core::ptr::copy_nonoverlapping(*original_proc.stack.bottom_phys, *proc_start.stack.bottom_phys, PROCESS_USER_STACK_SIZE);
        
        //copy heap
        core::ptr::copy_nonoverlapping(*original_proc.heap_bottom, *proc_start.heap_bottom, PROCESS_USER_HEAP_SIZE);
    }

    proc_start.stack.ptr = original_proc.stack.ptr;

    ProcessRunning {
        page_directory: proc_start.page_directory,
        heap: original_proc.heap,
        heap_bottom: proc_start.heap_bottom,
        signals: Vec::new(),
        signal_callback: None,
        stack: proc_start.stack,
        pid: proc_start.pid,
        owner: proc_start.owner,
        parent: proc_start.pid,
        children: Vec::new(),
        func: proc_start.func,
        fork: 0,
        exit: false,
    }
}