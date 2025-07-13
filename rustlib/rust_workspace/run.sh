#! /bin/bash

: "${WASM_OUTPUT_DIR="dist"}"
export WASM_OUTPUT_DIR

./wasm_build.sh "$@" || exit 1
cd ${WASM_OUTPUT_DIR}/ || exit 1
live-server --wait=1000
