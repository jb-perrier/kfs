global _enable_paging
global _set_page_directory

_set_page_directory:
    mov eax, [esp+4]
    mov cr3, eax
    ret

_enable_paging:
    mov eax, cr0
    or eax, 0x80000000 ; PG bit
    mov cr0, eax
    ret

