#!/usr/bin/env bash
set -euo pipefail
cargo build --release -p tundra-cli
BIN=target/release/tundra-cli
TGT=${BIN##*/}                # tundra-cli
VER=$(git describe --tags --abbrev=0 2>/dev/null || git rev-parse --short HEAD)
ARCH=$(uname -m)              # x86_64, arm64, etc.

tar czvf "${TGT}-${VER}-linux-${ARCH}.tar.gz" -C target/release "${TGT}"

echo "Built and packaged: ${TGT}-${VER}-linux-${ARCH}.tar.gz"
