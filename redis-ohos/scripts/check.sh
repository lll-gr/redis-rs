#!/usr/bin/env bash
set -e

# Check script for HarmonyOS development environment
# Usage: ./check.sh

echo "========================================="
echo "Checking HarmonyOS Development Environment"
echo "========================================="
echo ""

# Check Rust installation
echo "Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✓ Rust installed: $RUST_VERSION"
else
    echo "✗ Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Cargo
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo "✓ Cargo installed: $CARGO_VERSION"
else
    echo "✗ Cargo not found."
    exit 1
fi

echo ""

# Check ohrs installation
echo "Checking ohrs installation..."
if command -v ohrs &> /dev/null; then
    OHRS_VERSION=$(ohrs --version 2>&1 || echo "unknown")
    echo "✓ ohrs installed: $OHRS_VERSION"
else
    echo "✗ ohrs not found. Please install it:"
    echo "  cargo install ohrs"
    exit 1
fi

echo ""

# Check OHOS_NDK_HOME
echo "Checking OHOS_NDK_HOME..."
if [ -z "$OHOS_NDK_HOME" ]; then
    echo "⚠ OHOS_NDK_HOME not set."
    echo "  ohrs will use its default NDK location."
    echo "  To set it manually, add to your shell profile:"
    echo "    export OHOS_NDK_HOME=/path/to/ohos-sdk/native"
else
    echo "✓ OHOS_NDK_HOME set: $OHOS_NDK_HOME"
    if [ -d "$OHOS_NDK_HOME" ]; then
        echo "✓ OHOS_NDK_HOME directory exists"
    else
        echo "✗ OHOS_NDK_HOME directory does not exist: $OHOS_NDK_HOME"
    fi
fi

echo ""

# Check Rust targets for HarmonyOS
echo "Checking Rust targets for HarmonyOS..."
TARGETS=(
    "aarch64-unknown-linux-ohos"
    "armv7-unknown-linux-ohos"
    "x86_64-unknown-linux-ohos"
)

for TARGET in "${TARGETS[@]}"; do
    if rustup target list --installed | grep -q "$TARGET"; then
        echo "✓ Target installed: $TARGET"
    else
        echo "⚠ Target not installed: $TARGET"
        echo "  Install with: rustup target add $TARGET"
    fi
done

echo ""

# Check project structure
echo "Checking project structure..."
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

if [ -f "$PROJECT_DIR/Cargo.toml" ]; then
    echo "✓ Cargo.toml found"
else
    echo "✗ Cargo.toml not found in $PROJECT_DIR"
    exit 1
fi

if [ -f "$PROJECT_DIR/build.rs" ]; then
    echo "✓ build.rs found"
else
    echo "✗ build.rs not found"
fi

if [ -d "$PROJECT_DIR/src" ]; then
    echo "✓ src directory found"
else
    echo "✗ src directory not found"
    exit 1
fi

echo ""

# Check dependencies in Cargo.toml
echo "Checking key dependencies..."
if grep -q "napi-ohos" "$PROJECT_DIR/Cargo.toml"; then
    echo "✓ napi-ohos dependency found"
else
    echo "✗ napi-ohos dependency not found in Cargo.toml"
fi

if grep -q "napi-derive-ohos" "$PROJECT_DIR/Cargo.toml"; then
    echo "✓ napi-derive-ohos dependency found"
else
    echo "✗ napi-derive-ohos dependency not found in Cargo.toml"
fi

if grep -q "redis" "$PROJECT_DIR/Cargo.toml"; then
    echo "✓ redis dependency found"
else
    echo "✗ redis dependency not found in Cargo.toml"
fi

echo ""
echo "========================================="
echo "Environment check completed!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. If any targets are missing, install them with:"
echo "   rustup target add aarch64-unknown-linux-ohos"
echo "2. Build the project with:"
echo "   ./scripts/build-harmony.sh [debug|release]"
echo "3. Clean build artifacts with:"
echo "   ./scripts/clean.sh"

