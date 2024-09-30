OUTPUT_FORMAT(elf32-i386)
ENTRY(_start)

_KERNEL_BASE = 0x4000000;
SECTIONS
{
    . = 1M;

    .boot : {
		*(.multiboot_header) /* ensure that the multiboot header is at the beginning */
		*(.boottext)
	}

    . = _KERNEL_BASE;
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
    _KERNEL_END = .;
}