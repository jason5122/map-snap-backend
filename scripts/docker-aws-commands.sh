#!/bin/bash

docker build --tag docker-rust-image .
docker build --tag docker-rust-image --platform="linux/amd64" . # build for x86
docker run --detach --publish 8000:8000 --name docker-rust-container docker-rust-image

docker stop docker-rust-container
docker start docker-rust-container

docker rm docker-rust-container

# AWS ECR
REGISTRY_URL="759384306288.dkr.ecr.us-east-1.amazonaws.com"

aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin "$REGISTRY_URL"

docker tag docker-rust-image:latest "$REGISTRY_URL"/my-repository
docker push "$REGISTRY_URL"/my-repository
docker image inspect "$REGISTRY_URL"/my-repository:latest # verify arch
