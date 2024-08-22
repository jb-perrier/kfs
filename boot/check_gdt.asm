global check_gdt

check_gdt:
    ; code segment register
    mov ax, cs
    cmp ax, 0x08
    jne cs_error

    ; data segment register
    mov ax, ds
    cmp ax, 0x10
    jne ds_error

    ; stack segment register
    mov ax, ss
    cmp ax, 0x10
    jne ss_error

    mov eax, 0
    ret

gdt_load_error:
    mov eax, 1
    ret

cs_error:
    mov eax, 2
    ret

ds_error:
    mov eax, 3
    ret

ss_error:
    mov eax, 4
    ret