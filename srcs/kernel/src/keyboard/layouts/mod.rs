mod qwerty;
pub use qwerty::*;

mod azerty;
pub use azerty::*;

use crate::text;

#[derive(Clone, Copy)]
pub enum Key {
	None,
	Backspace,
	Enter,
	Shift,
	Ctrl,
	CapsLock,
	LeftArrow,
	RightArrow,
	UpArrow,
	DownArrow,
	Char(char),
}

pub fn get_char(layout: &[Key; 128], scancode: u8) -> Key {
    // text::write_num!(scancode);
    // text::write_str("\n");
	*layout.get(scancode as usize).unwrap_or(&Key::None)
    // Key::Char(' ')
}