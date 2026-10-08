# Icon Integration Guide

This document describes how icons are integrated into the dpino project and all packaging formats.

## Icon Structure

Icons are stored in two locations:

1. **Source Icons**: `assets/icons/`
   - All icon sizes (16x16 through 512x512)
   - PRIMARY.png (primary icon reference)

2. **Final Structure**: `final/dpino/usr/share/icons/hicolor/`
   - Organized by size in Freedesktop standard structure
   - Used by all packaging formats

## Icon Sizes

The following icon sizes are included:

- **16x16** - Small system icons
- **22x22** - Panel icons
- **24x24** - Small toolbars
- **32x32** - Standard icons
- **48x48** - Large icons
- **64x64** - Very large icons
- **128x128** - Application icons (PRIMARY)
- **256x256** - High-res icons
- **512x512** - Ultra high-res (for modern displays)

## Copying Icons

Use the helper script to copy icons from assets to final structure:

```bash
./scripts/copy_icons.sh
```

This script:
- Creates all necessary directory structures
- Copies all icon sizes from `assets/icons/` to `final/dpino/usr/share/icons/hicolor/`
- Verifies each icon is copied successfully

## Packaging Integration

Icons are automatically included in all package formats:

### Debian (.deb)
- Icons copied in `scripts/build_deb.sh`
- All sizes included in package structure
- Installed to `/usr/share/icons/hicolor/*/apps/`

### Snap
- Icons included via `snapcraft.yaml`
- Organized in `share/icons/hicolor/*/apps/`
- All sizes included

### Flatpak
- Icons installed via `dpino.json` manifest
- All sizes included in build commands
- Installed to `/app/share/icons/hicolor/*/apps/`

### AppImage
- Icons copied in `scripts/build_appimage.sh`
- All sizes included in AppDir structure
- Standard Freedesktop icon locations

### Arch Linux AUR
- Icons installed via `PKGBUILD`
- All sizes included in package function
- Installed to `/usr/share/icons/hicolor/*/apps/`

### RPM
- Icons listed in `dpino.spec` %files section
- All sizes included
- Installed to `/usr/share/icons/hicolor/*/apps/`

## Desktop Entry

The desktop entry file (`dpino.desktop`) references the icon:

```ini
Icon=dpino
```

This references the icon name without extension, which the desktop environment will resolve to the appropriate size from the hicolor theme.

## CI/CD Integration

GitHub Actions workflows automatically copy icons before building packages:

- Icons are copied using `scripts/copy_icons.sh`
- This happens before each package build
- Ensures icons are always included in releases

## Manual Icon Updates

To update icons:

1. Replace icons in `assets/icons/` with new versions
2. Run `./scripts/copy_icons.sh` to update final structure
3. Rebuild packages

## Icon Naming Convention

- Source icons: `{size}.png` (e.g., `128x128.png`)
- Final icons: `dpino.png` in each size directory
- Desktop entry: References `dpino` (without extension)

## Verification

To verify icons are properly installed:

```bash
# Check source icons
ls -la assets/icons/*.png

# Check final structure
find final/dpino/usr/share/icons -name "dpino.png"

# Check installed icons (after package installation)
find /usr/share/icons/hicolor -name "dpino.png"
```

## Troubleshooting

### Icons not showing in desktop environment

1. Verify icons are in correct location
2. Run `gtk-update-icon-cache` or `update-icon-caches`
3. Check desktop entry Icon field matches icon name

### Missing icon sizes

1. Ensure all sizes exist in `assets/icons/`
2. Run `./scripts/copy_icons.sh`
3. Rebuild packages

### Icon not found in package

1. Check build script includes icon copying
2. Verify icon paths in packaging files
3. Check package contents: `dpkg -L dpino` (for .deb)

