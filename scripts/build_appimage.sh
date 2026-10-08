#!/bin/bash
set -e

VERSION="${1:-0.1.0}"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${PROJECT_DIR}/build/appimage"
APPIMAGE_NAME="dpino-${VERSION}-x86_64.AppImage"

echo "Building AppImage for dpino v${VERSION}"

# Clean previous build
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}/AppDir"

# Build the binary
cd "${PROJECT_DIR}"
cargo build --release

# Create AppDir structure
mkdir -p "${BUILD_DIR}/AppDir/usr/bin"
mkdir -p "${BUILD_DIR}/AppDir/usr/share/applications"
mkdir -p "${BUILD_DIR}/AppDir/usr/share/icons/hicolor/"{16x16,22x22,24x24,32x32,48x48,64x64,128x128,256x256,512x512}"/apps"

# Copy files
cp "${PROJECT_DIR}/target/release/dpino" "${BUILD_DIR}/AppDir/usr/bin/dpino"
cp "${PROJECT_DIR}/final/dpino/usr/share/applications/dpino.desktop" \
   "${BUILD_DIR}/AppDir/usr/share/applications/dpino.desktop"

# Copy all icons
for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    if [ -f "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" ]; then
        cp "${PROJECT_DIR}/final/dpino/usr/share/icons/hicolor/${size}/apps/dpino.png" \
           "${BUILD_DIR}/AppDir/usr/share/icons/hicolor/${size}/apps/dpino.png"
    fi
done

# Create AppRun
cat > "${BUILD_DIR}/AppDir/AppRun" <<'EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
exec "${HERE}/usr/bin/dpino" "$@"
EOF
chmod +x "${BUILD_DIR}/AppDir/AppRun"

# Download appimagetool if not present
APPIMAGETOOL="${BUILD_DIR}/appimagetool.AppImage"
if [ ! -f "${APPIMAGETOOL}" ]; then
    wget -O "${APPIMAGETOOL}" \
        "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
    chmod +x "${APPIMAGETOOL}"
fi

# Build AppImage
cd "${BUILD_DIR}"
ARCH=x86_64 "${APPIMAGETOOL}" AppDir "${APPIMAGE_NAME}"

echo "AppImage built: ${BUILD_DIR}/${APPIMAGE_NAME}"

