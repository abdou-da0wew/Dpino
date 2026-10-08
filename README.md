# Dpino

A Rust-powered package metadata extractor + desktop entry generator + installer + browser.

## Quick Start

### From Source

```bash
# Build
cargo build --release

# Run
./target/release/dpino --help
```

### Install from Package

dpino is available for multiple distributions:

- **Debian/Ubuntu**: `.deb` package (see [Packaging Guide](docs/PACKAGING.md))
- **Snap**: `snap install dpino`
- **Flatpak**: `flatpak install io.abdou.dpino`
- **Arch Linux**: `yay -S dpino` (AUR)
- **Homebrew**: `brew install abdou-da0wew/dpino/dpino`
- **Cargo**: `cargo install dpino`

## Features

- Scan directories for `.deb`, `.AppImage`, and AppDir packages
- Extract complete metadata (name, version, categories, icons, etc.)
- Generate Freedesktop-compliant `.desktop` files
- Install desktop entries and icons to XDG directories
- Full-screen TUI browser
- Complete CLI interface

## Documentation

- [User Guide](docs/README.md) - Full documentation
- [Architecture](docs/ARCHITECTURE.md) - Technical details
- [Packaging Guide](docs/PACKAGING.md) - Distribution packages
- [Security](docs/SECURITY.md) - Security considerations

## Building Packages

Build all distribution packages:

```bash
./scripts/build_all.sh 0.1.0
```

Or build individually:

```bash
make deb VERSION=0.1.0    # Debian package
make snap VERSION=0.1.0   # Snap package
make appimage VERSION=0.1.0  # AppImage
```

See [Packaging Guide](docs/PACKAGING.md) for details.

## License

MIT OR Apache-2.0

