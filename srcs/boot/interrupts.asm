extern PROC_ESP
extern PROC_PAGE_DIR

extern execute_signal_from_asm;

%macro INT_COMMON_HANDLER 1
    pushad
    mov [PROC_ESP], esp         ; save esp
    mov eax, cr3
    mov [PROC_PAGE_DIR], eax    ; save eax to PROC_PAGE_DIR
    call %1_handler
    mov esp, [PROC_ESP]         ; set the new esp (in case we are switching task)
    mov eax, [PROC_PAGE_DIR]
    mov cr3, eax                ; set the new page directory (for the same reason)
    call execute_signal_from_asm;
    popad
    add esp, 8                  ; int_no, err_code
    sti
    iretd
%endmacro

extern irq_handler
extern isr_handler

%macro ISR_NOERRCODE 1
global _isr%1

_isr%1:
    cli
    push byte 0
    push byte %1
    INT_COMMON_HANDLER isr
%endmacro

%macro ISR_ERRCODE 1
global _isr%1

_isr%1:
    cli
    push byte %1
    INT_COMMON_HANDLER isr
%endmacro 

%macro IRQ 2
global _irq%1
_irq%1:
    cli
    push byte 0
    push byte %2
    INT_COMMON_HANDLER irq
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

ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 2
ISR_NOERRCODE 3
ISR_NOERRCODE 4
ISR_NOERRCODE 5
ISR_NOERRCODE 6
ISR_NOERRCODE 7
ISR_ERRCODE   8
ISR_NOERRCODE 9
ISR_ERRCODE   10
ISR_ERRCODE   11
ISR_ERRCODE   12
ISR_ERRCODE   13
ISR_ERRCODE   14
ISR_NOERRCODE 15
ISR_NOERRCODE 16
ISR_NOERRCODE 17
ISR_NOERRCODE 18
ISR_NOERRCODE 19
ISR_NOERRCODE 20
ISR_NOERRCODE 21
ISR_NOERRCODE 22
ISR_NOERRCODE 23
ISR_NOERRCODE 24
ISR_NOERRCODE 25
ISR_NOERRCODE 26
ISR_NOERRCODE 27
ISR_NOERRCODE 28
ISR_NOERRCODE 29
ISR_NOERRCODE 30
ISR_NOERRCODE 31

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