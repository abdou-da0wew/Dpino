use crate::cache::{PackageCache, PackageMetadata};
use crate::formats;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Scanner {
    cache: PackageCache,
}

impl Scanner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: PackageCache::new()?,
        })
    }

    pub fn scan_directory(&mut self, path: &Path) -> Result<Vec<PackageMetadata>> {
        let mut found = Vec::new();

        for entry in WalkDir::new(path)
            .follow_links(false)
            .max_depth(8)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let p = e.path();
                // Skip common non-package directories
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    if name == ".git" || name == "target" || name == "node_modules" || name == "build" || name == ".idea" || name == ".vscode" {
                        return false;
                    }
                }
                true
            })
        {
            let entry_path = entry.path();

            if let Some(format) = formats::detect_format(entry_path) {
                let metadata = if let Some(cached) = self.cache.get(&entry_path.to_path_buf()) {
                    cached.clone()
                } else {
                    match format.extract_metadata(entry_path) {
                        Ok(mut meta) => {
                            meta.source_path = entry_path.to_path_buf();
                            self.cache.insert(entry_path.to_path_buf(), meta.clone())
                                .context("Failed to cache metadata")?;
                            meta
                        }
                        Err(e) => {
                            log::warn!("Failed to extract metadata from {}: {}", 
                                entry_path.display(), e);
                            continue;
                        }
                    }
                };

                found.push(metadata);
            }
        }

        // Batch save after scan completes
        if let Err(e) = self.cache.save() {
            log::warn!("Failed to save cache: {}", e);
        }

        Ok(found)
    }

    pub fn list_all(&self) -> Vec<&PackageMetadata> {
        self.cache.all()
    }

    pub fn get_cache(&self) -> &PackageCache {
        &self.cache
    }

    pub fn get_cache_mut(&mut self) -> &mut PackageCache {
        &mut self.cache
    }
}

