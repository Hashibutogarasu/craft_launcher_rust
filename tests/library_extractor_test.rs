#[cfg(test)]
mod tests {
    use craft_launcher_rust::craft_launcher::java::library_extractor::library_extractor::extract_native_libraries;
    use std::path::PathBuf;

    #[test]
    fn test_native_library_extraction() {
        // Get the test data directory path
        let test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

        // Use a version that is known to have native libraries
        let version_id = "1.16.5";

        // Extract native libraries
        let result = extract_native_libraries(&test_data_dir, version_id);
        
        match result {
            Ok(extracted_dir) => {
                println!("Successfully extracted native libraries to: {}", extracted_dir.display());
                
                // Verify that the directory contains at least some files
                let entries = std::fs::read_dir(&extracted_dir).expect("Failed to read directory");
                let file_count = entries.count();
                
                assert!(file_count > 0, "No files were extracted");
                println!("Found {} extracted files", file_count);
            },
            Err(e) => {
                if let craft_launcher_rust::craft_launcher::java::library_extractor::library_extractor::LibraryExtractionError::NoNativeLibrariesFound = e {
                    println!("No native libraries found for this version and OS, which is acceptable");
                } else {
                    panic!("Failed to extract native libraries: {:?}", e);
                }
            }
        }
    }
}
