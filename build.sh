#!/bin/bash

IMG=monitrust
CNT=monitrustcontainer

docker build . -t $IMG
docker run --name $CNT $IMG
docker cp $CNT:/app/target/release/monitrust ./
docker rm $CNT
docker image prune -f --filter label=stage=intermediate
