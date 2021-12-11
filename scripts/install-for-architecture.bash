#!/usr/bin/env bash

rm -rf ./lua/libdash_nvim.so ./lua/deps/

if [[ "$(uname -m)" == "arm64" ]]; then
  cp ./bin/arm/libdash_nvim.so ./lua/
  mkdir -p ./lua/deps/
  cp -r ./bin/arm/deps/ ./lua/deps/
  echo "Installed binary for ARM/M1 architecture"
else
  cp ./bin/x86/libdash_nvim.so ./lua/
  mkdir -p ./lua/deps/
  cp -r ./bin/x86/deps/ ./lua/deps/
  echo "Installed binary for x86 architecture"
fi
