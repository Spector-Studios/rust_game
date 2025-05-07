#! /bin/sh
./build.sh $1
cd dist/
live-server --wait 500
