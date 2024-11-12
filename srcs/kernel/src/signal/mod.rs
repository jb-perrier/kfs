use alloc::string::String;

use crate::{kernel::kernel, text};

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Echo(String),
    Exit,
}

impl Signal {
    pub fn name(&self) -> &str {
        match self {
            Signal::Echo(_) => "Echo",
            Signal::Exit => "Exit",
        }
    }
}

#[no_mangle] pub extern "C" fn execute_signal_from_asm() {
    let Some(proc) = kernel().get_current_process() else {
        return;
    };

    let Some(proc) = proc.as_running_mut() else {
        return;
    };

    proc.execute_signals();
}