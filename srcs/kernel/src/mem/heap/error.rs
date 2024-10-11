#[derive(Debug, Clone, Copy)]
pub enum Error {
    OutOfMemory,
    Unallocated,
    NotOwned,
}