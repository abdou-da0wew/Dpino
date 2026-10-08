use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context, Result};

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

pub fn get_xdg_data_home() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".local/share"))
}

pub fn get_xdg_applications_dir() -> Result<PathBuf> {
    Ok(get_xdg_data_home()?.join("applications"))
}

pub fn get_xdg_icons_dir() -> Result<PathBuf> {
    Ok(get_xdg_data_home()?.join("icons"))
}

pub fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode() & 0o111 != 0
        }
        #[cfg(not(unix))]
        {
            false
        }
    } else {
        false
    }
}

pub fn read_file_safe(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

pub fn write_file_safe(path: &Path, contents: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    fs::write(path, contents)
        .with_context(|| format!("Failed to write file: {}", path.display()))
}

