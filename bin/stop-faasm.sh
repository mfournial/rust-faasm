#!/bin/bash

set -e

THIS_DIR=$(dirname $(readlink -f $0))
FAASM_DIR=${THIS_DIR}/../third-party/faasm

pushd ${FAASM_DIR} >> /dev/null

docker-compose stop

popd >> /dev/null