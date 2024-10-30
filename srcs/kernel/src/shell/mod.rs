use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    asm,
    dump::{dump, save_kernel_stack},
    kernel,
    keyboard::{set_azerty, set_qwerty},
    signal::Signal,
    text,
};

pub mod style;

const LINE_MAX_LENGTH: usize = 128;
static mut COMMAND_BUFFER: [u8; LINE_MAX_LENGTH] = [0; LINE_MAX_LENGTH];
static mut COMMAND_BUFFER_INDEX: usize = 0;
static mut COMMAND_BUFFER_LENGTH: usize = 0;

pub fn print_shell() {
    text::write_str(">");
    let index = text::get_index();
    text::set_cursor_pos(index);
}

pub enum ShellError {
    InvalidCommand,
}

pub fn execute() -> Result<(), ShellError> {
    unsafe {
        text::write('\n');

        let command_slice = &COMMAND_BUFFER[..COMMAND_BUFFER_LENGTH];
        let command_str = core::str::from_utf8_unchecked(command_slice);
        let mut command_iter = command_str.split_whitespace();
        let cmd = command_iter.next();
        let args = command_iter.collect::<Vec<&str>>();

        match cmd {
            Some("clear") => {
                text::clear();
            }
            Some("exit") => {
                asm::out_u16(0x604, 0x2000);
            }
            Some("quoi") => {
                text::write_str("QUOICOUBEH\n");
            }
            Some("signal") => {
                if let Some(sub_cmd) = args.first().copied() {
                    if sub_cmd == "echo" {
                        let msg = args.get(1).copied().ok_or(ShellError::InvalidCommand)?;
                        text::write_str("Debug\n");
                        kernel().process.push_signal(Signal::Echo(msg.to_string()));
                        text::write_str("Debug2\n");

                    }
                    kernel().process.execute_signals();
                }
            }
            Some("keyboard") => {
                let layout = args.first().copied().ok_or(ShellError::InvalidCommand)?;
                if layout == "azerty" {
                    set_azerty();
                } else if layout == "qwerty" {
                    set_qwerty();
                }
            }
            Some("stack_save") => {
                let heap = &mut kernel().process.heap;
                let stack = save_kernel_stack(heap).unwrap();
                text::write_format!("Saved stack {:p} {}\n", stack.0, stack.1);
                let stack_bottom = stack.0;
                let stack_top = stack.0.add(stack.1);
                let stack_ptr =
                    stack_top.sub(asm::get_stack_top().addr() - asm::get_stack_ptr().addr());
                let mut limit = stack_ptr.add(8 * 20);
                if limit >= stack_top {
                    limit = stack_top;
                }
                dump(stack_ptr, limit);
                heap.deallocate(stack.0);
            }
            Some("stack_ptr") => {
                text::write_format!("{:p}\n", asm::get_stack_ptr());
            }
            Some("stack_dump") => {
                let stack_bottom = asm::get_stack_bottom();
                let stack_top = asm::get_stack_top();
                let stack_ptr = asm::get_stack_ptr();
                let mut limit = stack_ptr.add(8 * 20);
                if limit >= stack_top {
                    limit = stack_top;
                }
                dump(stack_ptr, limit);
            }
            Some("help") => {
                text::write_str("Commands:\n");
                text::write_str("clear: Clear the screen\n");
                text::write_str("exit: Shutdown the system\n");
                text::write_str("help: Print this help\n");
            }
            Some(cmd) => command_not_found(cmd),
            None => command_not_found(""),
        }

        reset();

        print_shell();
        Ok(())
    }
}

fn command_not_found(cmd: &str) {
    text::write_str("Command not found: ");
    text::write_str(cmd);
    text::write_str("\n");
}

pub fn add_char(c: char) {
    unsafe {
        if COMMAND_BUFFER_INDEX >= LINE_MAX_LENGTH {
            return;
        }
        if COMMAND_BUFFER_INDEX != COMMAND_BUFFER_LENGTH {
            for i in (COMMAND_BUFFER_INDEX..COMMAND_BUFFER_LENGTH).rev() {
                COMMAND_BUFFER[i + 1] = COMMAND_BUFFER[i];
            }
        }
        COMMAND_BUFFER[COMMAND_BUFFER_INDEX] = c as u8;
        COMMAND_BUFFER_INDEX += 1;
        COMMAND_BUFFER_LENGTH += 1;

        text::write(c);
        text::set_cursor_pos(text::get_cursor_index());
    }
}

pub fn remove_char() {
    unsafe {
        if COMMAND_BUFFER_INDEX > 0 {
            for i in COMMAND_BUFFER_INDEX..COMMAND_BUFFER_LENGTH {
                COMMAND_BUFFER[i - 1] = COMMAND_BUFFER[i];
            }
            COMMAND_BUFFER_LENGTH -= 1;
            COMMAND_BUFFER_INDEX -= 1;
            COMMAND_BUFFER[COMMAND_BUFFER_LENGTH] = 0;

            text::erase();
            let index = text::get_cursor_index();
            text::set_cursor_pos(index);
        }
    }
}

pub fn reset() {
    unsafe {
        COMMAND_BUFFER_INDEX = 0;
        COMMAND_BUFFER_LENGTH = 0;
        for i in 0..COMMAND_BUFFER.len() {
            COMMAND_BUFFER[i] = 0;
        }

        let index = text::get_index();
        text::set_cursor_index(index);
        text::set_cursor_pos(index);
    }
}

pub fn move_left() {
    unsafe {
        if COMMAND_BUFFER_INDEX > 0 {
            COMMAND_BUFFER_INDEX -= 1;

            let index = text::get_cursor_index();
            text::set_cursor_index(index - 1);
            text::set_cursor_pos(index - 1);
        }
    }
}

pub fn move_right() {
    unsafe {
        if COMMAND_BUFFER_INDEX < COMMAND_BUFFER_LENGTH {
            COMMAND_BUFFER_INDEX += 1;

            let index = text::get_cursor_index();
            text::set_cursor_index(index + 1);
            text::set_cursor_pos(index + 1);
        }
    }
}
