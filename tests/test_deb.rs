#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use dpino::formats::deb::DebPackage;
    use dpino::formats::PackageFormat;

    #[test]
    fn test_deb_detection() {
        let path = PathBuf::from("tests/fixtures/sample.deb");
        if path.exists() {
            assert!(DebPackage::detect(&path));
        }
    }

    #[test]
    fn test_deb_metadata_extraction() {
        let path = PathBuf::from("tests/fixtures/sample.deb");
        if path.exists() {
            let pkg = DebPackage;
            let result = pkg.extract_metadata(&path);
            assert!(result.is_ok(), "Failed to extract metadata: {:?}", result.err());
            let metadata = result.unwrap();
            assert!(!metadata.name.is_empty());
            assert!(!metadata.package_type.is_empty());
        }
    }
}

