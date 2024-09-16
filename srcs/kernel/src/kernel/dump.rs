pub fn dump(base: usize, limit: usize) {
    let mut i = base;
    while i < limit {
        let ptr = i as *const u8;
        let mut j = 0;
        while j < 16 {
            let byte = unsafe { *ptr.add(j) };
            KERN.write_usize(byte);
            j += 1;
        }
        KERN.write_str("|");
        j = 0;
        while j < 16 {
            let byte = unsafe { *ptr.add(j) };
            if byte.is_ascii_alphanumeric() {
                KERN.write_usize(byte as usize);
            } else {
                KERN.write_str(".");
            }
            j += 1;
        }
        KERN.write_str("\n");
        i += 16;
    }
}