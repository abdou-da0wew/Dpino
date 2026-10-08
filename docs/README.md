# dpino

A Rust-powered package metadata extractor + desktop entry generator + installer + browser.

## Overview

dpino scans directories for Debian packages (`.deb`), AppImages (`.AppImage`), and AppDir folders, extracts their metadata, generates Freedesktop-compliant `.desktop` files, and provides both CLI and TUI interfaces for managing packages.

## Features

- **Package Detection**: Automatically detects `.deb`, `.AppImage`, and AppDir packages
- **Metadata Extraction**: Extracts name, version, categories, maintainer, icon, exec command, description, and mimetypes
- **Desktop Entry Generation**: Creates valid Freedesktop `.desktop` files
- **Installation**: Installs desktop entries and icons to XDG directories
- **TUI Browser**: Full-screen terminal UI for browsing and managing packages
- **CLI Interface**: Complete command-line interface for all operations

## Installation

```bash
cargo build --release
sudo cp target/release/dpino /usr/local/bin/
```

## Usage

### CLI Commands

```bash
# Scan a directory for packages
dpino scan --path /path/to/packages [--json]

# Inspect a package file
dpino inspect /path/to/package.deb

# List all cached packages
dpino list [--json]

# Extract a package
dpino extract /path/to/package.deb --out /tmp/extracted

# Install desktop entry
dpino install-desktop /path/to/package.deb [--user] [--dry-run]

# Uninstall desktop entry
dpino uninstall myapp.desktop [--user]

# Refresh desktop database
dpino refresh

# Launch TUI browser
dpino browse [--path /path/to/scan]
```

### TUI Keybindings

- `↑/↓`: Navigate package list
- `Enter`: Inspect selected package
- `i`: Install desktop entry
- `e`: Extract package
- `d`: Delete from cache
- `r`: Refresh
- `q`: Quit

## Package Format Support

### Debian Packages (.deb)

- Extracts control.tar.* and data.tar.*
- Parses control file for metadata
- Extracts desktop entries and icons from package
- Auto-generates desktop entries if not present

### AppImage

- Uses `--appimage-extract` to extract contents
- Reads desktop entries from extracted squashfs-root
- Extracts icons
- Wraps Exec line with absolute path to AppImage

### AppDir

- Reads AppRun executable
- Reads included `.desktop` files
- Extracts icons from usr/share/icons or local icons

## Security

- Never runs extracted binaries unless explicitly requested
- Sanitizes all filenames
- No absolute overwrites without confirmation
- Never writes outside XDG directories unless running as root
- Uses temporary directories for extraction
- Provides warnings when external tools are missing

## Dependencies

- `clap`: CLI argument parsing
- `walkdir`: Directory traversal
- `ar`, `tar`, `xz2`, `flate2`: Debian package extraction
- `ratatui`, `crossterm`: TUI interface
- `serde`, `serde_json`: Serialization
- `thiserror`, `anyhow`: Error handling
- `tempfile`: Temporary file handling
- `log`, `env_logger`: Logging

## References

- [Debian Package Format](https://www.debian.org/doc/debian-policy/ch-controlfields.html)
- [AppImage Specification](https://docs.appimage.org/)
- [Freedesktop Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- [ratatui Documentation](https://docs.rs/ratatui/)

## License

MIT OR Apache-2.0

