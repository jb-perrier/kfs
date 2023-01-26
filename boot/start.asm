MBALIGN  equ  1 << 0
MEMINFO  equ  1 << 1
MBFLAGS  equ  MBALIGN | MEMINFO
MAGIC    equ  0x1BADB002
CHECKSUM equ -(MAGIC + MBFLAGS)
 
; multiboot header
section .multiboot
align 4
	dd MAGIC
	dd MBFLAGS
	dd CHECKSUM
section .bss
align 16
stack_bottom:
resb 16384
stack_top:
 
section .text
global _start:function (_start.end - _start)
_start:
	; we are in 32 bits protected mode
 
	; setup the stack
	; x82 stack grow downwards, so we need to start from the top
	mov esp, stack_top
 
	; TODO: use multiboot_info struct in register EBX to get a nice memory map and pass it to kernel allocator
	; TODO: Enabling floating point instructions, instructions set
	; TODO: load GDT
    ; ABI requires the stack to be aligned on 16 bytes on the call, keep that in mind if adding code above
	push eax
	push ebx
	extern kmain
	call kmain
 
	; infinite loop, since kmain returned while it should not
	cli
.hang:	hlt
	jmp .hang
.end: