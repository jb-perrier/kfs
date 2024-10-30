use alloc::boxed::Box;

use crate::{kernel::kernel, signal::Signal};

pub enum ApiError {
    InvalidPid,
}

// signals
pub fn send_signal(signal: Signal, pid: usize) -> Result<(), ApiError> {
    let processes = &mut kernel().processes;
    let process = processes.get_mut(pid).ok_or(ApiError::InvalidPid)?;
    process.push_signal(signal);
    Ok(())
}

pub fn set_signal_handler(handler: Box<dyn Fn(Signal)>) -> Result<(), ApiError> {
    let processes = &mut kernel().processes;
    let current_pid = kernel().scheduler.current;
    let process = processes.get_mut(current_pid).ok_or(ApiError::InvalidPid)?;
    process.signal_callback = Some(handler);
    Ok(())
}

// fork
pub fn fork() -> Result<usize, ApiError> {
    unimplemented!()
}