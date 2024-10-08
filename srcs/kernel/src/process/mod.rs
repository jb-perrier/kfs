use crate::mem::{frame::FrameAllocator, heap::Heap, paging::directory::PageDirectory};

#[repr(C)]
#[derive(Default)]
pub struct Process {
    pub page_directory: Option<*mut PageDirectory>,
    pub heap: Option<Heap>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            page_directory: None,
            heap: None,
        }
    }

    pub fn page_directory(&self) -> *mut PageDirectory {
        self.page_directory.unwrap()
    }

    pub fn heap(&self) -> &Heap {
        self.heap.as_ref().unwrap()
    }

    pub fn heap_mut(&mut self) -> &mut Heap {
        self.heap.as_mut().unwrap()
    }
}