section .text
global _switch_tcb

; param1: TaskControlBlock
_switch_tcb:
    popa
    add esp, 8 ; pop the interrupt number / err code
    iret

section .text
global _start_process

; param1: TaskControlBlock
_start_process:
    push byte 0
    push byte 0
    push byte 0
    push byte 0
    push byte 0

    push byte 0
    push byte 0
    pusha
    