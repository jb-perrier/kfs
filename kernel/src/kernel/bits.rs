use core::mem::size_of;

pub fn get_bit_at(input: u32, n: usize) -> bool {
    if (n < 32) {
        input & (1 << n) != 0
    } else {
        false
    }
}
