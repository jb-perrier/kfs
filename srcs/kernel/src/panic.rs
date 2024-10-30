use core::panic::PanicInfo;

use crate::{infinite_loop, text, Colors};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    text::write_str_with_colors("Kernel panic !\n", Colors::Red, Colors::Black);

    if let Some(msg) = _info.message().as_str() {
        text::write_str("  Message: ");
        text::write_str(msg);
        text::write_str("\n");
    }

    if let Some(location) = _info.location() {
        text::write_str("  File: ");
        text::write_str(location.file());
        text::write_str(":");
        text::write_num!(location.line());
        text::write_str(":");
        text::write_num!(location.column());
        text::write_str("\n");
    }

    infinite_loop!()
}