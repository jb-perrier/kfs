section .text
global clear_registers

clear_registers:
    xor eax, eax
    xor ebx, ebx
    xor ecx, ecx
    xor edx, edx
    xor esi, esi
    xor edi, edi
    xor ebp, ebp
    xor esp, esp
    ret

section .text
global _update_stack_pointers
_update_stack_pointers:
    mov ebp, [esp + 8]  ; Get the second parameter (new EBP value)
    mov esp, [esp + 4]  ; Get the first parameter (new ESP value)
    ret