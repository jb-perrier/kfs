. $HOME/.cargo/env;
cd /home/kfs/
mkdir bin
cd kernel
mkdir /home/kfs/logs/
cargo build --target-dir=../bin -Z build-std=core