use crate::cache::PackageMetadata;
use crate::desktop::DesktopEntry;
use crate::utils;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;

pub struct DebPackage;

impl DebPackage {
    pub fn detect(path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("deb"))
            .unwrap_or(false)
    }

    pub fn extract_metadata(path: &Path) -> Result<PackageMetadata> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open DEB file: {}", path.display()))?;
        let mut archive = ar::Archive::new(BufReader::new(file));

        let mut control_tar: Option<Vec<u8>> = None;
        let mut data_tar: Option<Vec<u8>> = None;

        while let Some(entry_result) = archive.next_entry() {
            let mut entry = entry_result
                .context("Failed to read archive entry")?;
            
            let name = entry.header().identifier();
            let name_str = std::str::from_utf8(name)
                .unwrap_or("")
                .trim_end_matches('\0')
                .to_string();

            let mut data = Vec::new();
            entry.read_to_end(&mut data)
                .context("Failed to read archive entry data")?;

            if name_str.starts_with("control.tar") {
                control_tar = Some(data);
            } else if name_str.starts_with("data.tar") {
                data_tar = Some(data);
            }
        }

        let temp_dir = tempfile::tempdir()
            .context("Failed to create temporary directory")?;

        let mut metadata = PackageMetadata {
            source_path: path.to_path_buf(),
            package_type: "deb".to_string(),
            ..Default::default()
        };

        if let Some(control_data) = control_tar {
            Self::extract_control(&control_data, &temp_dir, &mut metadata)?;
        }

        if let Some(data_data) = data_tar {
            Self::extract_data(&data_data, &temp_dir, &mut metadata)?;
        }

        Ok(metadata)
    }

    fn extract_control(data: &[u8], _temp_dir: &TempDir, metadata: &mut PackageMetadata) -> Result<()> {
        let tar = Self::decompress_tar(data)?;
        let mut archive = tar::Archive::new(tar);

        for entry_result in archive.entries()? {
            let mut entry = entry_result?;
            let path = entry.path()?.to_path_buf();
            let path_str = path.to_string_lossy();

            if path_str == "./control" || path_str == "control" {
                let mut content = String::new();
                entry.read_to_string(&mut content)?;
                Self::parse_control(&content, metadata)?;
            }
        }

        Ok(())
    }

    fn extract_data(data: &[u8], _temp_dir: &TempDir, metadata: &mut PackageMetadata) -> Result<()> {
        let tar = Self::decompress_tar(data)?;
        let mut archive = tar::Archive::new(tar);

        let mut desktop_content: Option<String> = None;
        let mut icon_path: Option<PathBuf> = None;

        for entry_result in archive.entries()? {
            let entry = entry_result?;
            let path = entry.path()?.to_path_buf();
            let path_str = path.to_string_lossy();

            if path_str.contains("usr/share/applications") && path_str.ends_with(".desktop") {
                let mut content = String::new();
                let mut entry_reader = entry;
                entry_reader.read_to_string(&mut content)?;
                desktop_content = Some(content);
            }

            if path_str.contains("usr/share/icons") && 
               (path_str.ends_with(".png") || path_str.ends_with(".svg") || path_str.ends_with(".xpm")) {
                let icon_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string());
                if icon_name.is_some() {
                    icon_path = Some(PathBuf::from(path_str.trim_start_matches("./")));
                }
            }
        }

        if let Some(content) = desktop_content {
            metadata.desktop_entry = Some(content.clone());
            if let Ok(entry) = DesktopEntry::parse(&content) {
                if metadata.exec_command.is_none() {
                    metadata.exec_command = Some(entry.exec);
                }
                if metadata.icon_path.is_none() {
                    if let Some(icon) = entry.icon {
                        metadata.icon_path = Some(PathBuf::from(icon));
                    }
                }
            }
        }

        if icon_path.is_some() && metadata.icon_path.is_none() {
            metadata.icon_path = icon_path;
        }

        Ok(())
    }

    fn decompress_tar(data: &[u8]) -> Result<Box<dyn Read + '_>> {
        if data.starts_with(b"\x1f\x8b") {
            Ok(Box::new(GzDecoder::new(data)))
        } else if data.starts_with(b"\xfd7zXZ") {
            Ok(Box::new(XzDecoder::new(data)))
        } else {
            Ok(Box::new(Cursor::new(data)))
        }
    }

    fn parse_control(content: &str, metadata: &mut PackageMetadata) -> Result<()> {
        let mut current_field = String::new();
        let mut current_value = String::new();

        for line in content.lines() {
            if line.starts_with(' ') || line.starts_with('\t') {
                current_value.push_str(line.trim_start());
                current_value.push(' ');
            } else if let Some((key, value)) = line.split_once(':') {
                if !current_field.is_empty() {
                    Self::set_control_field(&current_field, &current_value.trim(), metadata);
                }
                current_field = key.trim().to_string();
                current_value = value.trim().to_string();
            }
        }

        if !current_field.is_empty() {
            Self::set_control_field(&current_field, &current_value.trim(), metadata);
        }

        Ok(())
    }

    fn set_control_field(field: &str, value: &str, metadata: &mut PackageMetadata) {
        match field {
            "Package" => metadata.name = value.to_string(),
            "Version" => metadata.version = value.to_string(),
            "Maintainer" => metadata.maintainer = value.to_string(),
            "Description" => {
                let desc = value.lines().next().unwrap_or(value);
                metadata.description = desc.to_string();
            }
            "Section" => {
                let category = Self::section_to_category(value);
                if !category.is_empty() {
                    metadata.categories.push(category);
                }
            }
            _ => {}
        }
    }

    fn section_to_category(section: &str) -> String {
        match section.to_lowercase().as_str() {
            "admin" | "system" => "System",
            "games" => "Game",
            "graphics" => "Graphics",
            "network" => "Network",
            "office" => "Office",
            "science" => "Science",
            "sound" | "audio" => "AudioVideo",
            "utils" | "utilities" => "Utility",
            "video" => "AudioVideo",
            "web" | "www" => "Network",
            _ => "Application",
        }.to_string()
    }

    pub fn extract_to(path: &Path, out_dir: &Path) -> Result<()> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open DEB file: {}", path.display()))?;
        let mut archive = ar::Archive::new(BufReader::new(file));

        utils::ensure_dir(out_dir)?;

        while let Some(entry_result) = archive.next_entry() {
            let mut entry = entry_result
                .context("Failed to read archive entry")?;
            
            let name = entry.header().identifier();
            let name_str = std::str::from_utf8(name)
                .unwrap_or("")
                .trim_end_matches('\0');

            if name_str.starts_with("data.tar") {
                let mut data = Vec::new();
                entry.read_to_end(&mut data)
                    .context("Failed to read data.tar")?;

                let tar = Self::decompress_tar(&data)?;
                let mut archive = tar::Archive::new(tar);
                archive.unpack(out_dir)
                    .context("Failed to unpack data.tar")?;
            }
        }

        Ok(())
    }
}

impl crate::formats::PackageFormat for DebPackage {
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

