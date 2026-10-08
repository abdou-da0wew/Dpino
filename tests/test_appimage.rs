#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use dpino::formats::appimage::AppImagePackage;
    use dpino::formats::PackageFormat;

    #[test]
    fn test_appimage_detection() {
        let path = PathBuf::from("tests/fixtures/sample.AppImage");
        if path.exists() {
            assert!(AppImagePackage::detect(&path));
        }
    }

    #[test]
    fn test_appimage_metadata_extraction() {
        let path = PathBuf::from("tests/fixtures/sample.AppImage");
        if path.exists() {
            let pkg = AppImagePackage;
            let result = pkg.extract_metadata(&path);
            assert!(result.is_ok(), "Failed to extract metadata: {:?}", result.err());
            let metadata = result.unwrap();
            assert!(!metadata.name.is_empty());
            assert_eq!(metadata.package_type, "AppImage");
        }
    }
}

