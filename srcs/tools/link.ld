OUTPUT_FORMAT(elf32-i386)
ENTRY(_start)

SECTIONS
{
    . = 2M;

    _KERNEL_START = .;
    .text ALIGN(4K):
    {
        code = .;
        *(.multiboot)
        *(.text)
    }
    .rodata ALIGN(4K):
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
    _KERNEL_END = ALIGN(4K);
}