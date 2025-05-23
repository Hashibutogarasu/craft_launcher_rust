use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents base structure for Minecraft version JSON files
/// Common elements found across different version types (Vanilla, Forge, Fabric, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseVersion {
    /// The unique identifier of this version
    pub id: String,

    /// The time this version was last updated
    pub time: String,

    /// The time this version was released
    #[serde(rename = "releaseTime")]
    pub release_time: String,

    /// The type of this version (release, snapshot, etc)
    #[serde(rename = "type")]
    pub type_: String,

    /// The main Java class to execute
    #[serde(rename = "mainClass")]
    pub main_class: String,

    /// Libraries required by this version
    pub libraries: Vec<Library>,

    /// Identifier of the parent version (for modded versions)
    #[serde(rename = "inheritsFrom", skip_serializing_if = "Option::is_none")]
    pub inherits_from: Option<String>,

    /// Arguments for game and JVM (newer format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Arguments>,

    /// Command-line arguments to pass to Minecraft (legacy format)
    #[serde(rename = "minecraftArguments", skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,

    /// Logging configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
}

/// Arguments structure for newer Minecraft versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    /// Game arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<ArgumentValue>>,

    /// JVM arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm: Option<Vec<ArgumentValue>>,
}

/// Value that can be a string or a complex rule-based argument
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    /// Simple string argument
    String(String),

    /// Rule-based argument with conditions
    RuleArgument {
        /// Rules for when this argument applies
        rules: Vec<Rule>,

        /// The actual argument value(s)
        value: ArgumentValueInner,
    },
}

/// Inner value for rule-based arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValueInner {
    /// Single string value
    Single(String),

    /// Multiple string values
    Multiple(Vec<String>),
}

/// A library required by a Minecraft version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// The name of the library
    pub name: String,

    /// Download information for the library
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<LibraryDownloads>,

    /// Rules for when this library applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,

    /// Native library extraction information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<Extract>,

    /// Platform-specific natives mapping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives: Option<HashMap<String, String>>,

    /// URL to download the library (used by Forge and Fabric)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Download information for a library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    /// The main artifact of the library
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<LibraryArtifact>,

    /// Platform-specific classifier artifacts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<HashMap<String, LibraryArtifact>>,
}

/// A library artifact with path, hash, size and URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtifact {
    /// The path where the artifact should be stored
    pub path: String,

    /// The SHA-1 hash of the artifact
    pub sha1: String,

    /// The size of the artifact in bytes
    pub size: i64,

    /// The URL to download the artifact
    pub url: String,
}

/// Rules for when a library applies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Allow or disallow based on this rule
    pub action: String,

    /// OS-specific constraints for this rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Os>,

    /// Features required for this rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<HashMap<String, bool>>,
}

/// Operating system constraints for a rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Os {
    /// The name of the OS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The version of the OS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The architecture of the OS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
}

/// Native library extraction parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extract {
    /// Files to exclude from extraction
    pub exclude: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logging {
    /// Client logging configuration
    pub client: LoggingClient,
}

/// Client logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingClient {
    /// Command-line argument for logging
    pub argument: String,

    /// Log configuration file
    pub file: LogFile,

    /// Type of logging configuration
    #[serde(rename = "type")]
    pub type_: String,
}

