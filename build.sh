#!/bin/bash
set -e

echo "Building release..."
cargo build --release --target x86_64-unknown-linux-musl

echo "Copying release binary to 'latest' directory..."
mkdir -p latest
cp target/x86_64-unknown-linux-musl/release/hixelserver latest/

echo "Done! You can find the latest binary at: latest/hixelserver"
