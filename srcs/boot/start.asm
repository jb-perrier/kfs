extern kmain

extern _KERNEL_START
extern _KERNEL_END

MBALIGN  equ  1 << 0
MEMINFO  equ  1 << 1
MBFLAGS  equ  MBALIGN | MEMINFO
MAGIC    equ  0x1BADB002
CHECKSUM equ -(MAGIC + MBFLAGS)

KERNEL_STACK_SIZE equ 16384
; multiboot header
section .multiboot
align 4
	dd MAGIC
	dd MBFLAGS
	dd CHECKSUM

section .bss
align 16
stack_bottom:
resb KERNEL_STACK_SIZE
stack_top:
 
section .text

global get_stack_top
get_stack_top:
	mov eax, stack_top
	ret

global get_stack_bottom
get_stack_bottom:
	mov eax, stack_bottom
	ret

global get_stack_ptr
get_stack_ptr:
	mov eax, esp
	ret

global _start
_start:
	; we are in 32 bits protected mode
 
	; setup the stack
	; x82 stack grow downwards, so we need to start from the top
	mov esp, stack_top
 
    ; ABI requires the stack to be aligned on 16 bytes on the call

	push ebx
	push eax
	call kmain
 
	cli
.hang:
	hlt
	jmp .hang
.end:

