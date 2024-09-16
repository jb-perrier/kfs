use super::KERN;

pub unsafe fn dump(mut base: *const u8, limit: *const u8) {
    // KERN.write_str("dump: ");
    // KERN.write_usize(base as usize);
    // KERN.write_str(" => ");
    // KERN.write_usize(limit as usize);
    // KERN.write_str("\n");

    let mut ptr = base;
    while ptr.addr() < limit.addr() {
        if (ptr.addr() - base.addr()) % 8 == 0 {
            print_addr_header(ptr);
        } else if (ptr.addr() - base.addr()) % 2 == 0 {
            KERN.write_str(" ");
        }
        print_as_hex(*ptr as usize, 2);
        ptr = ptr.add(1);
        if (ptr.addr() - base.addr()) % 8 == 0 {
            KERN.write_str("\n");
        }
    }
}

unsafe fn print_addr_header(ptr: *const u8) {
    print_as_hex(ptr.addr(), 8);
    KERN.write_str(": ");
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
