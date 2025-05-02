cargo build --target wasm32-unknown-unknown $2

cp ./wasm_helper/index.html ./wasm_helper/dist/$1
cp ./wasm_helper/favicon.ico ./wasm_helper/dist/$1
cp ./wasm_helper/mq_js_bundle.js ./wasm_helper/dist/$1

cp ./target/wasm32-unknown-unknown/debug/$1.wasm ./wasm_helper/dist/$1/game.wasm

cd ./wasm_helper/dist/$1
jwebserver
