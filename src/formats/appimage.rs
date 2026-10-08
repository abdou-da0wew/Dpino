use crate::cache::PackageMetadata;
use crate::desktop::DesktopEntry;
use crate::utils;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

pub struct AppImagePackage;

impl AppImagePackage {
    pub fn detect(path: &Path) -> bool {
        if path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("AppImage"))
            .unwrap_or(false) {
            return utils::is_executable(path);
        }
        false
    }

    pub fn extract_metadata(path: &Path) -> Result<PackageMetadata> {
        let abs_path = path.canonicalize()
            .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;

        let temp_dir = tempfile::tempdir()
            .context("Failed to create temporary directory")?;

        let extract_dir = temp_dir.path().join("squashfs-root");

        let output = Command::new(&abs_path)
            .arg("--appimage-extract")
            .current_dir(temp_dir.path())
            .output()
            .context("Failed to run AppImage extract command")?;

        if !output.status.success() {
            anyhow::bail!("AppImage extraction failed: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        let mut metadata = PackageMetadata {
            source_path: abs_path.clone(),
            package_type: "AppImage".to_string(),
            name: abs_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ..Default::default()
        };

        let desktop_files: Vec<PathBuf> = walkdir::WalkDir::new(&extract_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "desktop")
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        if let Some(desktop_path) = desktop_files.first() {
            if let Ok(content) = fs::read_to_string(desktop_path) {
                metadata.desktop_entry = Some(content.clone());
                if let Ok(entry) = DesktopEntry::parse(&content) {
                    metadata.name = entry.name;
                    metadata.description = entry.comment;
                    metadata.categories = entry.categories;
                    metadata.mimetypes = entry.mime_types;
                    metadata.exec_command = Some(format!("{} %U", abs_path.display()));
                    
                    if let Some(icon) = entry.icon {
                        let icon_path = extract_dir.join(&icon);
                        if icon_path.exists() {
                            metadata.icon_path = Some(icon_path);
                        } else {
                            let icon_path = extract_dir.join("usr/share/icons").join(&icon);
                            if icon_path.exists() {
                                metadata.icon_path = Some(icon_path);
                            }
                        }
                    }
                }
            }
        }

        if metadata.icon_path.is_none() {
            let icon_dirs = vec![
                extract_dir.join(".DirIcon"),
                extract_dir.join("usr/share/icons"),
            ];

            for icon_dir in icon_dirs {
                if icon_dir.is_file() {
                    metadata.icon_path = Some(icon_dir);
                    break;
                } else if icon_dir.is_dir() {
                    if let Ok(entries) = fs::read_dir(&icon_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if let Some(ext) = path.extension() {
                                if ext == "png" || ext == "svg" || ext == "xpm" {
                                    metadata.icon_path = Some(path);
                                    break;
                                }
                            }
                        }
                    }
                }
                if metadata.icon_path.is_some() {
                    break;
                }
            }
        }

        if metadata.exec_command.is_none() {
            metadata.exec_command = Some(format!("{} %U", abs_path.display()));
        }

        Ok(metadata)
    }

    pub fn extract_to(path: &Path, out_dir: &Path) -> Result<()> {
        let abs_path = path.canonicalize()
            .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;

        let temp_dir = tempfile::tempdir()
            .context("Failed to create temporary directory")?;

        let extract_dir = temp_dir.path().join("squashfs-root");

        let output = Command::new(&abs_path)
            .arg("--appimage-extract")
            .current_dir(temp_dir.path())
            .output()
            .context("Failed to run AppImage extract command")?;

        if !output.status.success() {
            anyhow::bail!("AppImage extraction failed: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        utils::ensure_dir(out_dir)?;

        for entry in walkdir::WalkDir::new(&extract_dir) {
            let entry = entry?;
            let src_path = entry.path();
            let rel_path = src_path.strip_prefix(&extract_dir)?;
            let dst_path = out_dir.join(rel_path);

            if src_path.is_dir() {
                fs::create_dir_all(&dst_path)?;
            } else if src_path.is_file() {
                if let Some(parent) = dst_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}

impl crate::formats::PackageFormat for AppImagePackage {
    fn detect(path: &std::path::Path) -> bool
    where
        Self: Sized,
    {
        Self::detect(path)
    }

    fn extract_metadata(&self, path: &std::path::Path) -> Result<PackageMetadata> {
        Self::extract_metadata(path)
    }

    fn extract_to(&self, path: &std::path::Path, out_dir: &std::path::Path) -> Result<()> {
        Self::extract_to(path, out_dir)
    }
}

