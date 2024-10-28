use crate::{asm, mem::heap::{self, Heap}};

use super::text;

static mut STR_BUF: [u8; 4] = [0; 4];

pub unsafe fn dump(mut base: *const u8, limit: *const u8) {
    let mut ptr = base;
    let mut line_ptr = base;
    while ptr.addr() < limit.addr() {
        if (ptr.addr() - base.addr()) % 8 == 0 {
            print_addr_header(ptr);
        } else if (ptr.addr() - base.addr()) % 2 == 0 {
            text::write_str(" ");
        }

        if ptr.addr() < limit.addr() {
            print_as_hex(*ptr as usize, 2);
        } else {
            text::write_str("  ");
        }

        ptr = ptr.add(1);
        if (ptr.addr() - base.addr()) % 8 == 0 {
            let save_ptr = ptr;
            ptr = line_ptr;
            text::write_str("  ");
            while ptr.addr() < save_ptr.addr() && ptr.addr() < limit.addr() {
                let c = *ptr as char;
                if c.is_ascii_graphic() {
                    text::write_str(core::str::from_utf8(&[*ptr]).unwrap());
                } else {
                    text::write_str(".");
                }
                ptr = ptr.add(1);
            }
            text::write_str("\n");
            line_ptr = ptr;
        }
    }
    text::write_str("\n");
}

unsafe fn print_addr_header(ptr: *const u8) {
    print_as_hex(ptr.addr(), 8);
    text::write_str("  ");
}

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
static mut HEX_BUF: [u8; 8] = [0; 8];

pub unsafe fn print_as_hex(mut value: usize, max_width: usize) {
    for i in (0..8).rev() {
        if value == 0 {
            HEX_BUF[i] = b'0';
        } else {
            let remainder = value % 16;
            HEX_BUF[i] = HEX_CHARS[remainder];
            value /= 16;
        }
    }

    for j in (8 - max_width)..8 {
        text::write_str(core::str::from_utf8(&[HEX_BUF[j]]).unwrap());
    }
}

pub fn save_kernel_stack(heap: &mut Heap) -> Result<*mut u8, heap::Error> {
    unsafe {
        let stack_top = asm::get_stack_top();
        let stack_bottom = asm::get_stack_bottom();
        let stack_ptr = asm::get_stack_ptr();
        let stack_size = stack_bottom.addr() - stack_top.addr();

        text::write_format!("Saving kernel stack !\n");
        text::write_format!("Stack size: 0x{:x}\n", stack_size);
        let layout = core::alloc::Layout::from_size_align(stack_size as usize, 16).unwrap();
        let saved_stack = heap.allocate(layout)?;
        core::ptr::copy(stack_bottom as *const u8, saved_stack, stack_size as usize);
        Ok(saved_stack)
    }
}