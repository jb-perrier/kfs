use crate::{asm, infinite_loop, kernel::kernel, process::Process, text, trace};

use super::ProcessId;

pub struct ProcessStopped {
    pub pid: ProcessId,
    pub owner: usize,
    pub parent: ProcessId,
}

impl ProcessStopped {
    pub fn from_running(running: &super::ProcessRunning) -> Self {
        ProcessStopped {
            pid: running.pid,
            owner: running.owner,
            parent: running.parent,
        }
    }
}

#[no_mangle] pub extern "C" fn process_end() -> ! {
    trace!();
    let index = kernel().get_current_process_index();
    let proc = kernel().processes.remove(index);
    text::write_format!("Process {} ended\n", proc.pid().0);
    let mut proc = match proc {
        Process::Running(p) => p,
        _ => panic!("process_start called on a non-starting process"),
    };
    // we init the process here, so he can use the virtual paging to init heap, ..
    let proc = ProcessStopped::from_running(&proc);
    kernel().processes.insert(index, Process::Stopped(proc));

    // waiting for the scheduler to switch to another process
    infinite_loop!();
}