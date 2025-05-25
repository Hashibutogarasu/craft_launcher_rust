pub mod version_manifest_parser {
    use serde::{Deserialize, Serialize};

    /// Structure representing the latest Minecraft versions
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Latest {
        pub release: String,
        pub snapshot: String,
    }

    /// Structure representing a single version in the version manifest
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Version {
        pub id: String,
        #[serde(rename = "type")]
        pub version_type: String,
        pub url: String,
        pub time: String,
        pub release_time: Option<String>,
        #[serde(rename = "releaseTime")]
        pub release_time_alternate: Option<String>,
        pub sha1: String,
        pub compliance_level: Option<i32>,
        #[serde(rename = "complianceLevel")]
        pub compliance_level_alternate: Option<i32>,
    }

    /// Main structure representing the Minecraft version manifest
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VersionManifest {
        pub latest: Latest,
        pub versions: Vec<Version>,
    }

    impl VersionManifest {
        /// Parse a version manifest from JSON string
        ///
        /// # Arguments
        ///
        /// * `json_str` - The JSON string containing the version manifest data
        ///
        /// # Returns
        ///
        /// * `Result<VersionManifest, serde_json::Error>` - The parsed manifest or an error
        pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json_str)
        }

        /// Get a specific version by its ID
        ///
        /// # Arguments
        ///
        /// * `id` - The version ID to search for
        ///
        /// # Returns
        ///
        /// * `Option<&Version>` - The version if found, or None
        pub fn get_version(&self, id: &str) -> Option<&Version> {
            self.versions.iter().find(|v| v.id == id)
        }

        /// Get the latest release version
        ///
        /// # Returns
        ///
        /// * `Option<&Version>` - The latest release version if found, or None
        pub fn get_latest_release(&self) -> Option<&Version> {
            self.get_version(&self.latest.release)
        }

        /// Get the latest snapshot version
        ///
        /// # Returns
        ///
        /// * `Option<&Version>` - The latest snapshot version if found, or None
        pub fn get_latest_snapshot(&self) -> Option<&Version> {
            self.get_version(&self.latest.snapshot)
        }
    }

    /// Parse a version manifest from a file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the version manifest file
    ///
    /// # Returns
    ///
    /// * `Result<VersionManifest, Box<dyn std::error::Error>>` - The parsed manifest or an error
    pub fn parse_version_manifest_from_file(
        path: &str,
    ) -> Result<VersionManifest, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let manifest = VersionManifest::from_json(&content)?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::version_manifest_parser::*;
    use std::path::PathBuf;

    // Helper function to get the test data path
    fn get_test_data_path(relative_path: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data");
        path.push(relative_path);
        path
    }

    #[test]
    /// Test that the version manifest can be parsed from a file
    fn test_parse_version_manifest() {
        let manifest_path = get_test_data_path("versions/version_manifest_v2.json");
        let result = parse_version_manifest_from_file(manifest_path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to parse version manifest");

        let manifest = result.unwrap();
        assert_eq!(manifest.latest.release, "1.21.5");
        assert_eq!(manifest.latest.snapshot, "25w21a");
        assert!(
            !manifest.versions.is_empty(),
            "Versions list should not be empty"
        );
    }

    #[test]
    /// Test that specific versions can be retrieved from the manifest
    fn test_get_versions() {
        let manifest_path = get_test_data_path("versions/version_manifest_v2.json");
        let manifest = parse_version_manifest_from_file(manifest_path.to_str().unwrap()).unwrap();

        // Test getting latest release
        let latest_release = manifest.get_latest_release();
        assert!(latest_release.is_some(), "Latest release should be found");
        assert_eq!(latest_release.unwrap().id, "1.21.5");

        // Test getting latest snapshot
        let latest_snapshot = manifest.get_latest_snapshot();
        assert!(latest_snapshot.is_some(), "Latest snapshot should be found");
        assert_eq!(latest_snapshot.unwrap().id, "25w21a");

        // Test getting a specific version
        let specific_version = manifest.get_version("25w20a");
        assert!(specific_version.is_some(), "Version 25w20a should be found");
        let version = specific_version.unwrap();
        assert_eq!(version.id, "25w20a");
        assert_eq!(version.version_type, "snapshot");
    }

    #[test]
    /// Test handling of non-existent versions
    fn test_nonexistent_version() {
        let manifest_path = get_test_data_path("versions/version_manifest_v2.json");
        let manifest = parse_version_manifest_from_file(manifest_path.to_str().unwrap()).unwrap();

        let nonexistent = manifest.get_version("this-version-does-not-exist");
        assert!(
            nonexistent.is_none(),
            "Non-existent version should return None"
        );
    }

    #[test]
    /// Test error handling for missing files
    fn test_missing_file() {
        let result = parse_version_manifest_from_file("non-existent-file.json");
        assert!(result.is_err(), "Should return error for missing file");
    }
}
