use super::KERN;

static mut STR_BUF: [u8; 4] = [0; 4];

pub unsafe fn dump(mut base: *const u8, limit: *const u8) {
    // KERN.write_str("dump: ");
    // KERN.write_usize(base as usize);
    // KERN.write_str(" => ");
    // KERN.write_usize(limit as usize);
    // KERN.write_str("\n");

    let mut ptr = base;
    let mut line_ptr = base;
    while ptr.addr() < limit.addr() || (ptr.addr() - base.addr()) % 8 != 0 {
        if (ptr.addr() - base.addr()) % 8 == 0 {
            print_addr_header(ptr);
        } else if (ptr.addr() - base.addr()) % 2 == 0 {
            KERN.write_str(" ");
        }

        if ptr.addr() < limit.addr() {
            print_as_hex(*ptr as usize, 2);
        } else {
            KERN.write_str("  ");
        }

        ptr = ptr.add(1);
        if (ptr.addr() - base.addr()) % 8 == 0 {
            let save_ptr = ptr;
            ptr = line_ptr;
            KERN.write_str("  ");
            while ptr.addr() < save_ptr.addr() && ptr.addr() < limit.addr() {
                let c = *ptr as char;
                if c.is_ascii_graphic() {
                    KERN.write_str(core::str::from_utf8(&[*ptr]).unwrap());
                } else {
                    KERN.write_str(".");
                }
                ptr = ptr.add(1);
            }
            KERN.write_str("\n");
            line_ptr = ptr;
        }
    }
}

unsafe fn print_addr_header(ptr: *const u8) {
    print_as_hex(ptr.addr(), 8);
    KERN.write_str("  ");
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
        KERN.write_str(core::str::from_utf8(&[HEX_BUF[j]]).unwrap());
    }
}
