.\tools\install-fake-toolchain-rust-analyzer.ps1
cd kernel
cargo fmt
cd ..
docker build ./tools -t kfs-builder
$result = docker ps -a -q -f "name=kfs-builder"
if ([string]::IsNullOrEmpty($result))
{
    docker run -d --name kfs-builder -v ${PWD}/:/home/kfs/ kfs-builder
} else {
    docker container start kfs-builder
}
# docker exec -t "kfs-builder" sh /home/kfs/tools/docker-entrypoint.sh
docker exec -t "kfs-builder" bash -c 'export PATH=/root/.cargo/bin:$PATH & make -f /home/kfs/Makefile'

# docker kill container kfs-builder
