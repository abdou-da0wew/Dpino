use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub categories: Vec<String>,
    pub maintainer: String,
    pub icon_path: Option<PathBuf>,
    pub exec_command: Option<String>,
    pub description: String,
    pub mimetypes: Vec<String>,
    pub source_path: PathBuf,
    pub package_type: String,
    pub desktop_entry: Option<String>,
}

impl Default for PackageMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            categories: Vec::new(),
            maintainer: String::new(),
            icon_path: None,
            exec_command: None,
            description: String::new(),
            mimetypes: Vec::new(),
            source_path: PathBuf::new(),
            package_type: String::new(),
            desktop_entry: None,
        }
    }
}

pub struct PackageCache {
    cache_path: PathBuf,
    packages: HashMap<PathBuf, PackageMetadata>,
}

impl PackageCache {
    pub fn new() -> Result<Self> {
        let cache_dir = utils::get_xdg_data_home()?.join("dpino");
        utils::ensure_dir(&cache_dir)?;
        let cache_path = cache_dir.join("cache.json");
        
        let mut cache = Self {
            cache_path,
            packages: HashMap::new(),
        };
        
        cache.load()?;
        Ok(cache)
    }

    pub fn load(&mut self) -> Result<()> {
        if self.cache_path.exists() {
            let content = std::fs::read_to_string(&self.cache_path)
                .context("Failed to read cache file")?;
            let entries: Vec<(String, PackageMetadata)> = serde_json::from_str(&content)
                .unwrap_or_default();
            
            self.packages = entries
                .into_iter()
                .map(|(k, v)| (PathBuf::from(k), v))
                .collect();
        }
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let entries: Vec<(String, PackageMetadata)> = self.packages
            .iter()
            .map(|(k, v)| (k.to_string_lossy().to_string(), v.clone()))
            .collect();
        
        let content = serde_json::to_string_pretty(&entries)
            .context("Failed to serialize cache")?;
        
        std::fs::write(&self.cache_path, content)
            .context("Failed to write cache file")?;
        
        Ok(())
    }

    pub fn get(&self, path: &PathBuf) -> Option<&PackageMetadata> {
        self.packages.get(path)
    }

    pub fn insert(&mut self, path: PathBuf, metadata: PackageMetadata) -> Result<()> {
        self.packages.insert(path, metadata);
        self.save()?;
        Ok(())
    }

    pub fn remove(&mut self, path: &PathBuf) -> Result<()> {
        self.packages.remove(path);
        self.save()?;
        Ok(())
    }

    pub fn all(&self) -> Vec<&PackageMetadata> {
        self.packages.values().collect()
    }

    pub fn clear(&mut self) -> Result<()> {
        self.packages.clear();
        self.save()?;
        Ok(())
    }
}

