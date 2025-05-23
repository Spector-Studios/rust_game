#! /bin/bash

rm -r dist

cargo build --target=wasm32-unknown-unknown $1

mkdir dist -p

for name in dungeoncrawl srpg_game
do

  mkdir dist/$name/resources -p
  cp wasm_helper/game/* dist/$name/ -r
  
  cp resources/$name/* dist/$name/resources/ -r
  cp wasm_helper/index.html dist/index.html

  if [ "$1" == "--release" ]
  then
    cp target/wasm32-unknown-unknown/release/$name.wasm dist/$name/game.wasm
  else
    cp target/wasm32-unknown-unknown/debug/$name.wasm dist/$name/game.wasm
  fi

done
