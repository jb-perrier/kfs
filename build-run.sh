docker build ./docker-builder -t kfs-builder
docker container prune -f
docker run -d --name kfs-builder -v %~dp0:/home/kfs kfs-builder