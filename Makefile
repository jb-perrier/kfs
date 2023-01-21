include ./tools/colors.mk

.PHONY: all before boot link kernel clean fclean re

all: before boot kernel link 
	@mkdir -p bin
	@echo "Build finished !"

before:
	@mkdir -p /home/kfs/build/kernel

link:
	@echo "$(_CYAN)Linking Boot + Kernel ...$(_END)"
	@ld -m elf_i386 -T /home/kfs/tools/link.ld -o /home/kfs/build/kfs /home/kfs/build/start.o /home/kfs/build/libkernel.a
	@echo "$(_BOLD)$(_GREEN)DONE$(_END)"

kernel:
	@echo "$(_CYAN)Building kernel ...$(_END)"
	@cd kernel && cargo build --target-dir=/home/kfs/build/kernel -Z build-std=core
	@cp ./build/kernel/target/debug/libkernel.a ./build/libkernel.a
	@echo "$(_GREEN)DONE$(_END)"

boot:
	@echo "$(_CYAN)Building boot ...$(_END)"
	@nasm -f elf32 /home/kfs/boot/start.asm -o /home/kfs/build/start.o
	@echo "$(_BOLD)$(_GREEN)DONE$(_END)"

clean:
	@rm -rf ./kernel/target
	@rm -rf ./build
	@echo "$(_YELLOW)Intermediate objects deleted !$(_END)" 

fclean: clean
	@rm ./bin
	@echo "$(_YELLOW)Binaries deleted !$(_END)" 

re: fclean all

