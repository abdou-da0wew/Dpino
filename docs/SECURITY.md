# Security Considerations

## File Operations

- All filenames are sanitized using `utils::sanitize_filename()`
- Path traversal is prevented by using `PathBuf::join()` and canonicalization
- No absolute path overwrites without explicit confirmation

## Extraction

- All extractions use temporary directories via `tempfile`
- Extracted files are never executed unless explicitly requested with `--force-run` (not implemented in initial version)
- Temporary directories are cleaned up automatically

## Installation

- Desktop entries and icons are only installed to XDG directories:
  - User: `~/.local/share/applications` and `~/.local/share/icons`
  - System: `/usr/share/applications` and `/usr/share/icons`
- No writes outside these directories unless running as root

## External Tools

- AppImage extraction requires the AppImage to be executable
- Desktop database refresh uses system `update-desktop-database` command
- Warnings are provided when external tools are missing

## Input Validation

- All user input is validated before use
- Package format detection prevents processing of invalid files
- JSON parsing is safe and handles malformed input gracefully

## Recommendations

- Run as non-root user when possible
- Review extracted packages before installation
- Use `--dry-run` flag to preview changes
- Keep dependencies up to date

