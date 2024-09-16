extern kmain

extern kernel_virtual_start
extern kernel_virtual_end
extern kernel_physical_start
extern kernel_physical_end

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

global _start:function (_start.end - _start)
_start:
	; we are in 32 bits protected mode
 
	; setup the stack
	; x82 stack grow downwards, so we need to start from the top
	mov esp, stack_top
 
	; TODO: use multiboot_info struct in register EBX to get a nice memory map and pass it to kernel allocator
	; TODO: Enabling floating point instructions, instructions set
	; TODO: load GDT; zill be done in rust kernel
    ; ABI requires the stack to be aligned on 16 bytes on the call

	push ebx
	push eax
	call kmain
 
	; infinite loop, since kmain returned while it should not
	cli
.hang:
	hlt
	jmp .hang
.end:

