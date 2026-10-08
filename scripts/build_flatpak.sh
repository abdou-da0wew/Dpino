#!/bin/bash
set -e

VERSION="${1:-0.1.0}"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${PROJECT_DIR}/build/flatpak"

echo "Building Flatpak package for dpino v${VERSION}"

# Clean previous build
rm -rf "${BUILD_DIR}"

# Build the binary
cd "${PROJECT_DIR}"
cargo build --release

# Create flatpak directory
mkdir -p "${BUILD_DIR}"

# Copy manifest
cp "${PROJECT_DIR}/packaging/flatpak/dpino.json" "${BUILD_DIR}/dpino.json"

# Copy binary
mkdir -p "${BUILD_DIR}/target/release"
cp "${PROJECT_DIR}/target/release/dpino" "${BUILD_DIR}/target/release/dpino"

# Copy desktop file and icons for flatpak build
mkdir -p "${BUILD_DIR}/final/dpino/usr/share/applications"
mkdir -p "${BUILD_DIR}/final/dpino/usr/share/icons/hicolor/"{16x16,22x22,24x24,32x32,48x48,64x64,128x128,256x256,512x512}"/apps"
cp "${PROJECT_DIR}/final/dpino/usr/share/applications/dpino.desktop" \
   "${BUILD_DIR}/final/dpino/usr/share/applications/dpino.desktop"
for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    if [ -f "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" ]; then
        cp "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" \
           "${BUILD_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png"
    fi
done

# Build flatpak
cd "${BUILD_DIR}"
flatpak-builder --install --user build-dir dpino.json

echo "Flatpak package built in ${BUILD_DIR}/build-dir"

