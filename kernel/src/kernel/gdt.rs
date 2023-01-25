#[repr(C)]
pub struct Entry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C)]
pub struct Ptr {
    limit: u16,
    base: u32,
}