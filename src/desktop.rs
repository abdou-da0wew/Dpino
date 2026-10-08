use std::path::PathBuf;
use crate::cache::PackageMetadata;
use crate::utils;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub comment: String,
    pub categories: Vec<String>,
    pub mime_types: Vec<String>,
    pub terminal: bool,
    pub type_: String,
}

impl DesktopEntry {
    pub fn from_metadata(metadata: &PackageMetadata) -> Self {
        let exec = metadata.exec_command.clone().unwrap_or_else(|| {
            format!("{}", metadata.source_path.display())
        });

        let icon = metadata.icon_path.as_ref()
            .and_then(|p| p.to_str().map(|s| s.to_string()));

        Self {
            name: metadata.name.clone(),
            exec,
            icon,
            comment: metadata.description.clone(),
            categories: metadata.categories.clone(),
            mime_types: metadata.mimetypes.clone(),
            terminal: false,
            type_: "Application".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut lines = vec![
            "[Desktop Entry]".to_string(),
            format!("Type={}", self.type_),
            format!("Name={}", escape_desktop_string(&self.name)),
            format!("Exec={}", escape_desktop_string(&self.exec)),
        ];

        if let Some(ref icon) = self.icon {
            lines.push(format!("Icon={}", escape_desktop_string(icon)));
        }

        if !self.comment.is_empty() {
            lines.push(format!("Comment={}", escape_desktop_string(&self.comment)));
        }

        if !self.categories.is_empty() {
            lines.push(format!("Categories={}", self.categories.join(";")));
        }

        if !self.mime_types.is_empty() {
            lines.push(format!("MimeType={}", self.mime_types.join(";")));
        }

        if self.terminal {
            lines.push("Terminal=true".to_string());
        }

        lines.push("".to_string());
        lines.join("\n")
    }

    pub fn parse(content: &str) -> Result<Self> {
        let mut name = String::new();
        let mut exec = String::new();
        let mut icon = None;
        let mut comment = String::new();
        let mut categories = Vec::new();
        let mut mime_types = Vec::new();
        let mut terminal = false;
        let mut type_ = "Application".to_string();

        let mut in_desktop_entry = false;

        for line in content.lines() {
            let line = line.trim();
            
            if line == "[Desktop Entry]" {
                in_desktop_entry = true;
                continue;
            }

            if !in_desktop_entry {
                continue;
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = unescape_desktop_string(value.trim());

                match key {
                    "Name" => name = value,
                    "Exec" => exec = value,
                    "Icon" => icon = Some(value),
                    "Comment" => comment = value,
                    "Categories" => {
                        categories = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "MimeType" => {
                        mime_types = value.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "Terminal" => terminal = value.parse().unwrap_or(false),
                    "Type" => type_ = value,
                    _ => {}
                }
            }
        }

        Ok(Self {
            name: if name.is_empty() { "Unknown".to_string() } else { name },
            exec: if exec.is_empty() { "/bin/false".to_string() } else { exec },
            icon,
            comment,
            categories,
            mime_types,
            terminal,
            type_: if type_.is_empty() { "Application".to_string() } else { type_ },
        })
    }
}

fn escape_desktop_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('"', "\\\"")
}

fn unescape_desktop_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('"') => result.push('"'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

pub fn generate_desktop_id(name: &str) -> String {
    let sanitized = utils::sanitize_filename(name)
        .to_lowercase()
        .replace(' ', "-")
        .replace('_', "-");
    format!("{}.desktop", sanitized)
}

pub fn install_desktop_entry(
    entry: &DesktopEntry,
    desktop_id: &str,
    user: bool,
    dry_run: bool,
) -> Result<PathBuf> {
    let applications_dir = if user {
        utils::get_xdg_applications_dir()?
    } else {
        PathBuf::from("/usr/share/applications")
    };

    let desktop_path = applications_dir.join(desktop_id);
    let content = entry.to_string();

    if dry_run {
        println!("[DRY RUN] Would write desktop entry to: {}", desktop_path.display());
        println!("[DRY RUN] Content:\n{}", content);
        return Ok(desktop_path);
    }

    utils::ensure_dir(&applications_dir)?;
    utils::write_file_safe(&desktop_path, content.as_bytes())?;

    Ok(desktop_path)
}

pub fn uninstall_desktop_entry(desktop_id: &str, user: bool) -> Result<()> {
    let applications_dir = if user {
        utils::get_xdg_applications_dir()?
    } else {
        PathBuf::from("/usr/share/applications")
    };

    let desktop_path = applications_dir.join(desktop_id);

    if !desktop_path.exists() {
        anyhow::bail!("Desktop entry not found: {}", desktop_path.display());
    }

    std::fs::remove_file(&desktop_path)
        .with_context(|| format!("Failed to remove desktop entry: {}", desktop_path.display()))?;

    Ok(())
}

