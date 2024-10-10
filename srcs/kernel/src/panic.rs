use crate::{infinite_loop, text, Colors};

pub fn panic(msg: &str) -> ! {
    text::clear();
    text::write_str_with_colors("PANIC: ", &Colors::Black, &Colors::Red);
    text::write_str_with_colors(msg, &Colors::Black, &Colors::Red);
    infinite_loop!()
}