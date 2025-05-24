pub mod assets_parser {
    use crate::craft_launcher::utils::file_operations::file_utils;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt;
    use std::path::PathBuf;

    /// Error type for assets parsing operations
    #[derive(Debug)]
    pub struct AssetsParseError {
        message: String,
    }

    impl AssetsParseError {
        fn new(message: &str) -> Self {
            AssetsParseError {
                message: message.to_string(),
            }
        }
    }

    impl fmt::Display for AssetsParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Assets parse error: {}", self.message)
        }
    }

    impl Error for AssetsParseError {}

    /// Represents the root structure of a Minecraft assets index JSON file
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AssetsIndex {
        /// Format version of the assets index
        pub objects: HashMap<String, AssetObject>,

        /// Whether to map resources to a legacy format
        #[serde(rename = "map_to_resources", skip_serializing_if = "Option::is_none")]
        pub map_to_resources: Option<bool>,

        /// Whether to use a virtual file system for assets
        #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
        pub virtual_mode: Option<bool>,
    }

    /// Represents a single asset object entry in the assets index
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AssetObject {
        /// The SHA-1 hash of the asset file
        pub hash: String,

        /// The size of the asset file in bytes
        pub size: u64,
    }

    impl AssetsIndex {
        /// Parse an assets index from JSON string
        pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json)
        }

        /// Convert the assets index to a JSON string
        pub fn to_json(&self) -> Result<String, serde_json::Error> {
            serde_json::to_string(self)
        }

        /// Get the number of assets in this index
        pub fn get_asset_count(&self) -> usize {
            self.objects.len()
        }

        /// Get the total size of all assets in this index
        pub fn get_total_size(&self) -> u64 {
            self.objects.values().map(|obj| obj.size).sum()
        }

        /// Check if this assets index uses the virtual file system
        pub fn is_virtual(&self) -> bool {
            self.virtual_mode.unwrap_or(false)
        }

        /// Check if this assets index should map resources to legacy format
        pub fn is_map_to_resources(&self) -> bool {
            self.map_to_resources.unwrap_or(false)
        }

        /// Get the path of an asset in the assets directory structure
        ///
        /// The path is determined by the first two characters of the hash
        pub fn get_asset_path(&self, name: &str) -> Option<(String, &AssetObject)> {
            self.objects.get(name).map(|obj| {
                let hash = &obj.hash;
                let directory = &hash[0..2];
                let path = format!("{}/{}", directory, hash);
                (path, obj)
            })
        }

        /// Get the raw JSON string for an assets index from a root directory and index ID
        ///
        /// This function constructs the path to the assets index JSON file using the pattern:
        /// `root_dir/assets/indexes/{id}.json` and returns the raw JSON content.
        ///
        /// # Arguments
        ///
        /// * `root_dir` - Path to the root directory containing the assets folder
        /// * `id` - The ID of the assets index (e.g., "1.12", "17")
        ///
        /// # Returns
        ///
        /// * `Result<String, Box<dyn Error>>` - The raw JSON string or an error
        pub fn get_json_from_root(root_dir: &PathBuf, id: &str) -> Result<String, Box<dyn Error>> {
            // Construct path to assets/indexes/[id].json
            let index_path = root_dir
                .join("assets")
                .join("indexes")
                .join(format!("{}.json", id));

            // Check if the file exists
            if !file_utils::exists(&index_path) {
                return Err(Box::new(AssetsParseError::new(&format!(
                    "Assets index file not found at: {}",
                    index_path.display()
                ))));
            }

            // Read the file contents
            let json_str = file_utils::read_text(&index_path)?;
            Ok(json_str)
        }

        /// Load assets index from a root directory and index ID
        ///
        /// This function constructs the path to the assets index JSON file using the pattern:
        /// `root_dir/assets/indexes/{id}.json` and loads the assets index.
        ///
        /// # Arguments
        ///
        /// * `root_dir` - Path to the root directory containing the assets folder
        /// * `id` - The ID of the assets index (e.g., "1.12", "17")
        ///
        /// # Returns
        ///
        /// * `Result<Self, Box<dyn Error>>` - The parsed assets index or an error
        pub fn from_root_dir(root_dir: &PathBuf, id: &str) -> Result<Self, Box<dyn Error>> {
            let json_str = Self::get_json_from_root(root_dir, id)?;
            let assets_index = Self::from_json(&json_str)?;
            Ok(assets_index)
        }

        /// Load assets index from a specific file path
        ///
        /// # Arguments
        ///
        /// * `file_path` - Path to the assets index JSON file
        ///
        /// # Returns
        ///
        /// * `Result<Self, Box<dyn Error>>` - The parsed assets index or an error
        pub fn from_file(file_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
            // Check if the file exists
            if !file_utils::exists(file_path) {
                return Err(Box::new(AssetsParseError::new(&format!(
                    "Assets index file not found at: {}",
                    file_path.display()
                ))));
            }

            // Read and parse the file
            let json_str = file_utils::read_text(file_path)?;
            let assets_index = Self::from_json(&json_str)?;

            Ok(assets_index)
        }
    }

    impl AssetObject {
        /// Get the file path for this asset in the objects directory
        pub fn get_path(&self) -> String {
            let directory = &self.hash[0..2];
            format!("{}/{}", directory, self.hash)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::assets_parser::AssetsIndex;
    use std::path::PathBuf;

    // Tests that the parser correctly loads an assets index from a root directory
    #[test]
    fn test_from_root_dir() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
        let result = AssetsIndex::from_root_dir(&root_dir, "1.12");

        // Check that the result is Ok and contains valid assets data
        assert!(
            result.is_ok(),
            "Failed to parse assets index: {:?}",
            result.err()
        );

        let assets = result.unwrap();
        // Verify that the assets index contains objects
        assert!(
            assets.get_asset_count() > 0,
            "Assets index should contain objects"
        );

        // Check some expected properties of the assets index
        println!(
            "Found {} assets with total size {}",
            assets.get_asset_count(),
            assets.get_total_size()
        );
    } // Tests that the parser correctly loads an assets index from a specific file
    #[test]
    fn test_from_file() {
        let assets_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("assets/indexes/1.12.json");
        let result = AssetsIndex::from_file(&assets_file);

        // Check that the result is Ok and contains valid assets data
        assert!(
            result.is_ok(),
            "Failed to parse assets index: {:?}",
            result.err()
        );

        let assets = result.unwrap();
        // Verify that the assets index contains objects
        assert!(
            assets.get_asset_count() > 0,
            "Assets index should contain objects"
        );

        // Check if we can get paths for assets
        if let Some((path, obj)) = assets.get_asset_path("minecraft/sounds/random/bow.ogg") {
            assert!(!path.is_empty(), "Asset path should not be empty");
            assert_eq!(
                path.len(),
                obj.get_path().len(),
                "Path methods should return same length"
            );
        } else {
            panic!("Should find minecraft/sounds/random/bow.ogg in assets");
        }
    } // Tests the virtual mode and map_to_resources properties
    #[test]
    fn test_assets_properties() {
        let assets_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("assets/indexes/1.12.json");
        let result = AssetsIndex::from_file(&assets_file);

        assert!(result.is_ok());
        let assets = result.unwrap();

        // Check virtual mode property
        println!("Is virtual mode: {}", assets.is_virtual());

        // Check map_to_resources property
        println!("Is map_to_resources: {}", assets.is_map_to_resources());
    }

    // Tests error handling for assets index parsing
    #[test]
    fn test_error_handling() {
        // Test with non-existent path
        let invalid_path = PathBuf::from("non_existent_directory");
        let result = AssetsIndex::from_root_dir(&invalid_path, "1.12");
        assert!(result.is_err(), "Should error with invalid path");

        // Test with non-existent ID
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
        let result = AssetsIndex::from_root_dir(&root_dir, "non_existent_version");
        assert!(result.is_err(), "Should error with invalid version ID");
    }

    // Tests getting the raw JSON string from a root directory and ID
    #[test]
    fn test_get_json_from_root() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
        let result = AssetsIndex::get_json_from_root(&root_dir, "1.12");

        // Check that the result is Ok and contains valid JSON
        assert!(
            result.is_ok(),
            "Failed to get JSON string: {:?}",
            result.err()
        );

        let json_str = result.unwrap();
        assert!(!json_str.is_empty(), "JSON string should not be empty");

        // Verify that the JSON string can be parsed into an AssetsIndex
        let parse_result = AssetsIndex::from_json(&json_str);
        assert!(
            parse_result.is_ok(),
            "Failed to parse JSON string: {:?}",
            parse_result.err()
        );
    }
}
