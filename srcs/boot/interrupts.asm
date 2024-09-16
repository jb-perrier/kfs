
global disable_interrupts

disable_interrupts:
    cli
    ret

global enable_interrupts

enable_interrupts:
    sti
    ret