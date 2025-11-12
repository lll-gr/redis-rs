# Makefile for Redis HarmonyOS SDK

.PHONY: help check clean build build-debug build-release build-all package install

# Default target
help:
	@echo "Redis HarmonyOS SDK - Build Targets"
	@echo "===================================="
	@echo ""
	@echo "Available targets:"
	@echo "  make check         - Check development environment"
	@echo "  make build         - Build debug version (ARM64 only)"
	@echo "  make build-debug   - Build debug version (ARM64 only)"
	@echo "  make build-release - Build release version (ARM64 only)"
	@echo "  make build-all     - Build all architectures (release)"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make package       - Create distribution package"
	@echo "  make install       - Install to HarmonyOS project (set OHOS_PROJECT_PATH)"
	@echo ""
	@echo "Examples:"
	@echo "  make build-release"
	@echo "  make build-all"
	@echo "  make package VERSION=1.0.0"
	@echo "  make install OHOS_PROJECT_PATH=/path/to/project"

# Check development environment
check:
	@./scripts/check.sh

# Clean build artifacts
clean:
	@./scripts/clean.sh

# Build debug version (ARM64 only)
build: build-debug

build-debug:
	@./scripts/build-harmony.sh debug

# Build release version (ARM64 only)
build-release:
	@./scripts/build-harmony.sh release

# Build all architectures
build-all:
	@./scripts/build-all-targets.sh release

# Create distribution package
package:
	@./scripts/package.sh $(VERSION)

# Install to HarmonyOS project
install:
ifndef OHOS_PROJECT_PATH
	@echo "ERROR: OHOS_PROJECT_PATH not set"
	@echo "Usage: make install OHOS_PROJECT_PATH=/path/to/your/harmonyos/project"
	@exit 1
endif
	@echo "Installing Redis SDK to HarmonyOS project..."
	@if [ ! -d "harmonyos-build" ]; then \
		echo "ERROR: Build directory not found. Run 'make build-release' first."; \
		exit 1; \
	fi
	@mkdir -p "$(OHOS_PROJECT_PATH)/entry/libs/Redis_sdk"
	@cp -r harmonyos-build/* "$(OHOS_PROJECT_PATH)/entry/libs/Redis_sdk/"
	@echo "✓ SDK installed to $(OHOS_PROJECT_PATH)/entry/libs/Redis_sdk/"
	@echo ""
	@echo "Next steps:"
	@echo "1. Add dependency in entry/oh-package.json5:"
	@echo '   "dependencies": {'
	@echo '     "libredis_ohos.so": "file:./libs/Redis_sdk"'
	@echo '   }'
	@echo "2. Import in your code:"
	@echo "   import { RedisClient } from 'libredis_ohos.so';"

