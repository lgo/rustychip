#!/bin/bash
#
# revendor.sh will rebuild the Cargo vendor dependencies.

set -e

cd third_party/cargo
rm -rf remote
cargo raze

echo "Completed... Attempting to build."

bazel build //third_party/cargo/...