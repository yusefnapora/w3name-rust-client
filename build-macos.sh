#!/usr/bin/env bash

export SDKROOT=$(xcrun -sdk macosx11.1 --show-sdk-path) 
export MACOSX_DEPLOYMENT_TARGET=$(xcrun -sdk macosx11.1 --show-sdk-platform-version) 

cargo build --target=$RUST_BUILD_TARGET