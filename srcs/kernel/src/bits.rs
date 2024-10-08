use core::mem::size_of;

pub fn get_bit_at(input: usize, n: usize) -> bool {
    if (n < 32) {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub fn set_bit_at(input: &mut usize, n: usize, value: bool) {
    if (n < 32) {
        if value {
            *input |= 1 << n;
        } else {
            *input &= !(1 << n);
        }
    }
}