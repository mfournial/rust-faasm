#!/bin/bash

set -e

FAASM_USER=rust
FAASM_FUNC=hello

FAASM_ROOT=/usr/local/code/faasm
FUNC_DIR=${FAASM_ROOT}/wasm/${FAASM_USER}/${FAASM_FUNC}
mkdir -p ${FUNC_DIR}

FUNC_WASM=workspace/target/wasm32-unknown-unknown/debug/faasm-wrapper.wasm
cp ${FUNC_WASM} ${FUNC_DIR}/function.wasm

codegen_func ${FAASM_USER} ${FAASM_FUNC}
simple_runner ${FAASM_USER} ${FAASM_FUNC}
