Qemu ram: 128 MB
Higher half kernel constrained to ram
Physical:

Virtual:
[0x400 0000 – 0x700 0000] 33.554.432 bytes : Kernel space
[0x000 0000 – 0x3FF FFFF] 67.108.863 bytes : User space

Not working:
- Be aware of big variables on the stack crash the kernel sometimes, prefer const / static outside of function.