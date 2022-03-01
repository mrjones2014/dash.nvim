#!/usr/bin/env bash

set -e

rm -rf ./lua/libdash_nvim.so ./lua/deps/

DASH_ZIP_TO_DOWNLOAD=""
DASH_LAST_TAG=$(git describe --tags --abbrev=0)
DASH_UNZIP_DIR=""

if [[ "$(uname -m)" == "arm64" ]]; then
  DASH_ZIP_TO_DOWNLOAD="https://github.com/mrjones2014/dash.nvim/releases/download/$DASH_LAST_TAG/arm.zip"
  DASH_UNZIP_DIR="arm"
else
  DASH_ZIP_TO_DOWNLOAD="https://github.com/mrjones2014/dash.nvim/releases/download/$DASH_LAST_TAG/x86.zip"
  DASH_UNZIP_DIR="x86"
fi

rm -rf ./lua/arm/
rm -rf ./lua/x86/
curl -L "$DASH_ZIP_TO_DOWNLOAD" --output libdash.zip
unzip libdash.zip -d ./lua/
mv "./lua/$DASH_UNZIP_DIR/deps/" ./lua/
mv "./lua/$DASH_UNZIP_DIR/libdash_nvim.so" ./lua/
rm -rf "./lua/$DASH_UNZIP_DIR"
