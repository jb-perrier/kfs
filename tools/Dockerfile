FROM ubuntu:latest

RUN apt update
RUN apt upgrade
RUN apt install curl -y
RUN apt install build-essential -y
RUN apt install nasm -y
RUN apt install grub2 -y
RUN apt install xorriso -y

RUN curl https://sh.rustup.rs -sSf > rustup-init.sh ; sh ./rustup-init.sh -y; rm rustup-init.sh; . $HOME/.cargo/env; rustup default nightly; \
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

RUN mkdir /home/kfs

ENTRYPOINT ["tail", "-f", "/dev/null"]