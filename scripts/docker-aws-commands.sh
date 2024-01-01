#!/bin/bash

IMAGE_NAME="map-snap-backend"

# Using `--no-cache` will rebuild from scratch.
docker build --tag "$IMAGE_NAME" .
docker build --tag "$IMAGE_NAME" --platform="linux/amd64" .
docker run --detach --publish 8000:8000 --name "$IMAGE_NAME" "$IMAGE_NAME"

docker start "$IMAGE_NAME"
docker stop "$IMAGE_NAME"

docker rm "$IMAGE_NAME"

# AWS ECR
REGISTRY_URL="759384306288.dkr.ecr.us-east-1.amazonaws.com"

aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin "$REGISTRY_URL"

aws ecr create-repository --repository-name "$IMAGE_NAME" --region us-east-1

docker tag "$IMAGE_NAME":latest "$REGISTRY_URL"/"$IMAGE_NAME":latest
docker push "$REGISTRY_URL"/"$IMAGE_NAME"
docker image inspect "$REGISTRY_URL"/"$IMAGE_NAME":latest
