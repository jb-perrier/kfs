use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{asm, infinite_loop, kernel::kernel, mem::heap::{Heap, HeapError}, process::{process_stopped::{process_end, ProcessStopped}, Process, ProcessId, ProcessState}, signal::Signal, socket::Socket};

pub enum UserError {
    InvalidPid,
    SocketNotFound,
    ProcessNotFound,
    HeapError(HeapError),
}

fn alloc(layout: core::alloc::Layout) -> Result<*mut u8, UserError> {
    let heap = &mut kernel().get_current_process().unwrap().as_running_mut().unwrap().heap;
    heap.allocate(layout).map_err(UserError::HeapError)
}

fn dealloc(ptr: *mut u8, layout: core::alloc::Layout) -> Result<(), UserError> {
    let heap = &mut kernel().get_current_process().ok_or(UserError::ProcessNotFound)?.as_running_mut().unwrap().heap;
    heap.deallocate(ptr).map_err(UserError::HeapError)
}

pub fn send_signal(signal: Signal, pid: usize) -> Result<(), UserError> {
    let processes = &mut kernel().processes;
    let process = processes.get_mut(pid).ok_or(UserError::InvalidPid)?.as_running_mut().unwrap();
    process.signals.push(signal);
    Ok(())
}

pub fn set_signal_handler(handler: Box<dyn Fn(Signal)>) -> Result<(), UserError> {
    let processes = &mut kernel().processes;
    let current_pid = kernel().scheduler.current;
    let process = processes.get_mut(current_pid).ok_or(UserError::InvalidPid)?.as_running_mut().unwrap();
    process.signal_callback = Some(handler);
    Ok(())
}

pub fn fork() {
    asm::disable_interrupts();
    let proc = kernel().get_current_process().unwrap().as_running_mut().unwrap();
    proc.fork += 1;
    asm::enable_interrupts();

    // will change on next scheduling tick
    #[allow(clippy::while_immutable_condition)]
    while proc.fork > 0 {
        asm::nop(1);
    }
}

pub fn create_socket(name: String) -> Result<(), UserError> {
    let socket = Socket::new(name);
    kernel().sockets.push(socket);
    Ok(())
}

pub fn remove_socket(name: String) -> Result<(), UserError> {
    let mut index = None;
    for (i, socket) in kernel().sockets.iter().enumerate() {
        if socket.name == name {
            index = Some(i);
            break;
        }
    }

    if let Some(i) = index {
        kernel().sockets.remove(i);
        Ok(())
    } else {
        Err(UserError::SocketNotFound)
    }
}

pub fn socket_read(name: String) -> Result<Option<Vec<u8>>, UserError> {
    let mut index = None;
    for (i, socket) in kernel().sockets.iter().enumerate() {
        if socket.name == name {
            index = Some(i);
            break;
        }
    }

    let current_pid = kernel().get_current_process().unwrap().pid();
    if let Some(i) = index {
        let socket = &mut kernel().sockets[i];
        let data = socket.receive(current_pid);
        let data = data.map(|d| d.payload);
        Ok(data)
    } else {
        Err(UserError::SocketNotFound)
    }
}

pub fn wait(pid: ProcessId) -> Result<ProcessState, UserError> {
    let process = kernel().get_process_mut(pid).ok_or(UserError::InvalidPid)?;
    let old_state = process.state();
    let mut current_state = process.state();
    if current_state == ProcessState::Stopped {
        return Ok(current_state);
    }

    while current_state == old_state {
        current_state = process.state();
    }
    Ok(current_state)
}

pub fn exit() -> ! {
    process_end();
}

pub fn get_pid() -> ProcessId {
    kernel().get_current_process().unwrap().pid()
}

pub fn kill(pid: ProcessId) -> Result<(), UserError> {
    let index = kernel().processes.iter().position(|p| p.pid() == pid).ok_or(UserError::InvalidPid)?;
    let proc = kernel().processes.remove(index);
    let proc = proc.as_running().ok_or(UserError::ProcessNotFound)?;
    let proc = Process::Stopped(ProcessStopped::from_running(proc));
    kernel().processes.insert(index, proc);
    Ok(())
}