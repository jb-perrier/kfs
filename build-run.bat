.\scripts\install-fake-toolchain-rust-analyzer.bat

docker build ./docker-builder -t kfs-builder
FOR /F "tokens=* USEBACKQ" %%g IN (`docker ps -a -q -f "name=kfs-builder"`) do (SET VAR=%%g)
if [%VAR%] == [] (
    docker run -t --name kfs-builder -v %~dp0:/home/kfs/ kfs-builder
) else (
    docker container start kfs-builder
)

