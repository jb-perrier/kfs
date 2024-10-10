section .text
global _divide_zero

_divide_zero:
    mov eax, 10
    mov ebx, 0
    div ebx
    ret