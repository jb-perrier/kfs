.\tools\install-fake-toolchain-rust-analyzer.bat
docker build ./tools -t kfs-builder
FOR /F "tokens=* USEBACKQ" %%g IN (`docker ps -a -q -f "name=kfs-builder"`) do (SET VAR=%%g)
if [%VAR%] == [] (
    docker run -d --name kfs-builder -v ./:/home/kfs/ kfs-builder
) else (
    docker container start kfs-builder
)
docker exec -t "kfs-builder" sh /home/kfs/tools/docker-entrypoint.sh
docker container stop kfs-builder
