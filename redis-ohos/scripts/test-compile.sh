#!/usr/bin/env bash
set -e

# Test compilation script for redis-ohos
# This script tests if the code compiles without actually building the full binary

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "========================================="
echo "Testing Redis-OHOS Compilation"
echo "========================================="
echo "Project dir: $PROJECT_DIR"
echo ""

cd "$PROJECT_DIR"

# Check if we can compile
echo "Running cargo check..."
cargo check --target aarch64-unknown-linux-ohos

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Compilation check passed!"
    exit 0
else
    echo ""
    echo "✗ Compilation check failed!"
    exit 1
fi

