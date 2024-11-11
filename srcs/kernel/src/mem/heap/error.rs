#[derive(Debug, Clone, Copy)]
pub enum HeapError {
    OutOfMemory,
    Unallocated,
    NotOwned,
}
