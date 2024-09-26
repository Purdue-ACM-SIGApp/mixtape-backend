#!/usr/bin/env sh

case $1 in
dev)
  SUFFIX="-dev"
  TAG="mixtape-api-dev:latest"
  ;;
prod)
  SUFFIX=""
  TAG="mixtape-api:latest"
  ;;
*)
  echo Invalid Command!
  exit 1
esac

docker rm "$(docker stop "$(docker ps -a -q --filter ancestor=mongo:latest --format="{{.ID}}")")"

cargo clippy

if [ $? -ne 0 ]; then
  exit 1
fi

MONGO_ID=$(docker run -p 27017:27017 -d mongo:latest --noauth)

if [ "$2" = "push" ]; then
  # Copy this in later
  echo unimplemented!
  exit 1
else
  docker buildx build --load -t "$TAG" --build-arg TARGET_CRATE=api --build-arg MONGO_URI=mongodb://172.17.0.2:27017 --target webserver .
fi

docker stop "$MONGO_ID"
docker rm "$MONGO_ID"