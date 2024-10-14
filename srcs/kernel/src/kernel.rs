use core::ptr::addr_of_mut;

use multiboot::information::Multiboot;

use crate::{infinite_loop, mem::frame::FrameAllocator, process::Process, text};

pub struct Kernel<'a, 'b> {
    pub multiboot: Option<Multiboot<'a, 'b>>,
    pub frame_allocator: Option<FrameAllocator>,
    pub process: Process,
}

// impl<'a, 'b> Kernel<'a, 'b> {
//     pub fn new() -> Self {
//         Self {
//             multiboot: None,
//             frame_allocator: None,
//             process: Process::new(),
//         }
//     }
// }