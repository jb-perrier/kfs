use core::panic;

use crate::{
    asm::{
        self, get_proc_esp, get_proc_page_dir, set_proc_esp, set_proc_page_dir, HandlerRegisters,
    },
    infinite_loop,
    kernel::{kernel, kernel_option},
    process::process_stopped::ProcessStopped,
    text, write_format,
};

use super::{address::VirtAddr, fork::fork, process_start::ProcessStart, Process, ProcessState};

pub struct Scheduler {
    pub current: usize,
    pub running: bool,
    pub start: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            current: 0,
            running: false,
            start: true,
        }
    }

    pub fn next(&mut self, processes: &[Process]) -> usize {
        let start = self.current;
        let mut looped = false;
        loop {
            self.current += 1;
            if self.current >= processes.len() {
                // text::write_format!("self.current = 0\n");
                self.current = 0;
            }
            if self.current == start {
                if looped {
                    text::write_format!("No running process found, stopping !\n");
                    infinite_loop!();
                }
                looped = true;
                // text::write_format!("Looped\n");
            }
            if !processes[self.current].is_stopped() {
                break;
            }
        }

        self.current
    }

    pub fn switch_to_process(&mut self, current: &mut Process, next: &Process) {}

    pub fn run(&mut self) -> ! {
        self.running = true;
        // waiting for next task switch
        infinite_loop!()
    }
}

pub fn tick_scheduler(regs: HandlerRegisters) {
    if kernel_option().is_none() {
        return;
    }

    if !kernel().scheduler.running {
        return;
    }

    // special handling when the previous process is the kernel one
    // after the first "user" process is started the kernel process will never be the current one
    // and shouldn't be used anymore
    //TODO: start scheduler directly from kernel main
    if kernel().scheduler.start {
        kernel().scheduler.start = false;
        let proc = kernel().processes.get_mut(0).unwrap();
        if let Process::Start(proc) = proc {
            start_process(proc);
        }
        panic!("Kernel process should not be the current one");
    }

    let Some(current_process) = kernel().get_current_process() else {
        panic!("No current process");
        return;
    };

    if let Some(current_process) = current_process.as_running_mut() {
        // save the stack ptr
        current_process.stack.ptr = VirtAddr::from_usize(get_proc_esp() as usize);

        // fork if needed
        while current_process.fork > 0 {
            let new_proc = fork(current_process);
            kernel().processes.push(Process::Running(new_proc));
            current_process.fork -= 1;
        }
    };

    let next = kernel().scheduler.next(&kernel().processes);
    let next_process = kernel().processes.get_mut(next).unwrap();

    // text::write_str("Switching to proc ");
    // text::write_num!(next_process.pid().0);
    // text::write_str("\n");

    if let Process::Start(next_process) = next_process {
        start_process(next_process);
    }

    // if next_process.pid == current_process.pid {
    //     return 0;
    // }

    if let Process::Running(next_process) = next_process {
        let old_esp = get_proc_esp();
        let old_page = get_proc_page_dir();

        set_proc_esp(next_process.stack.ptr.addr() as u32);
        set_proc_page_dir(next_process.page_directory as u32);
    }
}

fn start_process(proc: &mut ProcessStart) -> ! {
    text::write_format!("Starting process: {}\n", proc.pid.0);
    let eip = proc.func as *const () as u32;
    let eflags = 0x0; // disable interrupts
    let ebp = proc.stack.top.addr() as u32;
    let esp = proc.stack.top.addr() as u32;
    let cr3 = proc.page_directory as u32;

    // write_format!("eip: {:x}\n", eip);
    // write_format!("eflags: {:x}\n", eflags);
    // write_format!("ebp: {:x}\n", ebp);
    // write_format!("esp: {:x}\n", esp);
    // write_format!("cr3: {:x}\n", cr3);

    // let dir = unsafe { &(*proc.page_directory) };
    // let esp_phys = proc.stack.top.physical(dir);
    // write_format!("esp physical: {:x}\n", esp_phys.addr());
    // write_format!("cr3 physical: {:x}\n", cr3);
    // let eip_phys = VirtAddr::from_usize(eip as usize).physical(dir);
    // write_format!("eip physical: {:x}\n", eip_phys.addr());
    // infinite_loop!();
    // jump into the new process
    // if the previous task was the kernel one the kernel stack will be in bad state and cannot be reused !
    asm::jump_in_new_process(esp, ebp, eip, eflags, cr3);
    infinite_loop!()
}
