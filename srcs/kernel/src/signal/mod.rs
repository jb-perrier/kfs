static mut SIGNAL_AUEUE: [Signal; 32] = [Signal::Null; 32];

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Signal {
    #[default] Null,
    Exit,
}

pub fn send_signal(signal: Signal) {
    let queue = signal_queue();
    for signal_slot in queue {
        if *signal_slot == Signal::Null {
            *signal_slot = signal;
            break;
        }
    }
}

fn signal_queue() -> &'static mut [Signal; 32] {
    unsafe { &mut SIGNAL_AUEUE }
}