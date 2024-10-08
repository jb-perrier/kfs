use crate::{infinite_loop, vga, Colors};

pub fn panic(msg: &str) -> ! {
    vga::clear();
    vga::write_str_with_colors("PANIC: ", &Colors::Black, &Colors::Red);
    vga::write_str_with_colors(msg, &Colors::Black, &Colors::Red);
    infinite_loop!()
}