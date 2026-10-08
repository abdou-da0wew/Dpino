#!/bin/bash
set -e

VERSION="${1:-0.1.0}"
ARCH="${2:-amd64}"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${PROJECT_DIR}/build/deb"
PACKAGE_DIR="${BUILD_DIR}/dpino"

echo "Building Debian package for dpino v${VERSION} (${ARCH})"

# Clean previous build
rm -rf "${BUILD_DIR}"

# Create package structure
mkdir -p "${PACKAGE_DIR}/DEBIAN"
mkdir -p "${PACKAGE_DIR}/usr/bin"
mkdir -p "${PACKAGE_DIR}/usr/share/applications"
mkdir -p "${PACKAGE_DIR}/usr/share/man/man1"
mkdir -p "${PACKAGE_DIR}/usr/share/icons/hicolor/"{16x16,22x22,24x24,32x32,48x48,64x64,128x128,256x256,512x512}"/apps"

# Build the binary
cd "${PROJECT_DIR}"
cargo build --release

# Copy files
cp "${PROJECT_DIR}/target/release/dpino" "${PACKAGE_DIR}/usr/bin/dpino"
cp "${PROJECT_DIR}/final/dpino/usr/share/applications/dpino.desktop" \
   "${PACKAGE_DIR}/usr/share/applications/dpino.desktop"
cp "${PROJECT_DIR}/final/dpino/usr/share/man/man1/dpino.1" \
   "${PACKAGE_DIR}/usr/share/man/man1/dpino.1"

# Copy all icons
for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    if [ -f "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" ]; then
        cp "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" \
           "${PACKAGE_DIR}/usr/share/icons/hicolor/${size}/apps/dpino.png"
    fi
done

# Create control file
cat > "${PACKAGE_DIR}/DEBIAN/control" <<EOF
Package: dpino
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libc6 (>= 2.31)
Maintainer: Abdou <aabdou911aydev@gmail.com>
Description: Rust-powered package metadata extractor + desktop entry installer + browser
 dpino scans directories for Debian packages (.deb), AppImages (.AppImage), and
 AppDir packages. It can extract metadata, generate Freedesktop-compliant desktop
 entries, install them to XDG directories, and provide an interactive TUI browser
 for package management.
EOF

# Build the package
cd "${BUILD_DIR}"
dpkg-deb --build dpino

echo "Package built: ${BUILD_DIR}/dpino.deb"

