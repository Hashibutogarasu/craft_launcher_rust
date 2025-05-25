pub mod version_parser {
    use crate::craft_launcher::core::version::legacy::legacy_fabric::legacy_fabric::LegacyFabricManifest;
    use crate::craft_launcher::core::version::legacy::legacy_forge::legacy_forge::LegacyForgeVersion;
    use crate::craft_launcher::core::version::legacy::legacy_vanilla::legacy_vanilla::LegacyVanillaVersion;
    use crate::craft_launcher::core::version::modern::modern_fabric::modern_fabric::ModernFabricManifest;
    use crate::craft_launcher::core::version::modern::modern_forge::modern_forge::ForgeVersion;
    use crate::craft_launcher::core::version::modern::modern_neoforge::modern_neoforge::NeoForgeVersion;
    use crate::craft_launcher::core::version::modern::modern_vanilla::modern_vanilla::ModernVanillaVersion;
    use crate::craft_launcher::utils::file_operations::file_utils;

    use serde_json::Value;
    use std::error::Error;
    use std::fmt;
    use std::path::PathBuf;

    /// Represents different types of Minecraft versions and loaders
    #[derive(Debug)]
    pub enum MinecraftVersion {
        /// Modern Vanilla Minecraft (1.13+)
        ModernVanilla(ModernVanillaVersion),
        /// Legacy Vanilla Minecraft (pre 1.13)
        LegacyVanilla(LegacyVanillaVersion),
        /// Modern Forge Minecraft
        ModernForge(ForgeVersion),
        /// Legacy Forge Minecraft
        LegacyForge(LegacyForgeVersion),
        /// Modern Fabric Minecraft
        ModernFabric(ModernFabricManifest),
        /// Legacy Fabric Minecraft
        LegacyFabric(LegacyFabricManifest),
        /// NeoForge Minecraft
        NeoForge(NeoForgeVersion),
    }

    /// Custom error for version parsing
    #[derive(Debug)]
    pub struct VersionParseError {
        message: String,
    }

    impl VersionParseError {
        fn new(message: &str) -> Self {
            VersionParseError {
                message: message.to_string(),
            }
        }
    }

    impl fmt::Display for VersionParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Version parse error: {}", self.message)
        }
    }

    impl Error for VersionParseError {}

    /// Parse a version JSON string into the appropriate version structure
    ///
    /// This function attempts to determine the type of version (vanilla, forge, fabric, etc.)
    /// and the era (modern or legacy) from the JSON content, then parses it accordingly.
    ///
    /// # Arguments
    ///
    /// * `json_str` - A string slice containing the version JSON
    ///
    /// # Returns
    ///
    /// * `Result<MinecraftVersion, Box<dyn Error>>` - The parsed version or an error
    pub fn parse_version(json_str: &str) -> Result<MinecraftVersion, Box<dyn Error>> {
        // First, parse as a generic JSON Value to determine the type
        let json_value: Value = serde_json::from_str(json_str)?;

        // Check if it's a loader by looking for "inheritsFrom" field
        if let Some(inherits_from) = json_value.get("inheritsFrom") {
            let inherits_from = inherits_from
                .as_str()
                .ok_or_else(|| VersionParseError::new("inheritsFrom field is not a string"))?; // Attempt to identify which loader
            let id = json_value.get("id").and_then(Value::as_str).unwrap_or("");

            // Check for NeoForge first
            if id.contains("neoforge") {
                match serde_json::from_str::<NeoForgeVersion>(json_str) {
                    Ok(neoforge) => return Ok(MinecraftVersion::NeoForge(neoforge)),
                    Err(e) => {
                        return Err(Box::new(VersionParseError::new(&format!(
                            "Failed to parse as NeoForge: {}",
                            e
                        ))));
                    }
                }
            }
            // Check for Forge (modern or legacy)
            else if id.contains("forge") {
                // Check for minecraftArguments to determine if legacy
                if json_value.get("minecraftArguments").is_some() {
                    match serde_json::from_str::<LegacyForgeVersion>(json_str) {
                        Ok(forge) => return Ok(MinecraftVersion::LegacyForge(forge)),
                        Err(e) => {
                            return Err(Box::new(VersionParseError::new(&format!(
                                "Failed to parse as Legacy Forge: {}",
                                e
                            ))));
                        }
                    }
                } else {
                    match serde_json::from_str::<ForgeVersion>(json_str) {
                        Ok(forge) => return Ok(MinecraftVersion::ModernForge(forge)),
                        Err(e) => {
                            return Err(Box::new(VersionParseError::new(&format!(
                                "Failed to parse as Modern Forge: {}",
                                e
                            ))));
                        }
                    }
                }
            }
            // Check for NeoForge
            else if id.contains("neoforge") {
                match serde_json::from_str::<NeoForgeVersion>(json_str) {
                    Ok(neoforge) => return Ok(MinecraftVersion::NeoForge(neoforge)),
                    Err(e) => {
                        return Err(Box::new(VersionParseError::new(&format!(
                            "Failed to parse as NeoForge: {}",
                            e
                        ))));
                    }
                }
            }
            // Check for Fabric
            else if id.contains("fabric") {
                // Parse Minecraft version number from inherits_from
                let mc_version = parse_minecraft_version(inherits_from);

                // Compare versions
                if is_modern_minecraft(&mc_version) {
                    match serde_json::from_str::<ModernFabricManifest>(json_str) {
                        Ok(fabric) => return Ok(MinecraftVersion::ModernFabric(fabric)),
                        Err(e) => {
                            return Err(Box::new(VersionParseError::new(&format!(
                                "Failed to parse as Modern Fabric: {}",
                                e
                            ))));
                        }
                    }
                } else {
                    match serde_json::from_str::<LegacyFabricManifest>(json_str) {
                        Ok(fabric) => return Ok(MinecraftVersion::LegacyFabric(fabric)),
                        Err(e) => {
                            return Err(Box::new(VersionParseError::new(&format!(
                                "Failed to parse as Legacy Fabric: {}",
                                e
                            ))));
                        }
                    }
                }
            }

            // Unknown loader type
            return Err(Box::new(VersionParseError::new(&format!(
                "Unknown loader type with id: {}",
                id
            ))));
        }

        // If not a loader, then it's vanilla. Check if modern or legacy.
        let has_arguments = json_value.get("arguments").is_some();
        let has_minecraft_arguments = json_value.get("minecraftArguments").is_some();

        if has_arguments && !has_minecraft_arguments {
            // Modern Vanilla (1.13+)
            match serde_json::from_str::<ModernVanillaVersion>(json_str) {
                Ok(vanilla) => Ok(MinecraftVersion::ModernVanilla(vanilla)),
                Err(e) => Err(Box::new(VersionParseError::new(&format!(
                    "Failed to parse as Modern Vanilla: {}",
                    e
                )))),
            }
        } else if has_minecraft_arguments && !has_arguments {
            // Legacy Vanilla (pre 1.13)
            match serde_json::from_str::<LegacyVanillaVersion>(json_str) {
                Ok(vanilla) => Ok(MinecraftVersion::LegacyVanilla(vanilla)),
                Err(e) => Err(Box::new(VersionParseError::new(&format!(
                    "Failed to parse as Legacy Vanilla: {}",
                    e
                )))),
            }
        } else {
            Err(Box::new(VersionParseError::new(
                "Could not determine version type based on available fields",
            )))
        }
    }

    /// Extract a semantic version from a Minecraft version string
    ///
    /// # Arguments
    ///
    /// * `version` - A string slice containing a Minecraft version like "1.16.5"
    ///
    /// # Returns
    ///
    /// * `String` - The extracted semantic version
    pub fn parse_minecraft_version(version: &str) -> String {
        // Extract the first semantic version-like pattern
        // A simple approach is to take everything up to the first non-version character
        let mut result = String::new();
        let mut has_seen_digit = false;
        let mut has_seen_dot = false;

        for c in version.chars() {
            if c.is_digit(10) {
                has_seen_digit = true;
                result.push(c);
            } else if c == '.' && has_seen_digit {
                has_seen_dot = true;
                result.push(c);
            } else if has_seen_dot && has_seen_digit {
                // Stop at first non-digit after we've seen a digit and a dot
                break;
            }
        }

        result
    }

    /// Check if a Minecraft version is considered "modern" (1.13 or newer)
    ///
    /// # Arguments
    ///
    /// * `version` - A string slice containing a semantic version like "1.16.5"
    ///
    /// # Returns
    ///
    /// * `bool` - True if the version is 1.13 or newer
    pub fn is_modern_minecraft(version: &str) -> bool {
        // Split by dots
        let parts: Vec<&str> = version.split('.').collect();

        if parts.len() >= 2 {
            if let Ok(major) = parts[0].parse::<i32>() {
                if let Ok(minor) = parts[1].parse::<i32>() {
                    // Modern is considered 1.13 and above
                    return major > 1 || (major == 1 && minor >= 13);
                }
            }
        }

        // Default to false if we can't determine the version
        false
    }

    /// Parse a version JSON file into the appropriate version structure
    ///
    /// This function reads a version JSON file and attempts to determine the type of version
    /// (vanilla, forge, fabric, etc.) and the era (modern or legacy), then parses it accordingly.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the JSON file containing version information
    ///
    /// # Returns
    ///
    /// * `Result<MinecraftVersion, Box<dyn Error>>` - The parsed version or an error
    pub fn parse_version_from_file(
        file_path: &PathBuf,
    ) -> Result<MinecraftVersion, Box<dyn Error>> {
        let json_str = file_utils::read_text(file_path)?;
        parse_version(&json_str)
    }

    /// Parse a version directory containing a JSON file with the same name as the directory
    ///
    /// # Arguments
    ///
    /// * `version_dir` - Path to the directory containing the version JSON file
    ///
    /// # Returns
    ///
    /// * `Result<MinecraftVersion, Box<dyn Error>>` - The parsed version or an error
    pub fn parse_version_directory(
        version_dir: &PathBuf,
    ) -> Result<MinecraftVersion, Box<dyn Error>> {
        if !file_utils::is_dir(version_dir) {
            return Err(Box::new(VersionParseError::new(&format!(
                "Not a directory: {}",
                version_dir.display()
            ))));
        }

        // Get the directory name
        let dir_name = version_dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                VersionParseError::new(&format!(
                    "Invalid directory name: {}",
                    version_dir.display()
                ))
            })?;

        // Construct the path to the JSON file
        let json_file_path = version_dir.join(format!("{}.json", dir_name));

        if !file_utils::exists(&json_file_path) {
            return Err(Box::new(VersionParseError::new(&format!(
                "Version JSON file does not exist: {}",
                json_file_path.display()
            ))));
        }

        parse_version_from_file(&json_file_path)
    }

    /**
     * Parse a version JSON file from a root directory path and version ID.
     *
     * This function constructs a path to the version JSON file using the pattern:
     * `root_dir/versions/version_id/version_id.json` and parses the version information.
     *
     * # Arguments
     *
     * * `root_dir` - Path to the root directory containing the versions folder
     * * `version_id` - The ID of the version to parse
     *
     * # Returns
     *
     * * `Result<MinecraftVersion, Box<dyn Error>>` - The parsed version or an error
     */    /**
     * Parse a version JSON file from a root directory path and version ID.
     * 
     * This function constructs a path to the version JSON file using the pattern:
     * `root_dir/versions/version_id/version_id.json` and parses the version information.
     */
    pub fn parse_version_from_root_dir(
        root_dir: &PathBuf,
        version_id: &str,
    ) -> Result<MinecraftVersion, Box<dyn Error>> {
        // Construct path to versions/[versionId]/[versionId].json
        let version_path = root_dir
            .join("versions")
            .join(version_id)
            .join(format!("{}.json", version_id));

        // Check if the file exists
        if !file_utils::exists(&version_path) {
            return Err(Box::new(VersionParseError::new(&format!(
                "Version file does not exist at path: {}",
                version_path.display()
            ))));
        }

        // Parse the file
        parse_version_from_file(&version_path)
    }
}

