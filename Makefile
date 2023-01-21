# include ./tools/colors.mk

BOOT_ASM			=		boot/start.asm
BOOT_ASM			=		boot/
NAME				=		kfs
IMAGE				=		kfs.iso
LINK_SCRIPT			=		scripts/link.ld

.PHONY: all before link image kernel clean fclean re

all: before image kernel
	@mkdir -p bin
	@cp ./build/target/debug/libkernel.a ./bin/libkernel.a
	@echo "$(_GREEN)Build finished !$(_END)"

before:
	@mkdir -p /home/kfs/build

link: 

image:
	@echo -n "Linking image ... "
	@echo "$(_GREEN)Link DONE$(_END)"

kernel:
	@echo "Building kernel ... "
	@cd kernel && cargo build --target-dir=/home/kfs/build -Z build-std=core
	@echo "$(_GREEN)Kernel DONE$(_END)"

clean:
	@rm -rf ./kernel/target
	@rm -rf ./build
	@echo "$(_YELLOW)Intermediate objects deleted !$(_END)" 

fclean: clean
	@rm ./bin
	@echo "$(_YELLOW)Binaries deleted !$(_END)" 

re: fclean all

