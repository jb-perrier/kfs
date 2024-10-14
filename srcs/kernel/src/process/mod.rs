use crate::mem::{frame::FrameAllocator, heap::Heap, paging::directory::PageDirectory};

#[repr(C)]
pub struct Process {
    pub page_directory: *mut PageDirectory,
    pub heap: Heap,
}

// impl Process {
//     pub fn new() -> Self {
//         Self {
//         }
//     }

// }