sh ./tools/install-fake-toolchain-rust-analyzer.sh

docker build ./tools -t kfs-builder
OUTPUT=$(docker ps -a -q -f "name=kfs-builder")
if test -z "${OUTPUT}"
then
    docker run -d --name kfs-builder -v ./:/home/kfs/ kfs-builder
else
    docker container start kfs-builder
fi
docker exec -t "kfs-builder" bash -c 'export PATH=/root/.cargo/bin:$PATH & make -f /home/kfs/Makefile'
docker container stop kfs-builder
qemu-system-i386 -s -cdrom ./bin/kfs.iso