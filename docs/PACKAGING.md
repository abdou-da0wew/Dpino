# dpino Packaging & Distribution Guide

This guide covers how to package and distribute `dpino` for all major Linux channels, including detailed setup instructions for each.

## Quick Start

Build all packages at once:

```bash
./scripts/build_all.sh 0.1.0
```

This will create packages in the `build/` directory:
- `build/deb/dpino.deb` - Debian package
- `build/appimage/*.AppImage` - AppImage
- `build/flatpak/build-dir/` - Flatpak build directory

## 1️⃣ APT / Debian (.deb)

**Target distros:** Ubuntu, Debian, Linux Mint, Pop!_OS

### Manual Build

```bash
./scripts/build_deb.sh 0.1.0 amd64
```

The package will be created at `build/deb/dpino.deb`.

### Install

```bash
sudo dpkg -i build/deb/dpino.deb
```

### Publish via Launchpad PPA

1. Create a Launchpad account and PPA
2. Upload the source package
3. Follow: https://help.launchpad.net/Packaging/PPA



### Prerequisites

```bash
```

### Build

```bash
```

### Test Locally

```bash
```



## 3️⃣ Flatpak

**Target distros:** Fedora, Ubuntu, Arch, etc.

### Prerequisites

```bash
flatpak install org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08
```

### Build

```bash
./scripts/build_flatpak.sh 0.1.0
```

### Test

```bash
flatpak run io.abdou.dpino
```

### Submit to Flathub

1. Fork https://github.com/flathub/flathub
2. Create a new app directory
3. Submit PR with manifest
4. Follow: https://github.com/flathub/flathub/wiki/App-Submission

## 4️⃣ Arch Linux / Pacman (AUR)

**Target distros:** Arch, Manjaro, EndeavourOS

### Build Locally

```bash
cd packaging/aur
makepkg -si
```

### Submit to AUR

1. Create AUR account
2. Clone empty AUR repo: `git clone ssh://aur@aur.archlinux.org/dpino.git`
3. Copy `PKGBUILD` and `.SRCINFO`
4. Push to AUR

The GitHub Actions workflow will automatically update AUR on releases.

## 5️⃣ RPM (.rpm)

**Target distros:** Fedora, RHEL, CentOS, openSUSE

### Build

```bash
VERSION=0.1.0
tar -czf dpino-${VERSION}.tar.gz --exclude='.git' --exclude='target' .
mkdir -p ~/rpmbuild/{SOURCES,SPECS,RPMS}
cp dpino-${VERSION}.tar.gz ~/rpmbuild/SOURCES/
cp packaging/rpm/dpino.spec ~/rpmbuild/SPECS/
rpmbuild -bb ~/rpmbuild/SPECS/dpino.spec
```

The RPM will be in `~/rpmbuild/RPMS/x86_64/`.

## 6️⃣ AppImage

**Portable Linux binary**

### Build

```bash
./scripts/build_appimage.sh 0.1.0
```

The AppImage will be at `build/appimage/dpino-0.1.0-x86_64.AppImage`.

### Usage

```bash
chmod +x dpino-0.1.0-x86_64.AppImage
./dpino-0.1.0-x86_64.AppImage
```

No installation required!

## 7️⃣ Homebrew (Linux + macOS)

### Formula Location

The formula is at `packaging/homebrew/dpino.rb`.

### Install from Tap

```bash
brew install abdou-da0wew/dpino/dpino
```

### Submit to Homebrew Core

1. Fork https://github.com/Homebrew/homebrew-core
2. Add formula to `Formula/`
3. Submit PR

## 8️⃣ Cargo / crates.io

**For Rust users only**

### Prerequisites

1. Create account at https://crates.io/
2. Get API token
3. Add to GitHub secrets as `CRATES_IO_TOKEN`

### Publish

The GitHub Actions workflow will automatically publish on releases.

Manual publish:

```bash
cargo publish --token YOUR_TOKEN
```

### Install

```bash
cargo install dpino
```

## CI/CD Integration

All packaging is automated via GitHub Actions:

- **CI**: Tests and builds on every push
- **Build Packages**: Creates all packages on tag releases
- **Publish crates.io**: Publishes to crates.io on releases
- **Publish AUR**: Updates AUR package on releases

### Workflows

- `.github/workflows/ci.yml` - Continuous integration
- `.github/workflows/build-packages.yml` - Package building
- `.github/workflows/publish-crates.yml` - crates.io publishing
- `.github/workflows/publish-aur.yml` - AUR updates

## Package Structure

All packages include:

```
dpino/
├─ usr/bin/dpino                    # Binary
├─ usr/share/applications/dpino.desktop  # Desktop entry
├─ usr/share/man/man1/dpino.1      # Man page
└─ usr/share/icons/hicolor/128x128/apps/dpino.png  # Icon (if available)
```

## Version Management

Update version in:

1. `Cargo.toml` - `version = "0.1.0"`
2. `packaging/deb/DEBIAN/control` - `Version: 0.1.0`
4. `packaging/aur/PKGBUILD` - `pkgver=0.1.0`
5. `packaging/rpm/dpino.spec` - `Version: 0.1.0`
6. `packaging/homebrew/dpino.rb` - URL and version

Then create a git tag:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

## References

- [Debian packaging guide](https://www.debian.org/doc/manuals/maint-guide/)
- [Launchpad PPA guide](https://help.launchpad.net/Packaging/PPA)
- [Flatpak docs](https://docs.flatpak.org/en/latest/)
- [Flathub app submission](https://github.com/flathub/flathub/wiki/App-Submission)
- [Arch Wiki AUR submission](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [Arch Wiki PKGBUILD](https://wiki.archlinux.org/title/PKGBUILD)
- [Fedora RPM packaging](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
- [AppImage packaging guide](https://docs.appimage.org/packaging-guide/index.html)
- [Homebrew formula docs](https://docs.brew.sh/Formula-Cookbook)
- [Publishing crates](https://doc.rust-lang.org/cargo/reference/publishing.html)

