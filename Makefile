.PHONY: all clean fclean re run

all:
	cd srcs;sh build-run.sh

build:
	cd srcs;sh build.sh

run:
	cd srcs;sh run.sh

clean:
	cd srcs;sh clean.sh

fclean: clean
	docker container prune -f

re: fclean all
