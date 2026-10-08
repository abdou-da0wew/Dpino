#!/bin/bash
# Helper script to copy icons from assets to final structure
set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Copying icons to final structure..."

# Create icon directories
mkdir -p "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/"{16x16,22x22,24x24,32x32,48x48,64x64,128x128,256x256,512x512}"/apps"

# Copy icons
for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    if [ -f "${PROJECT_DIR}/assets/icons/${size}.png" ]; then
        cp "${PROJECT_DIR}/assets/icons/${size}.png" \
           "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png"
        echo "Copied ${size}.png"
    else
        echo "Warning: ${size}.png not found in assets/icons/"
    fi
done

echo "Icons copied successfully!"

