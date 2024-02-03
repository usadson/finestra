#!/bin/sh

VERSION=$(cargo pkgid | cut -d "#" -f2)

cargo build

mkdir -p showcase.app/Contents/MacOS
mkdir -p showcase.app/Resources
cd showcase.app

cp ../../target/debug/showcase Contents/MacOS/showcase
cp ../Info.plist Contents/
(sed -e "s/CARGO_VERSION/$VERSION/g" ../Info.plist) > ./Contents/Info.plist
