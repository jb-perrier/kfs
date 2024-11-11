use crate::{process::Process, text};

static mut TRACING: bool = false;
static mut DEBUG_LOG: bool = false;

pub fn tracing() -> bool {
    unsafe { TRACING }
}

pub fn set_tracing(tracing: bool) {
    unsafe {
        TRACING = tracing;
    }
}

pub fn debug_log() -> bool {
    unsafe { DEBUG_LOG }
}

pub fn set_debug_log(debug_log: bool) {
    unsafe {
        DEBUG_LOG = debug_log;
    }
}

pub fn enable_debug() {
    set_debug_log(true);
    set_tracing(true);
}

pub fn disable_debug() {
    set_debug_log(false);
    set_tracing(false);
}

pub fn print_from_process(process: &Process, module: &str) {
    let process_id = process.pid();
    if (process_id.0 == 0) {
        text::write_str("kernel");
    } else {
        text::write_num!(process_id.0);
    }
    text::write_str(".");
    text::write_str(module);
    text::write_str(": ");
}

#[macro_export]
macro_rules! file_line {
    () => {
        concat!(file!(), ":", line!())
    };
}
pub use file_line;

// macro_rules! print_debug {
//     ($msg:expr) => {
//         {
//             let debug = $crate::debug::debug_log();
//             if (kernel.debug_log) {
//                 text::write_str($msg);
//                 let tracing = $crate::debug::tracing();
//                 if (tracing) {
//                     text::write_str_with_colors("\n\t-> ", &text::Colors::DarkGray, &text::Colors::Black);
//                     text::write_str_with_colors(file_line!(), &text::Colors::DarkGray, &text::Colors::Black);
//                 }
//                 text::write_str("\n");
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! debug {
    ($($exprs:expr);+ $(;)?) => {
        {
            let __debug__ = $crate::debug::debug_log();
            if (__debug__) {
                {
                    let tracing = $crate::debug::tracing();
                    if (tracing) {
                        for _ in 0..unsafe { $crate::debug::TRACING_DEPTH } {
                            $crate::text::write_str("  ");
                        }
                        $crate::text::write_str(" ");
                    }
                }
                $($exprs);+;
            }
        }
    };
}
pub use debug;

pub static mut TRACING_DEPTH: usize = 0;

pub struct TracingDecrementer;

impl Drop for TracingDecrementer {
    fn drop(&mut self) {
        unsafe {
            TRACING_DEPTH = TRACING_DEPTH.saturating_sub(1);
        }
    }
}

#[macro_export]
macro_rules! trace {
    () => {
        let __tracing_decrementer__ = $crate::debug::TracingDecrementer;
        {
            let tracing = $crate::debug::tracing();
            if (tracing) {
                for _ in 0..unsafe { $crate::debug::TRACING_DEPTH } {
                    $crate::text::write_str("  ");
                }
                $crate::text::write_str_with_colors(
                    "-> ",
                    $crate::text::Colors::DarkGray,
                    $crate::text::Colors::Black,
                );
                $crate::text::write_str_with_colors(
                    $crate::debug::file_line!(),
                    $crate::text::Colors::DarkGray,
                    $crate::text::Colors::Black,
                );
                $crate::text::write_str("\n");
                unsafe {
                    $crate::debug::TRACING_DEPTH += 1;
                }
            }
        }
    };
}
pub use trace;
