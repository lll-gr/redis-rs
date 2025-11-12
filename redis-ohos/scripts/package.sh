#!/usr/bin/env bash
set -e

# Package script for HarmonyOS SDK distribution
# Usage: ./package.sh [version]

VERSION="${1:-1.0.0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_DIR/harmonyos-build"
PACKAGE_DIR="$PROJECT_DIR/package"
PACKAGE_NAME="redis-ohos-sdk-${VERSION}"

echo "========================================="
echo "Packaging Redis SDK for HarmonyOS"
echo "========================================="
echo "Version: $VERSION"
echo "Package name: $PACKAGE_NAME"
echo ""

# Check if build directory exists
if [ ! -d "$BUILD_DIR" ]; then
    echo "ERROR: Build directory not found: $BUILD_DIR"
    echo "Please run build script first:"
    echo "  ./scripts/build-harmony.sh release"
    exit 1
fi

# Clean and create package directory
rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR/$PACKAGE_NAME"

echo "Copying build artifacts..."

# Copy build artifacts
cp -r "$BUILD_DIR"/* "$PACKAGE_DIR/$PACKAGE_NAME/"

# Copy documentation
if [ -f "$PROJECT_DIR/README.md" ]; then
    cp "$PROJECT_DIR/README.md" "$PACKAGE_DIR/$PACKAGE_NAME/"
    echo "✓ Copied README.md"
fi

# Copy examples
if [ -d "$PROJECT_DIR/examples" ]; then
    mkdir -p "$PACKAGE_DIR/$PACKAGE_NAME/examples"
    cp -r "$PROJECT_DIR/examples"/* "$PACKAGE_DIR/$PACKAGE_NAME/examples/"
    echo "✓ Copied examples"
fi

# Copy license from parent directory
if [ -f "$PROJECT_DIR/../LICENSE" ]; then
    cp "$PROJECT_DIR/../LICENSE" "$PACKAGE_DIR/$PACKAGE_NAME/"
    echo "✓ Copied LICENSE"
fi

# Create installation guide
cat > "$PACKAGE_DIR/$PACKAGE_NAME/INSTALL.md" << 'EOF'
# Installation Guide

## Quick Start

1. Copy the SDK to your HarmonyOS project:
   ```bash
   cp -r redis-ohos-sdk-*/* /path/to/your/project/entry/libs/Redis_sdk/
   ```

2. Add dependency in `entry/oh-package.json5`:
   ```json5
   {
     "dependencies": {
       "libredis_ohos.so": "file:./libs/Redis_sdk"
     }
   }
   ```

3. Import in your code:
   ```typescript
   import { RedisClient, initLogging } from 'libredis_ohos.so';
   ```

## Directory Structure

- `arm64-v8a/` - ARM64 library (for real devices)
- `armeabi-v7a/` - ARMv7 library (if included)
- `x86_64/` - x86_64 library (if included)
- `index.d.ts` - TypeScript definitions
- `oh-package.json5` - Package metadata
- `examples/` - Usage examples
- `README.md` - Full documentation

## Examples

See the `examples/` directory for:
- `basic_usage.ets` - Basic synchronous operations
- `async_usage.ets` - Async operations and performance testing

For more information, see README.md
EOF

echo "✓ Created INSTALL.md"

# Create version info
cat > "$PACKAGE_DIR/$PACKAGE_NAME/VERSION" << EOF
Version: $VERSION
Build Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Redis-rs Version: 1.0.0-rc.3
EOF

echo "✓ Created VERSION file"

# Create archive
cd "$PACKAGE_DIR"
echo ""
echo "Creating archive..."

# Create tar.gz
tar -czf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"
echo "✓ Created ${PACKAGE_NAME}.tar.gz"

# Create zip
if command -v zip &> /dev/null; then
    zip -r -q "${PACKAGE_NAME}.zip" "$PACKAGE_NAME"
    echo "✓ Created ${PACKAGE_NAME}.zip"
fi

# Calculate checksums
if command -v sha256sum &> /dev/null; then
    sha256sum "${PACKAGE_NAME}.tar.gz" > "${PACKAGE_NAME}.tar.gz.sha256"
    echo "✓ Created SHA256 checksum"
    if [ -f "${PACKAGE_NAME}.zip" ]; then
        sha256sum "${PACKAGE_NAME}.zip" > "${PACKAGE_NAME}.zip.sha256"
    fi
fi

cd "$PROJECT_DIR"

echo ""
echo "========================================="
echo "Package created successfully!"
echo "========================================="
echo ""
echo "Package directory: $PACKAGE_DIR"
echo ""
echo "Files created:"
ls -lh "$PACKAGE_DIR"
echo ""
echo "Package contents:"
echo "- ${PACKAGE_NAME}/ (directory)"
echo "- ${PACKAGE_NAME}.tar.gz"
if [ -f "$PACKAGE_DIR/${PACKAGE_NAME}.zip" ]; then
    echo "- ${PACKAGE_NAME}.zip"
fi
if [ -f "$PACKAGE_DIR/${PACKAGE_NAME}.tar.gz.sha256" ]; then
    echo "- ${PACKAGE_NAME}.tar.gz.sha256"
fi
echo ""
echo "To distribute:"
echo "1. Upload the archive files to your distribution server"
echo "2. Share the download link with users"
echo "3. Users can extract and follow INSTALL.md"

