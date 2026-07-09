#!/bin/bash
# Build script for OwoWork Soroban contract

set -e

echo "Building OwoWork contract..."
echo "============================"

# Build for WASM target
echo "1. Building contract for WASM target..."
cargo build --target wasm32-unknown-unknown --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    WASM_FILE="target/wasm32-unknown-unknown/release/owowork_contract.wasm"
    echo "✓ WASM file created: $WASM_FILE"
    echo "✓ File size: $(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE") bytes"
else
    echo "✗ Build failed!"
    exit 1
fi

echo ""
echo "2. Running tests..."
cargo test

if [ $? -eq 0 ]; then
    echo "✓ All tests passed!"
else
    echo "✗ Tests failed!"
    exit 1
fi

echo ""
echo "3. Checking for warnings..."
cargo check 2>&1 | grep -i warning || echo "✓ No warnings found"

echo ""
echo "Build completed successfully!"