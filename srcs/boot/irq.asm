extern irq_handler

%macro IRQ 2
global _irq%1
_irq%1:
    cli
    push byte 0
    push byte %2
    jmp irq_common_stub
%endmacro

IRQ 0, 32
IRQ 1, 33
IRQ 2, 34
IRQ 3, 35
IRQ 4, 36
IRQ 5, 37
IRQ 6, 38
IRQ 7, 39
IRQ 8, 40
IRQ 9, 41
IRQ 10, 42
IRQ 11, 43
IRQ 12, 44
IRQ 13, 45
IRQ 14, 46
IRQ 15, 47 

irq_common_stub:
    pushad
    push esp
    call irq_handler
    add esp, 4 ; esp
    cmp eax, 0
    je .no_change
    mov esp, eax
.no_change:
    popad
    add esp, 8 ; int_no, err_code
    sti
    iretd