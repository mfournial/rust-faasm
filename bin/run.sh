#!/bin/bash

set -e

# Default to localhost
HOST=${1:-127.0.0.1}
FAASM_USER=rust
FAASM_FUNC=hello

INVOKE_URL=http://${HOST}:8080/

echo "Invoking ${FAASM_USER}/${FAASM_FUNC} at ${INVOKE_URL}"

curl -X POST \
  -H "Content-Type: application/json" \
  --data '{"user": "'$FAASM_USER'", "function": "'$FAASM_FUNC'"}' \
  ${INVOKE_URL}

