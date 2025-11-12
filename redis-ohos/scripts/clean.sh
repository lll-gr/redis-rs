#!/usr/bin/env bash
set -e

# Clean script for HarmonyOS build artifacts
# Usage: ./clean.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "========================================="
echo "Cleaning Redis SDK for HarmonyOS"
echo "========================================="
echo "Project dir: $PROJECT_DIR"
echo ""

# Clean cargo build artifacts
echo "Cleaning cargo build artifacts..."
cd "$PROJECT_DIR"
cargo clean

# Clean output directory
OUTPUT_DIR="$PROJECT_DIR/harmonyos-build"
if [ -d "$OUTPUT_DIR" ]; then
    echo "Removing output directory: $OUTPUT_DIR"
    rm -rf "$OUTPUT_DIR"
fi

# Clean dist directory (ohrs generated files)
DIST_DIR="$PROJECT_DIR/dist"
if [ -d "$DIST_DIR" ]; then
    echo "Removing dist directory: $DIST_DIR"
    rm -rf "$DIST_DIR"
fi

# Clean target directory
TARGET_DIR="$PROJECT_DIR/../../target"
if [ -d "$TARGET_DIR" ]; then
    echo "Cleaning target directory..."
    # Only clean OHOS-specific targets to avoid affecting other builds
    rm -rf "$TARGET_DIR/aarch64-unknown-linux-ohos"
    rm -rf "$TARGET_DIR/armv7-unknown-linux-ohos"
    rm -rf "$TARGET_DIR/x86_64-unknown-linux-ohos"
fi

echo ""
echo "========================================="
echo "Clean completed successfully!"
echo "========================================="

