use super::Process;

pub struct Scheduler {
    pub current: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler { current: 0 }
    }

    pub fn next(&mut self, processes: &[Process]) -> usize {
        if self.current >= processes.len() {
            self.current = 0;
        }
        
        let next = self.current;

        if self.current + 1 >= processes.len() {
            self.current = 0;
        } else {
            self.current += 1;
        }

        next
    }
}