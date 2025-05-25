#[cfg(test)]
mod tests {
    use craft_launcher_rust::craft_launcher::utils::{
        file_operations::file_utils, networking::networking,
    };
    use serde_json::Value;
    use std::path::PathBuf;

    /**
     * Tests downloading the version manifest from Mojang's API and saving it to the test_data directory.
     * This test verifies that:
     * 1. The download function works correctly
     * 2. The file is saved with the correct name
     * 3. The content is valid JSON with the expected structure
     */
    #[test]
    fn test_download_version_manifest() {
        // Set up the test paths
        let test_dir = PathBuf::from("test_data/versions");
        let manifest_path = test_dir.join("version_manifest_v2.json"); // Ensure the directory exists
        if !test_dir.exists() {
            std::fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        }

        // If the file already exists, remove it to ensure a clean test
        if file_utils::exists(&manifest_path) {
            let _ = file_utils::delete_file(&manifest_path, true);
        }

        // Download the manifest
        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        let result = networking::download_file(url, &manifest_path);

        // Verify download was successful
        assert!(
            result.is_ok(),
            "Failed to download version manifest: {:?}",
            result
        );

        // Verify file exists
        assert!(
            file_utils::exists(&manifest_path),
            "Manifest file does not exist after download"
        );

        // Verify content is valid JSON with expected structure
        let json_result: Result<Value, _> =
            file_utils::read_struct_from_file_as_json(&manifest_path);
        assert!(json_result.is_ok(), "Failed to parse manifest as JSON");

        let json_data = json_result.unwrap();
        assert!(json_data.is_object(), "Manifest is not a JSON object");
        assert!(
            json_data.get("latest").is_some(),
            "Manifest missing 'latest' field"
        );
        assert!(
            json_data.get("versions").is_some(),
            "Manifest missing 'versions' field"
        );

        // Clean up resources
        file_utils::close_file(&manifest_path).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to close manifest file: {}", e);
        });
    }
}
