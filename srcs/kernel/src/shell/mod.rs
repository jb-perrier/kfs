use crate::{asm, text};

static mut COMMAND_BUFFER: [u8; 128] = [0; 128];
static mut COMMAND_BUFFER_INDEX: usize = 0;
static mut COMMAND_BUFFER_LENGTH: usize = 0;

pub fn print_shell() {
    text::write_str(">");
    let index = text::get_index();
    text::set_cursor_pos(index);
}

pub fn execute() {
    unsafe {
        text::write('\n');

        let command_slice = &COMMAND_BUFFER[..COMMAND_BUFFER_LENGTH];
        let command_str = core::str::from_utf8_unchecked(command_slice);
        let mut command_iter = command_str.split_whitespace();
        let first_command = command_iter.next();

        match first_command {
            Some("clear") => {
                text::clear();
            }
            Some("exit") => {
                asm::out_u16(0x604, 0x2000);
            }
            Some("quoi") => {
                text::write_str("QUOICOUBEH\n");
            }
            Some("help") => {
                text::write_str("Commands:\n");
                text::write_str("clear: Clear the screen\n");
                text::write_str("help: Print this help\n");
            }
            _ => {
                text::write_str("Command not found: ");
                text::write_str(command_str);
                text::write_str("\n");
            }
        }

        reset();

        print_shell();
    }
}

pub fn add_char(c: char) {
    unsafe {
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
