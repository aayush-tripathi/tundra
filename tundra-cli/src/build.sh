#!/usr/bin/env bash
set -e

# build only the CLI crate
cargo build --release -p tundra-cli

# report correct binary name
echo "Built target/release/tundra-cli"
