#!/usr/bin/env bash

if [[ "$(uname -m)" == "arm64" ]]; then
  cp ./bin/dash_lib_arm.dylib ./lua/dash_lib.so
else
  cp ./bin/dash_lib_x86.dylib ./lua/dash_lib.so
fi
