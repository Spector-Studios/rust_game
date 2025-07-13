#! /bin/bash

: "${WASM_OUTPUT_DIR:="dist"}"

rm -r "${WASM_OUTPUT_DIR}"

cargo build --target=wasm32-unknown-unknown "$@" || exit 1

mkdir dist -p

is_release_build=false
  
for arg in "$@"; do
  if [ "$arg" = "web-release" ]
  then
    is_release_build=true
  fi
done

for name in dungeoncrawl srpg_game
do
  mkdir "${WASM_OUTPUT_DIR}"/${name}/assets -p
  cp wasm_helper/game/* "${WASM_OUTPUT_DIR}"/${name}/ -r
  
  cp ${name}/assets/* "${WASM_OUTPUT_DIR}"/${name}/assets/ -r
  cp wasm_helper/index.html "${WASM_OUTPUT_DIR}"/index.html
  cp wasm_helper/favicon.ico "${WASM_OUTPUT_DIR}"/favicon.ico
    
  if [ "$is_release_build" = true ]
  then
    cp target/wasm32-unknown-unknown/web-release/${name}.wasm "${WASM_OUTPUT_DIR}"/${name}/game.wasm
  else
    cp target/wasm32-unknown-unknown/debug/${name}.wasm "${WASM_OUTPUT_DIR}"/${name}/game.wasm
  fi

done
