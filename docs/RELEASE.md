# Release Process

This document describes the complete release process for dpino.

## Pre-Release Checklist

- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] Version is updated in all files
- [ ] Changelog is updated
- [ ] All features are implemented and tested

## Release Steps

### 1. Update Version

Use the version update script:

```bash
./scripts/update_version.sh 0.1.0
```

This updates:
- `Cargo.toml`
- All packaging files
- Man page

### 2. Review Changes

```bash
git diff
git status
```

### 3. Commit Version Bump

```bash
git add .
git commit -m "Bump version to 0.1.0"
```

### 4. Create Tag

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
```

### 5. Push to GitHub

```bash
git push origin main
git push origin v0.1.0
```

## Automated Release

Once the tag is pushed, GitHub Actions will automatically:

1. **Build Packages** (`.github/workflows/build-packages.yml`)
   - Build Debian package
   - Build Snap package
   - Build AppImage
   - Build RPM package
   - Create GitHub release with all artifacts

2. **Publish to crates.io** (`.github/workflows/publish-crates.yml`)
   - Automatically publishes to crates.io
   - Requires `CRATES_IO_TOKEN` secret

3. **Update AUR** (`.github/workflows/publish-aur.yml`)
   - Automatically updates AUR package
   - Requires `AUR_SSH_PRIVATE_KEY` secret

## Manual Release Steps

If you need to build packages manually:

### Build All Packages

```bash
./scripts/build_all.sh 0.1.0
```

### Individual Packages

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

## Publishing Packages

### Debian/Ubuntu

1. Build package: `./scripts/build_deb.sh 0.1.0 amd64`
2. Test: `sudo dpkg -i build/deb/dpino.deb`
3. Upload to Launchpad PPA (if applicable)

### Snap

1. Build: `./scripts/build_snap.sh 0.1.0`
2. Test: `sudo snap install build/snap/dpino_*.snap --dangerous`
3. Publish: `snapcraft upload --release=stable dpino_*.snap`

### Flatpak

1. Build: `./scripts/build_flatpak.sh 0.1.0`
2. Test: `flatpak run io.abdou.dpino`
3. Submit to Flathub (via PR)

### Arch Linux AUR

1. Update PKGBUILD version
2. Update .SRCINFO: `makepkg --printsrcinfo > .SRCINFO`
3. Commit and push to AUR

### RPM

1. Build: `make rpm VERSION=0.1.0`
2. Test: `sudo rpm -ivh ~/rpmbuild/RPMS/x86_64/dpino-*.rpm`
3. Upload to COPR or OBS

### AppImage

1. Build: `./scripts/build_appimage.sh 0.1.0`
2. Test: `./build/appimage/dpino-*.AppImage`
3. Upload to GitHub releases

### Homebrew

1. Update formula version and URL
2. Test: `brew install --build-from-source dpino.rb`
3. Submit PR to homebrew-core

### crates.io

1. Ensure `CRATES_IO_TOKEN` is set in GitHub secrets
2. Tag release (automated)
3. Or manually: `cargo publish --token $TOKEN`

## Post-Release

- [ ] Verify all packages are available
- [ ] Test installation from each channel
- [ ] Update website/documentation
- [ ] Announce release (if applicable)

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

Examples:
- `0.1.0` - Initial release
- `0.1.1` - Bug fix
- `0.2.0` - New features
- `1.0.0` - Stable release

## Troubleshooting

### GitHub Actions Failures

- Check workflow logs
- Verify secrets are set correctly
- Ensure all dependencies are available

### Package Build Failures

- Check build logs
- Verify all dependencies are installed
- Test build scripts locally

### Publishing Failures

- Verify credentials/tokens
- Check package format compliance
- Review platform-specific requirements

