# Architecture

## Overview

dpino is structured as a modular Rust application with clear separation of concerns:

## Module Structure

### Core Modules

- **`main.rs`**: Entry point, CLI parsing, command dispatch
- **`cli.rs`**: Clap command definitions
- **`scanner.rs`**: Directory scanning and package discovery
- **`cache.rs`**: Package metadata caching
- **`desktop.rs`**: Desktop entry generation and parsing
- **`installer.rs`**: Desktop entry and icon installation
- **`tui.rs`**: Terminal user interface
- **`utils.rs`**: Utility functions (path handling, XDG directories, etc.)

### Format Modules (`formats/`)

- **`mod.rs`**: Format detection and trait definitions
- **`deb.rs`**: Debian package parser
- **`appimage.rs`**: AppImage parser
- **`appdir.rs`**: AppDir parser

## Data Flow

1. **Scanning**: `scanner.rs` uses `walkdir` to traverse directories, `formats::detect_format()` to identify package types
2. **Extraction**: Format-specific parsers extract metadata into `PackageMetadata`
3. **Caching**: Metadata is stored in JSON cache via `PackageCache`
4. **Generation**: `desktop.rs` generates `.desktop` files from metadata
5. **Installation**: `installer.rs` copies files to XDG directories and refreshes databases

## Error Handling

- Uses `anyhow::Result` for application-level errors
- Uses `thiserror` for structured error types (where needed)
- All I/O operations are wrapped with context

## Security Considerations

- All file operations use safe path handling
- Filenames are sanitized before use
- Temporary directories are used for extraction
- No execution of extracted binaries
- XDG directory enforcement

