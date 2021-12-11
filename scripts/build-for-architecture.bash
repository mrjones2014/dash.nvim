#!/usr/bin/env bash

ARCH=$1
TARGET=""
OUT_TARGET=""

case "$ARCH" in
"macos-arm")
  echo "Building for ARM/M1 architecture..."
  TARGET="aarch64-apple-darwin"
  OUT_TARGET="arm"
  ;;
"macos-x86")
  echo "Building for x86 architecture..."
  TARGET="x86_64-apple-darwin"
  OUT_TARGET="x86"
  ;;
"host")
  CURRENT_ARCH=$(uname -m)
  if [[ "$CURRENT_ARCH" == "arm64" ]]; then
    echo "Building for ARM/M1 architecture..."
    TARGET="aarch64-apple-darwin"
    OUT_TARGET="arm"
  elif [[ "$CURRENT_ARCH" == "x86_64" ]]; then
    echo "Building for x86 architecture..."
    TARGET="x86_64-apple-darwin"
    OUT_TARGET="x86"
  else
    # >&2 makes it go to STDERR
    echo >&2 "Unknown architecture '$CURRENT_ARCH'"
    exit 1
  fi
  ;;
"")
  echo "Architecture not passed, pass 'host' as argument to auto-detect architecture."
  exit 1
  ;;
esac

cargo build --release --target $TARGET
rm -rf ./bin/$OUT_TARGET/
mkdir -p ./bin/$OUT_TARGET/deps/
cp ./target/$TARGET/release/libdash_nvim.dylib ./bin/$OUT_TARGET/libdash_nvim.so
cp ./target/$TARGET/release/deps/*.rlib ./bin/$OUT_TARGET/deps/
