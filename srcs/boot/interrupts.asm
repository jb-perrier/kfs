
global _disable_interrupts

_disable_interrupts:
    cli
    ret

global _enable_interrupts

_enable_interrupts:
    sti
    ret

global _idt_flush

_idt_flush:
    mov eax, [esp+4]
    lidt [eax]
    ret