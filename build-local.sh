#!/bin/bash
# Local Build and Test Script for b64d Base64 Decoder

set -e

echo "=== Running Tests ==="
cargo test

echo ""
echo "=== Building Release Binary ==="
cargo build --release

echo ""
echo "=== Build Succeeded ==="
echo "Local release binary is available at: target/release/b64d"
