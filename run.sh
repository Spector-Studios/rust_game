#! /bin/sh
./build.sh $1
cd dist/
jwebserver
