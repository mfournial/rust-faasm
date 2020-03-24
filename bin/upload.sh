#!/bin/bash

set -e

THIS_DIR=$(dirname $(readlink -f $0))
pushd ${THIS_DIR}/.. >> /dev/null

# Default to localhost
HOST=${1:-127.0.0.1}

WASM_FILE=workspace/target/wasm32-unknown-unknown/debug/faasm-wrapper.wasm
FAASM_USER=rust
FAASM_FUNC=hello
UPLOAD_URL=http://${HOST}:8002/f/${FAASM_USER}/${FAASM_FUNC}

echo "Uploading to ${UPLOAD_URL}"
curl -X PUT ${UPLOAD_URL} -T ${WASM_FILE}

popd >> /dev/null
