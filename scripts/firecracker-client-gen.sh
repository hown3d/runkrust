#!/usr/bin/env sh
set -ex

RESP=$(curl -X POST --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  -d '{"openAPIUrl": "https://raw.githubusercontent.com/firecracker-microvm/firecracker/v1.4.1/src/api_server/swagger/firecracker.yaml", "options": {"packageName": "firecracker-client"}}' \
  'https://api.openapi-generator.tech/api/gen/clients/rust')

# {"code":"c2d483.3.4672-40e9-91df-b9ffd18d22b8","link":"http://localhost:8888/api/gen/download/c2d483.3.4672-40e9-91df-b9ffd18d22b8"}

code=$(echo $RESP | jq -r .code )

curl -o client.zip https://api.openapi-generator.tech/api/gen/download/$code

unzip client.zip
rm client.zip
mv rust-client firecracker-client