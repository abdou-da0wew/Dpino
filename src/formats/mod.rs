pub mod deb;
pub mod appimage;
pub mod appdir;

pub use deb::DebPackage;
pub use appimage::AppImagePackage;
pub use appdir::AppDirPackage;

use crate::cache::PackageMetadata;
use anyhow::Result;

pub trait PackageFormat: Send + Sync {
    fn detect(path: &std::path::Path) -> bool
    where
        Self: Sized;
    fn extract_metadata(&self, path: &std::path::Path) -> Result<PackageMetadata>;
    fn extract_to(&self, path: &std::path::Path, out_dir: &std::path::Path) -> Result<()>;
}

pub fn detect_format(path: &std::path::Path) -> Option<Box<dyn PackageFormat>> {
    if DebPackage::detect(path) {
        Some(Box::new(DebPackage))
    } else if AppImagePackage::detect(path) {
        Some(Box::new(AppImagePackage))
    } else if AppDirPackage::detect(path) {
        Some(Box::new(AppDirPackage))
    } else {
        None
    }
}

