# dpino Packaging Files

This directory contains packaging configurations for all supported distribution formats.

## Directory Structure

```
packaging/
├── deb/          # Debian/APT package
│   └── DEBIAN/
│       └── control
├── snap/         # Snap package
│   └── snapcraft.yaml
├── flatpak/      # Flatpak package
│   └── dpino.json
├── aur/          # Arch Linux AUR
│   └── PKGBUILD
├── rpm/          # RPM package (Fedora, RHEL, etc.)
│   └── dpino.spec
├── homebrew/     # Homebrew formula
│   └── dpino.rb
└── README.md     # This file
```

## Quick Reference

### Build Individual Packages

```bash
# Debian
./scripts/build_deb.sh 0.1.0 amd64

# Snap
./scripts/build_snap.sh 0.1.0

# Flatpak
./scripts/build_flatpak.sh 0.1.0

# AppImage
./scripts/build_appimage.sh 0.1.0

# RPM
make rpm VERSION=0.1.0
```

### Build All Packages

```bash
./scripts/build_all.sh 0.1.0
```

### Update Version

```bash
./scripts/update_version.sh 0.1.0
```

## Package Formats

### 1. Debian (.deb)
- **File**: `deb/DEBIAN/control`
- **Target**: Ubuntu, Debian, Linux Mint, Pop!_OS
- **Build**: `./scripts/build_deb.sh <version> <arch>`

### 2. Snap
- **File**: `snap/snapcraft.yaml`
- **Target**: Any Linux with Snap
- **Build**: `./scripts/build_snap.sh <version>`
- **Publish**: https://snapcraft.io/

### 3. Flatpak
- **File**: `flatpak/dpino.json`
- **Target**: Fedora, Ubuntu, Arch, etc.
- **Build**: `./scripts/build_flatpak.sh <version>`
- **Publish**: https://flathub.org/

### 4. Arch Linux AUR
- **File**: `aur/PKGBUILD`
- **Target**: Arch, Manjaro, EndeavourOS
- **Build**: `cd aur && makepkg -si`
- **Publish**: https://aur.archlinux.org/

### 5. RPM
- **File**: `rpm/dpino.spec`
- **Target**: Fedora, RHEL, CentOS, openSUSE
- **Build**: `make rpm VERSION=<version>`
- **Publish**: Fedora COPR, openSUSE Build Service

### 6. AppImage
- **Target**: Portable Linux
- **Build**: `./scripts/build_appimage.sh <version>`
- **No installation required**

### 7. Homebrew
- **File**: `homebrew/dpino.rb`
- **Target**: macOS, Linux
- **Install**: `brew install abdou-da0wew/dpino/dpino`
- **Publish**: https://github.com/Homebrew/homebrew-core

### 8. Cargo/crates.io
- **File**: `Cargo.toml` (root)
- **Target**: Rust users
- **Publish**: Automated via GitHub Actions
- **Install**: `cargo install dpino`

## CI/CD

All packages are built automatically via GitHub Actions:

- **CI**: `.github/workflows/ci.yml` - Tests and builds
- **Build Packages**: `.github/workflows/build-packages.yml` - Creates all packages on releases
- **Publish crates.io**: `.github/workflows/publish-crates.yml` - Publishes to crates.io
- **Publish AUR**: `.github/workflows/publish-aur.yml` - Updates AUR package

## Version Management

To update the version across all packages:

```bash
./scripts/update_version.sh 0.1.0
```

This updates:
- `Cargo.toml`
- `deb/DEBIAN/control`
- `snap/snapcraft.yaml`
- `aur/PKGBUILD`
- `rpm/dpino.spec`
- `homebrew/dpino.rb`
- Man page

## Requirements

### Debian
- `dpkg-dev`

### Snap
- `snapcraft` (install via `snap install snapcraft --classic`)

### Flatpak
- `flatpak-builder`
- Runtime: `org.freedesktop.Platform//23.08`
- SDK: `org.freedesktop.Sdk//23.08`

### AUR
- `makepkg`
- `git`

### RPM
- `rpm-build`
- `rpmdevtools`

### AppImage
- `wget` (for downloading appimagetool)
- Internet connection

## Testing Packages

### Debian
```bash
sudo dpkg -i build/deb/dpino.deb
dpino --version
```

### Snap
```bash
sudo snap install build/snap/dpino_*.snap --dangerous
dpino --version
```

### Flatpak
```bash
flatpak-builder --install --user build-dir flatpak/dpino.json
flatpak run io.abdou.dpino --version
```

### AppImage
```bash
chmod +x build/appimage/dpino-*.AppImage
./build/appimage/dpino-*.AppImage --version
```

## Troubleshooting

### Snap build fails
- Ensure `snapcraft` is installed: `sudo snap install snapcraft --classic`
- Check snapcraft.yaml syntax: `snapcraft lint`

### Flatpak build fails
- Install runtime: `flatpak install org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08`
- Check manifest syntax: `flatpak-builder --show-deps build-dir dpino.json`

### RPM build fails
- Ensure `rpm-build` is installed
- Check spec file syntax: `rpmlint dpino.spec`

## Documentation

For detailed instructions, see:
- [Packaging Guide](../docs/PACKAGING.md)
- [Architecture](../docs/ARCHITECTURE.md)

