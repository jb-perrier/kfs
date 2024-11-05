use super::Process;

pub struct Scheduler {
    pub current: usize,
    pub running: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler { current: 0, running: false }
    }

    pub fn next(&mut self, processes: &[Process]) -> usize {
        if self.current + 1 >= processes.len() {
            self.current = 0;
        } else {
            self.current += 1;
        }

        self.current
    }

    pub fn run(&mut self) {
        self.running = true;
    }
}