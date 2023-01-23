include /home/kfs/tools/colors.mk

BOOT_FILES	:=	$(shell cd /home/kfs/boot && ls ${prefix}*.asm)
BOOT_SRCS	:=	$(patsubst %.asm,/home/kfs/boot/%.asm,$(BOOT_FILES))
BOOT_OBJS	:=	$(patsubst /home/kfs/boot/%.asm,/home/kfs/build/%.o,$(BOOT_SRCS))

.PHONY: all before boot link kernel image clean fclean re run

all: before image
	@mkdir -p /home/kfs/bin
	@echo "Build finished !"

before:
	@mkdir -p /home/kfs/build/kernel

link: boot kernel
	@echo "$(_CYAN)Linking Boot + Kernel ...$(_END)"
	@ld -m elf_i386 -T /home/kfs/tools/link.ld -o /home/kfs/build/kfs.bin $(BOOT_OBJS) /home/kfs/build/libkernel.a 
	@echo "$(_BOLD)$(_GREEN)Done$(_END)"

kernel:
	@echo "$(_CYAN)Building kernel ...$(_END)"
	@cd /home/kfs/kernel && cargo build --target-dir=../build/kernel -Z build-std=core
	@cp /home/kfs/build/kernel/target/debug/libkernel.a /home/kfs/build/libkernel.a
	@echo "$(_GREEN)Done$(_END)"

/home/kfs/build/%.o: /home/kfs/boot/%.asm
	@nasm -f elf32 -g -F dwarf $< -o $@

before_boot:
	@echo "$(_CYAN)Building boot ...$(_END)"

boot: $(BOOT_OBJS)
	@echo "$(_BOLD)$(_GREEN)Done$(_END)"

image: link
	@echo "$(_CYAN)Building image ...$(_END)"
	@mkdir -p /home/kfs/tmp/boot/grub/
	@cp /home/kfs/boot/grub.cfg /home/kfs/tmp/boot/grub/grub.cfg
	@cp /home/kfs/build/kfs.bin /home/kfs/tmp/boot
	@grub-file --is-x86-multiboot /home/kfs/tmp/boot/kfs.bin
	@mkdir -p /home/kfs/bin/
	@grub-mkrescue -o /home/kfs/bin/kfs.iso --compress=xz /home/kfs/tmp/
	@rm -rf /home/kfs/tmp
	@echo "$(_BOLD)$(_GREEN)Done$(_END)"

clean:
	@rm -rf /home/kfs/kernel/target
	@rm -rf /home/kfs/build
	@echo "$(_YELLOW)Intermediate objects deleted !$(_END)" 

fclean: clean
	@rm /home/kfs/bin
	@echo "$(_YELLOW)Binaries deleted !$(_END)" 

re: fclean all

run:
	@qemu-system-i386 -s -cdrom /home/kfs/bin/kfs.iso