#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/multicam
cargo build --release --target=${TARGET_ARCH}