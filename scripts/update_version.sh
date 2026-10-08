#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

VERSION="$1"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Updating version to ${VERSION} in all files..."

# Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" "${PROJECT_DIR}/Cargo.toml"

# Update DEB control
sed -i "s/^Version:.*/Version: ${VERSION}/" "${PROJECT_DIR}/packaging/deb/DEBIAN/control"

# Update snapcraft.yaml
sed -i "s/^version:.*/version: '${VERSION}'/" "${PROJECT_DIR}/packaging/snap/snapcraft.yaml"

# Update PKGBUILD
sed -i "s/^pkgver=.*/pkgver=${VERSION}/" "${PROJECT_DIR}/packaging/aur/PKGBUILD"
sed -i "s/^pkgrel=.*/pkgrel=1/" "${PROJECT_DIR}/packaging/aur/PKGBUILD"

# Update RPM spec
sed -i "s/^Version:.*/Version: ${VERSION}/" "${PROJECT_DIR}/packaging/rpm/dpino.spec"
sed -i "s/^Release:.*/Release: 1%{?dist}/" "${PROJECT_DIR}/packaging/rpm/dpino.spec"

# Update Homebrew formula
sed -i "s|url \".*\"|url \"https://github.com/abdou-da0wew/dpino/archive/refs/tags/v${VERSION}.tar.gz\"|" "${PROJECT_DIR}/packaging/homebrew/dpino.rb"

# Update man page
sed -i "s/\.TH DPINO 1 \".*\" \".*\" \".*\"/\.TH DPINO 1 \"$(date +'%B %Y')\" \"v${VERSION}\" \"dpino manual\"/" "${PROJECT_DIR}/final/dpino/usr/share/man/man1/dpino.1"

echo "Version updated to ${VERSION} in all files!"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Commit: git commit -am \"Bump version to ${VERSION}\""
echo "  3. Tag: git tag -a v${VERSION} -m \"Release v${VERSION}\""
echo "  4. Push: git push origin main && git push origin v${VERSION}"

