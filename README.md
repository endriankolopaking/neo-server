This is the main server for marseadata build using Rust for reliability, speed and size.

build command `docker build -t huy2840/neo-server .`
run command `docker run -it --rm -d -p 8000:8000 huy2840/neo-server`
attach to docker for inspection `docker container exec -it container-name /bin/bash`