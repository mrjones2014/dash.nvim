#!/usr/bin/env bash

set -e

rm -rf ./lua/libdash_nvim.so ./lua/deps/

DASH_NVIM_LATEST_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/mrjones2014/dash.nvim/releases/latest)
# Allow sed; sometimes it's more readable than ${variable//search/replace}
# shellcheck disable=SC2001
DASH_NVIM_LATEST_VERSION=$(echo "$DASH_NVIM_LATEST_RELEASE" | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')

DASH_NVIM_ZIP_TO_DOWNLOAD=""
DASH_UNZIP_DIR=""

if [[ "$(uname -m)" == "arm64" ]]; then
  DASH_NVIM_ZIP_TO_DOWNLOAD="https://github.com/mrjones2014/dash.nvim/releases/download/$DASH_NVIM_LATEST_VERSION/arm.zip"
  DASH_UNZIP_DIR="arm"
else
  DASH_NVIM_ZIP_TO_DOWNLOAD="https://github.com/mrjones2014/dash.nvim/releases/download/$DASH_NVIM_LATEST_VERSION/x86.zip"
  DASH_UNZIP_DIR="x86"
fi

rm -rf ./lua/arm/
rm -rf ./lua/x86/
curl -L "$DASH_NVIM_ZIP_TO_DOWNLOAD" --output libdash.zip
unzip libdash.zip -d ./lua/
mv "./lua/$DASH_UNZIP_DIR/deps/" ./lua/
mv "./lua/$DASH_UNZIP_DIR/libdash_nvim.so" ./lua/
rm -rf "./lua/$DASH_UNZIP_DIR"
