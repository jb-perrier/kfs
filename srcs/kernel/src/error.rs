#[derive(Debug)]
pub enum KernelError {
    NoMemoryMap,
    NoSuitableMemoryRegionFound,
    FreeUnallocated,
    InvalidPointer,
    FrameOutOfMemory,
    HeapOutOfMemory,
    PagDirectoryFull,
    Unknown,
}
