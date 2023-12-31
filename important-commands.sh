#!/bin/bash

docker build --tag docker-rust-image .
docker run --detach --publish 3001:8000 --name docker-rust-container docker-rust-image

docker stop docker-rust-container
docker start docker-rust-container

docker rm docker-rust-container
