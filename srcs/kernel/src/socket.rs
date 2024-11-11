use alloc::{string::String, vec::Vec};

use crate::process::ProcessId;

pub struct Socket {
    pub name: String,
    pub data: Vec<SocketData>,
}

impl Socket {
    pub fn new(name: String) -> Socket {
        Socket {
            name,
            data: Vec::new(),
        }
    }

    pub fn send(&mut self, sender_id: ProcessId, payload: Vec<u8>) {
        self.data.push(SocketData {
            sender: sender_id,
            payload,
        });
    }

    pub fn receive(&mut self, receiver_id: ProcessId) -> Option<SocketData> {
        let mut index = None;
        for (i, data) in self.data.iter().enumerate() {
            if data.sender != receiver_id {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            Some(self.data.remove(i))
        } else {
            None
        }
    }
}

pub struct SocketData {
    pub sender: ProcessId,

    pub payload: Vec<u8>,
}
