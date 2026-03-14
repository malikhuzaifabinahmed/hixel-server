#!/bin/bash
set -e

echo "Building release..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "Copying release binary to 'latest' directory..."
mkdir -p latest
cp target/release/hixelserver latest/

echo "Done! You can find the latest binary at: latest/hixelserver"
