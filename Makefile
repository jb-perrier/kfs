include ./tools/colors.mk

BOOT_FILES	:=	$(shell cd boot && ls ${prefix}*.asm)
BOOT_SRCS	:=	$(patsubst %.asm,./boot/%.asm,$(BOOT_FILES))
BOOT_OBJS	:=	$(patsubst ./boot/%.asm,./build/%.o,$(BOOT_SRCS))

.PHONY: all before boot link kernel image clean fclean re run

all: before image
	@mkdir -p bin
	@echo "Build finished !"

before:
	@mkdir -p /home/kfs/build/kernel

link: boot kernel
	@echo "$(_CYAN)Linking Boot + Kernel ...$(_END)"
	@ld -m elf_i386 -T /home/kfs/tools/link.ld -o /home/kfs/build/kfs $(BOOT_OBJS) /home/kfs/build/libkernel.a 
	@echo "$(_BOLD)$(_GREEN)Done$(_END)"

kernel:
	@echo "$(_CYAN)Building kernel ...$(_END)"
	@cd kernel && cargo build --target-dir=../build/kernel -Z build-std=core
	@cp ./build/kernel/target/debug/libkernel.a ./build/libkernel.a
	@echo "$(_GREEN)Done$(_END)"

./build/%.o: ./boot/%.asm
	@nasm -f elf32 -g -F dwarf $< -o $@

before_boot:
	@echo "$(_CYAN)Building boot ...$(_END)"

boot: $(BOOT_OBJS)
	@echo "$(_BOLD)$(_GREEN)Done$(_END)"

image: link
	@echo "$(_CYAN)Building image ...$(_END)"
	@mkdir -p ./tmp/boot/grub/
	@cp ./boot/grub.cfg ./tmp/boot/grub/grub.cfg
	@cp ./build/kfs ./tmp/boot
	@grub-file --is-x86-multiboot ./tmp/boot/kfs
	@mkdir -p ./bin/
	@grub-mkrescue -o ./bin/kfs.iso --compress=xz ./tmp/
	@rm -rf ./tmp
	@echo "$(_BOLD)$(_GREEN)Done$(_END)"

clean:
	@rm -rf ./kernel/target
	@rm -rf ./build
	@echo "$(_YELLOW)Intermediate objects deleted !$(_END)" 

fclean: clean
	@rm ./bin
	@echo "$(_YELLOW)Binaries deleted !$(_END)" 

re: fclean all

run:
	@qemu-system-i386 -s -cdrom ./bin/kfs.iso