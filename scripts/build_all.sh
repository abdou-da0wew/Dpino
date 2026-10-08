#!/bin/bash
set -e

VERSION="${1:-0.1.0}"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Building all packages for dpino v${VERSION}"

# Build release binary first
cd "${PROJECT_DIR}"
cargo build --release

# Build all packages
echo "Building Debian package..."
"${PROJECT_DIR}/scripts/build_deb.sh" "${VERSION}"

else
fi

echo "Building Flatpak package..."
if command -v flatpak-builder &> /dev/null; then
    "${PROJECT_DIR}/scripts/build_flatpak.sh" "${VERSION}"
else
    echo "flatpak-builder not found, skipping Flatpak build"
fi

echo "Building AppImage..."
"${PROJECT_DIR}/scripts/build_appimage.sh" "${VERSION}"

echo "All packages built successfully!"
echo "Output directory: ${PROJECT_DIR}/build/"