#[cfg(test)]
mod tests {
    use super::version_parser::{
        MinecraftVersion, parse_version, parse_version_directory, parse_version_from_file,
    };
    use crate::craft_launcher::core::version::version_parser::version_parser;
    use std::path::PathBuf;

    // Common function to build path to test data or get the root directory
    fn test_data_path(version_dir: Option<&str>) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data");

        if let Some(version_dir) = version_dir {
            path.push("versions");
            path.push(version_dir);
        }

        path
    }

    // Tests that the parser correctly detects and returns Modern Vanilla version type
    #[test]
    fn test_parse_modern_vanilla() {
        let version_dir = test_data_path(Some("1.16.5"));
        let json_file = version_dir.join("1.16.5.json");

        // Parse the JSON file
        let result = parse_version_from_file(&json_file);

        // Check that the result is Ok and contains a ModernVanilla variant
        if let Err(e) = &result {
            panic!("Failed to parse ModernVanilla: {}", e);
        }
        assert!(result.is_ok());
        if let Ok(version) = result {
            match version {
                MinecraftVersion::ModernVanilla(_) => assert!(true),
                _ => panic!("Expected ModernVanilla variant, got: {:?}", version),
            }
        }

        // Also test using directory parsing
        let dir_result = parse_version_directory(&version_dir);
        assert!(dir_result.is_ok());
        if let Ok(version) = dir_result {
            match version {
                MinecraftVersion::ModernVanilla(_) => assert!(true),
                _ => panic!("Expected ModernVanilla variant, got: {:?}", version),
            }
        }
    }

    // Tests that the parser correctly detects and returns Legacy Vanilla version type
    #[test]
    fn test_parse_legacy_vanilla() {
        let version_dir = test_data_path(Some("1.12.2"));
        let json_file = version_dir.join("1.12.2.json");

        // Parse the JSON file
        let result = parse_version_from_file(&json_file);

        // Check that the result is Ok and contains a LegacyVanilla variant
        assert!(result.is_ok());
        if let Ok(version) = result {
            match version {
                MinecraftVersion::LegacyVanilla(_) => assert!(true),
                _ => panic!("Expected LegacyVanilla variant, got: {:?}", version),
            }
        }

        // Also test using directory parsing
        let dir_result = parse_version_directory(&version_dir);
        assert!(dir_result.is_ok());
        if let Ok(version) = dir_result {
            match version {
                MinecraftVersion::LegacyVanilla(_) => assert!(true),
                _ => panic!("Expected LegacyVanilla variant, got: {:?}", version),
            }
        }
    }

    // Tests that the parser correctly detects and returns Modern Forge version type
    #[test]
    fn test_parse_modern_forge() {
        let version_dir = test_data_path(Some("1.21-forge-51.0.33"));
        let json_file = version_dir.join("1.21-forge-51.0.33.json");

        // Parse the JSON file
        let result = parse_version_from_file(&json_file);

        // Check that the result is Ok and contains a ModernForge variant
        assert!(result.is_ok());
        if let Ok(version) = result {
            match version {
                MinecraftVersion::ModernForge(_) => assert!(true),
                _ => panic!("Expected ModernForge variant, got: {:?}", version),
            }
        }

        // Also test using directory parsing
        let dir_result = parse_version_directory(&version_dir);
        assert!(dir_result.is_ok());
        if let Ok(version) = dir_result {
            match version {
                MinecraftVersion::ModernForge(_) => assert!(true),
                _ => panic!("Expected ModernForge variant, got: {:?}", version),
            }
        }
    }

    // Tests that the parser correctly detects and returns Fabric version types
    #[test]
    fn test_parse_fabric() {
        // Modern Fabric
        let modern_version_dir = test_data_path(Some("fabric-loader-0.14.24-1.20.2"));
        let modern_json_file = modern_version_dir.join("fabric-loader-0.14.24-1.20.2.json");

        // Parse the JSON file
        let modern_result = parse_version_from_file(&modern_json_file);

        // Check that the result is Ok and contains a ModernFabric variant
        assert!(modern_result.is_ok());
        if let Ok(version) = modern_result {
            match version {
                MinecraftVersion::ModernFabric(_) => assert!(true),
                _ => panic!("Expected ModernFabric variant, got: {:?}", version),
            }
        }

        // Also test using directory parsing
        let modern_dir_result = parse_version_directory(&modern_version_dir);
        assert!(modern_dir_result.is_ok());
        if let Ok(version) = modern_dir_result {
            match version {
                MinecraftVersion::ModernFabric(_) => assert!(true),
                _ => panic!("Expected ModernFabric variant, got: {:?}", version),
            }
        }

        // Legacy Fabric
        let legacy_version_dir = test_data_path(Some("fabric-loader-0.16.14-1.12.2"));
        let legacy_json_file = legacy_version_dir.join("fabric-loader-0.16.14-1.12.2.json");

        // Parse the JSON file
        let legacy_result = parse_version_from_file(&legacy_json_file);

        // Check that the result is Ok and contains a LegacyFabric variant
        assert!(legacy_result.is_ok());
        if let Ok(version) = legacy_result {
            match version {
                MinecraftVersion::LegacyFabric(_) => assert!(true),
                _ => panic!("Expected LegacyFabric variant, got: {:?}", version),
            }
        }

        // Also test using directory parsing
        let legacy_dir_result = parse_version_directory(&legacy_version_dir);
        assert!(legacy_dir_result.is_ok());
        if let Ok(version) = legacy_dir_result {
            match version {
                MinecraftVersion::LegacyFabric(_) => assert!(true),
                _ => panic!("Expected LegacyFabric variant, got: {:?}", version),
            }
        }
    }

    // Tests that the parser correctly detects and returns NeoForge version type
    #[test]
    fn test_parse_neoforge() {
        let version_dir = test_data_path(Some("neoforge-21.5.66-beta"));
        let json_file = version_dir.join("neoforge-21.5.66-beta.json");

        // Parse the JSON file
        let result = parse_version_from_file(&json_file);

        // Check that the result is Ok and contains a NeoForge variant
        assert!(result.is_ok());
        if let Ok(version) = result {
            match version {
                MinecraftVersion::NeoForge(_) => assert!(true),
                _ => panic!("Expected NeoForge variant, got: {:?}", version),
            }
        }

        // Also test using directory parsing
        let dir_result = parse_version_directory(&version_dir);
        assert!(dir_result.is_ok());
        if let Ok(version) = dir_result {
            match version {
                MinecraftVersion::NeoForge(_) => assert!(true),
                _ => panic!("Expected NeoForge variant, got: {:?}", version),
            }
        }
    } // Tests the helper function for parsing Minecraft version numbers
    #[test]
    fn test_parse_minecraft_version() {
        assert_eq!(version_parser::parse_minecraft_version("1.16.5"), "1.16.5");
        assert_eq!(version_parser::parse_minecraft_version("1.8.9"), "1.8.9");
        assert_eq!(
            version_parser::parse_minecraft_version("1.12.2-pre1"),
            "1.12.2"
        );
        assert_eq!(
            version_parser::parse_minecraft_version("1.20.1-neoforge-47.1.70"),
            "1.20.1"
        );
    }

    // Tests the helper function that determines if a version is modern
    #[test]
    fn test_is_modern_minecraft() {
        // Modern versions (1.13+)
        assert!(version_parser::is_modern_minecraft("1.13"));
        assert!(version_parser::is_modern_minecraft("1.16.5"));
        assert!(version_parser::is_modern_minecraft("1.20.1"));

        // Legacy versions (pre-1.13)
        assert!(!version_parser::is_modern_minecraft("1.12.2"));
        assert!(!version_parser::is_modern_minecraft("1.8.9"));
        assert!(!version_parser::is_modern_minecraft("1.7.10"));

        // Edge cases
        assert!(!version_parser::is_modern_minecraft("invalid"));
        assert!(!version_parser::is_modern_minecraft(""));
    }

    // Tests error cases for invalid JSON
    #[test]
    fn test_parse_invalid_json() {
        // Invalid JSON syntax
        let result = parse_version("{not valid json}");
        assert!(result.is_err());

        // Valid JSON but not a valid version format (missing required fields)
        let result = parse_version(r#"{"id": "test"}"#);
        assert!(result.is_err());
    }

    // Tests error cases for file operations
    #[test]
    fn test_file_operations_errors() {
        // Non-existent file
        let non_existent_file = PathBuf::from("/non/existent/file.json");
        let result = parse_version_from_file(&non_existent_file);
        assert!(result.is_err());

        // Non-existent directory
        let non_existent_dir = PathBuf::from("/non/existent/directory");
        let result = parse_version_directory(&non_existent_dir);
        assert!(result.is_err());
    }

    // Test directory parsing with all available test data
    #[test]
    fn test_parse_all_directories() {
        let test_data_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("versions");

        // Get all subdirectories in test_data/versions
        if let Ok(entries) = std::fs::read_dir(&test_data_root) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        println!("Testing directory: {:?}", path);

                        // Try to parse the directory
                        let result = parse_version_directory(&path);

                        // We expect all directories in test_data/versions to be valid
                        assert!(result.is_ok(), "Failed to parse directory: {:?}", path);
                    }
                }
            }
        }
    }

    // Tests that the parser correctly works for parsing versions from root directory
    #[test]
    fn test_parse_version_from_root_dir() {
        // Test with modern vanilla
        let root_dir = test_data_path(None);
        let result = version_parser::parse_version_from_root_dir(&root_dir, "1.16.5");

        assert!(result.is_ok());
        if let Ok(version) = result {
            match version {
                MinecraftVersion::ModernVanilla(_) => (),
                _ => panic!("Expected ModernVanilla, got {:?}", version),
            }
        }

        // Test with legacy vanilla
        let legacy_result = version_parser::parse_version_from_root_dir(&root_dir, "1.12.2");
        assert!(legacy_result.is_ok());
        if let Ok(version) = legacy_result {
            match version {
                MinecraftVersion::LegacyVanilla(_) => (),
                _ => panic!("Expected LegacyVanilla, got {:?}", version),
            }
        }

        // Test with modern forge
        let forge_result =
            version_parser::parse_version_from_root_dir(&root_dir, "1.21-forge-51.0.33");
        assert!(forge_result.is_ok());
        if let Ok(version) = forge_result {
            match version {
                MinecraftVersion::ModernForge(_) => (),
                _ => panic!("Expected ModernForge, got {:?}", version),
            }
        }
    }
}
