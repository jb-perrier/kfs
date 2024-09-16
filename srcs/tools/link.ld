OUTPUT_FORMAT(elf32-i386)
ENTRY(_start)
SECTIONS
{
    . = 1M;
    kernel_virtual_start = .;
    kernel_physical_start = . - 1M;
    .text ALIGN(4K):
    {
        code = .;
        *(.multiboot)
        *(.text)
    }
    .rodata  ALIGN(4K):
    {
        *(.rodata)
    }
    .data ALIGN(4K):
    {
        data = .;
        *(.data)
    }
    .bss ALIGN(4K):
    {
        bss = .;
        *(.bss)
    }
    kernel_virtual_end = .;
    kernel_physical_end = . - 1M;
}