/// Log configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFile {
    /// ID of the log file
    pub id: String,

    /// SHA-1 hash of the log file
    pub sha1: String,

    /// Size of the log file
    pub size: i64,

    /// URL to download the log file
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests serialization and deserialization of BaseVersion with Fabric loader JSON
    #[test]
    fn test_fabric_version_serde() {
        // Test data for a Fabric loader version
        let fabric_json = r#"{
            "inheritsFrom": "1.20.2",
            "releaseTime": "2023-11-03T13:37:49+0000",
            "mainClass": "net.fabricmc.loader.impl.launch.knot.KnotClient",
            "libraries": [
                {
                    "name": "net.fabricmc:tiny-mappings-parser:0.3.0+build.17",
                    "url": "https://maven.fabricmc.net/"
                },
                {
                    "name": "net.fabricmc:fabric-loader:0.14.24",
                    "url": "https://maven.fabricmc.net/"
                }
            ],
            "arguments": {
                "jvm": [
                    "-DFabricMcEmu= net.minecraft.client.main.Main "
                ],
                "game": []
            },
            "id": "fabric-loader-0.14.24-1.20.2",
            "time": "2023-11-03T13:37:49+0000",
            "type": "release"
        }"#;

        // Deserialize JSON string to BaseVersion
        let fabric_version: BaseVersion =
            serde_json::from_str(fabric_json).expect("Failed to deserialize Fabric JSON");

        // Validate deserialized fields
        assert_eq!(fabric_version.id, "fabric-loader-0.14.24-1.20.2");
        assert_eq!(fabric_version.type_, "release");
        assert_eq!(fabric_version.time, "2023-11-03T13:37:49+0000");
        assert_eq!(fabric_version.release_time, "2023-11-03T13:37:49+0000");
        assert_eq!(
            fabric_version.main_class,
            "net.fabricmc.loader.impl.launch.knot.KnotClient"
        );
        assert_eq!(fabric_version.inherits_from, Some("1.20.2".to_string()));
        assert_eq!(fabric_version.libraries.len(), 2);
        assert_eq!(
            fabric_version.libraries[0].name,
            "net.fabricmc:tiny-mappings-parser:0.3.0+build.17"
        );
        assert_eq!(
            fabric_version.libraries[0].url,
            Some("https://maven.fabricmc.net/".to_string())
        );
        assert!(fabric_version.arguments.is_some());
        assert!(fabric_version.minecraft_arguments.is_none());

        // Serialize back to JSON
        let serialized = serde_json::to_string_pretty(&fabric_version)
            .expect("Failed to serialize Fabric version");

        // Deserialize again to verify round-trip conversion
        let deserialized: BaseVersion =
            serde_json::from_str(&serialized).expect("Failed to deserialize after serialization");

        // Validate key fields after round-trip
        assert_eq!(deserialized.id, fabric_version.id);
        assert_eq!(deserialized.inherits_from, fabric_version.inherits_from);
        assert_eq!(deserialized.libraries.len(), fabric_version.libraries.len());
    }

    /// Tests serialization and deserialization of BaseVersion with OptiFine JSON
    #[test]
    fn test_optifine_version_serde() {
        // Test data for an OptiFine version
        let optifine_json = r#"{
            "id": "1.19.2-OptiFine_HD_U_H9",
            "inheritsFrom": "1.19.2",
            "time": "2023-01-12T18:55:41+09:00",
            "releaseTime": "2023-01-12T18:55:41+09:00",
            "type": "release",
            "libraries": [
                {
                    "name": "optifine:OptiFine:1.19.2_HD_U_H9"
                },
                {
                    "name": "optifine:launchwrapper-of:2.3"
                }
            ],
            "mainClass": "net.minecraft.launchwrapper.Launch",
            "arguments": {
                "game": [
                    "--tweakClass",
                    "optifine.OptiFineTweaker"
                ]
            }
        }"#;

        // Deserialize JSON string to BaseVersion
        let optifine_version: BaseVersion =
            serde_json::from_str(optifine_json).expect("Failed to deserialize OptiFine JSON");

        // Validate deserialized fields
        assert_eq!(optifine_version.id, "1.19.2-OptiFine_HD_U_H9");
        assert_eq!(optifine_version.type_, "release");
        assert_eq!(optifine_version.time, "2023-01-12T18:55:41+09:00");
        assert_eq!(optifine_version.release_time, "2023-01-12T18:55:41+09:00");
        assert_eq!(
            optifine_version.main_class,
            "net.minecraft.launchwrapper.Launch"
        );
        assert_eq!(optifine_version.inherits_from, Some("1.19.2".to_string()));
        assert_eq!(optifine_version.libraries.len(), 2);
        assert_eq!(
            optifine_version.libraries[0].name,
            "optifine:OptiFine:1.19.2_HD_U_H9"
        );
        assert!(optifine_version.arguments.is_some());
        assert!(optifine_version.minecraft_arguments.is_none());

        // Serialize back to JSON
        let serialized = serde_json::to_string_pretty(&optifine_version)
            .expect("Failed to serialize OptiFine version");

        // Deserialize again to verify round-trip conversion
        let deserialized: BaseVersion =
            serde_json::from_str(&serialized).expect("Failed to deserialize after serialization");

        // Validate key fields after round-trip
        assert_eq!(deserialized.id, optifine_version.id);
        assert_eq!(deserialized.inherits_from, optifine_version.inherits_from);
        assert_eq!(
            deserialized.libraries.len(),
            optifine_version.libraries.len()
        );
    }

    /// Tests serialization and deserialization using file operations
    #[test]
    fn test_file_serde() {
        use crate::craft_launcher::utils::file_operations::file_utils;
        use std::fs;

        // Create temporary directory for the test
        let temp_dir = std::env::temp_dir().join("base_version_test");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");
        }

        // Test data for a Forge version
        let forge_version = BaseVersion {
            id: "1.21-forge-51.0.33".to_string(),
            time: "2024-08-08T04:54:37+00:00".to_string(),
            release_time: "2024-08-08T04:54:37+00:00".to_string(),
            type_: "release".to_string(),
            main_class: "net.minecraftforge.bootstrap.ForgeBootstrap".to_string(),
            inherits_from: Some("1.21".to_string()),
            libraries: vec![
                Library {
                    name: "net.minecraftforge:forge:1.21-51.0.33:universal".to_string(),
                    downloads: None,
                    rules: None,
                    extract: None,
                    natives: None,
                    url: None,
                },
                Library {
                    name: "net.minecraftforge:forge:1.21-51.0.33:client".to_string(),
                    downloads: None,
                    rules: None,
                    extract: None,
                    natives: None,
                    url: None,
                },
            ],
            arguments: Some(Arguments {
                game: Some(vec![ArgumentValue::String("forge_client".to_string())]),
                jvm: Some(vec![]),
            }),
            minecraft_arguments: None,
            logging: None,
        };

        // Define test file path
        let test_file = temp_dir.join("forge_version.json");

        // Write structure to file as JSON
        file_utils::write_struct_to_file_as_json(&test_file, &forge_version)
            .expect("Failed to write structure to file");

        // Read structure from file
        let read_version: BaseVersion = file_utils::read_struct_from_file_as_json(&test_file)
            .expect("Failed to read structure from file");

        // Validate key fields
        assert_eq!(read_version.id, forge_version.id);
        assert_eq!(read_version.type_, forge_version.type_);
        assert_eq!(read_version.main_class, forge_version.main_class);
        assert_eq!(read_version.inherits_from, forge_version.inherits_from);
        assert_eq!(read_version.libraries.len(), forge_version.libraries.len());

        // Clean up
        fs::remove_file(&test_file).expect("Failed to remove test file");
        fs::remove_dir(&temp_dir).expect("Failed to remove test directory");
    }

    /// Tests BaseVersion creation from actual file
    #[test]
    fn test_real_file_loading() {
        use crate::craft_launcher::utils::file_operations::file_utils;
        use std::env;
        use std::fs;

        // Create temporary directory for the test
        let temp_dir = env::temp_dir().join("real_version_test");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");
        }

        // Create a realistic OptiFine version JSON
        let optifine_json_path = temp_dir.join("1.20.1-OptiFine.json");
        let optifine_json_content = r#"{
          "id": "1.20.1-OptiFine_HD_U_I5",
          "inheritsFrom": "1.20.1",
          "time": "2023-08-13T22:28:41+09:00",
          "releaseTime": "2023-08-13T22:28:41+09:00",
          "type": "release",
          "libraries": [
            {
              "name": "optifine:OptiFine:1.20.1_HD_U_I5"
            },
            {
              "name": "optifine:launchwrapper-of:2.3"
            }
          ],
          "mainClass": "net.minecraft.launchwrapper.Launch",
          "arguments": {
            "game": [
              "--tweakClass",
              "optifine.OptiFineTweaker"
            ]
          }
        }"#;

        fs::write(&optifine_json_path, optifine_json_content)
            .expect("Failed to write OptiFine JSON file");

        // Load the file
        let loaded_version: BaseVersion =
            file_utils::read_struct_from_file_as_json(&optifine_json_path)
                .expect("Failed to load OptiFine JSON");

        // Validate key fields
        assert_eq!(loaded_version.id, "1.20.1-OptiFine_HD_U_I5");
        assert_eq!(loaded_version.inherits_from, Some("1.20.1".to_string()));
        assert_eq!(
            loaded_version.main_class,
            "net.minecraft.launchwrapper.Launch"
        );
        assert_eq!(loaded_version.libraries.len(), 2);

        // Test game arguments
        if let Some(args) = loaded_version.arguments {
            if let Some(game_args) = args.game {
                assert_eq!(game_args.len(), 2);
                match &game_args[0] {
                    ArgumentValue::String(s) => assert_eq!(s, "--tweakClass"),
                    _ => panic!("Expected string argument"),
                }
                match &game_args[1] {
                    ArgumentValue::String(s) => assert_eq!(s, "optifine.OptiFineTweaker"),
                    _ => panic!("Expected string argument"),
                }
            } else {
                panic!("Game arguments should be present");
            }
        } else {
            panic!("Arguments should be present");
        }

        // Clean up
        fs::remove_file(&optifine_json_path).expect("Failed to remove OptiFine JSON file");
        fs::remove_dir(&temp_dir).expect("Failed to remove test directory");
    }
}
