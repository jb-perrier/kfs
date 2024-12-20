#[no_mangle]
pub unsafe extern "C" fn strlen(ptr: *const char) -> isize {
    if ptr.is_null() {
        return 0;
    }
    let mut i = 0;
    while *ptr.offset(i) != '\0' {
        i += 1;
    }
    i
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: isize) -> *const u8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }
    let mut i = 0;
    while i < n {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut i32, c: i32, n: isize) -> *mut i32 {
    if s.is_null() {
        return s;
    }
    let mut i = 0;
    while i < n {
        core::ptr::write_unaligned(s, c);
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: isize) -> i32 {
    let mut i = 0;
    while i < n {
        let left = *s1.offset(i) as i32;
        let right = *s2.offset(i) as i32;
        if left != right {
            return left - right;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: isize) {
    if dest.is_null() || src.is_null() || n <= 0{
        return;
    }
    if (src as usize) < (dest as usize) {
        for i in (n - 1)..=0 {
            let value = core::ptr::read_unaligned(src.offset(i));
            core::ptr::write_unaligned(dest.offset(i), value);
        }
    } else {
        for i in 0..n {
            let value = core::ptr::read_unaligned(src.offset(i));
            core::ptr::write_unaligned(dest.offset(i), value);
        }
    }
}