use crate::cache::PackageMetadata;
use crate::desktop::{DesktopEntry, generate_desktop_id, install_desktop_entry, uninstall_desktop_entry};
use crate::formats;
use crate::utils;
use anyhow::{Context, Result};
use std::path::Path;

pub struct Installer;

impl Installer {
    pub fn install_desktop(
        metadata: &PackageMetadata,
        user: bool,
        dry_run: bool,
    ) -> Result<String> {
        let desktop_entry = if let Some(ref content) = metadata.desktop_entry {
            DesktopEntry::parse(content)
                .context("Failed to parse existing desktop entry")?
        } else {
            DesktopEntry::from_metadata(metadata)
        };

        let desktop_id = generate_desktop_id(&metadata.name);
        
        if let Some(ref icon_path) = metadata.icon_path {
            if !dry_run {
                Self::install_icon(icon_path, &metadata.name, user)?;
            } else {
                println!("[DRY RUN] Would install icon from: {}", icon_path.display());
            }
        }

        install_desktop_entry(&desktop_entry, &desktop_id, user, dry_run)?;

        if !dry_run {
            Self::refresh_desktop_database(user)?;
        } else {
            println!("[DRY RUN] Would refresh desktop database");
        }

        Ok(desktop_id)
    }

    pub fn uninstall(desktop_id: &str, user: bool) -> Result<()> {
        uninstall_desktop_entry(desktop_id, user)?;
        Self::refresh_desktop_database(user)?;
        Ok(())
    }

    fn install_icon(icon_path: &Path, app_name: &str, user: bool) -> Result<()> {
        let icons_dir = if user {
            utils::get_xdg_icons_dir()?.join("hicolor").join("256x256").join("apps")
        } else {
            Path::new("/usr/share/icons/hicolor/256x256/apps").to_path_buf()
        };

        utils::ensure_dir(&icons_dir)?;

        let icon_name = icon_path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}.png", utils::sanitize_filename(app_name)));

        let dest_path = icons_dir.join(&icon_name);

        std::fs::copy(icon_path, &dest_path)
            .with_context(|| format!("Failed to copy icon to {}", dest_path.display()))?;

        Ok(())
    }

    fn refresh_desktop_database(user: bool) -> Result<()> {
        if user {
            let applications_dir = utils::get_xdg_applications_dir()?;
            let output = std::process::Command::new("update-desktop-database")
                .arg(&applications_dir)
                .output()
                .context("Failed to run update-desktop-database")?;

            if !output.status.success() {
                log::warn!("update-desktop-database failed: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
        } else {
            let output = std::process::Command::new("update-desktop-database")
                .arg("/usr/share/applications")
                .output()
                .context("Failed to run update-desktop-database")?;

            if !output.status.success() {
                log::warn!("update-desktop-database failed: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
        }

        Ok(())
    }

    pub fn extract_package(package_path: &Path, out_dir: &Path) -> Result<()> {
        if let Some(format) = formats::detect_format(package_path) {
            format.extract_to(package_path, out_dir)
                .with_context(|| format!("Failed to extract package: {}", package_path.display()))
        } else {
            anyhow::bail!("Unknown package format: {}", package_path.display())
        }
    }
}

