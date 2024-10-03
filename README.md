Qemu ram: 256 MB
Higher half kernel constrained to ram
Physical:
2MB Reserved for BIOS/UEFI
_KERNEL_START
_KERNEL_END
Align 4K
Pages until end of ram

Virtual:
[0x00000000 – 0xBFFFFFFF] 3GB : User space
[0xC0000000 – 0xFFFFFFFF] 1GB : Kernel space

Not working:
- Be aware of big variables on the stack crash the kernel sometimes, prefer const / static outside of function.