use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod modern_forge {
    use crate::library_struct::Library;

    use super::*;

    /// Structure representing the main Forge version configuration file
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ForgeVersion {
        #[serde(rename = "_comment", skip_serializing_if = "Option::is_none")]
        pub comment: Option<Vec<String>>,
        pub id: String,
        pub time: String,
        #[serde(rename = "releaseTime")]
        pub release_time: String,
        #[serde(rename = "inheritsFrom")]
        pub inherits_from: String,
        #[serde(rename = "type")]
        pub version_type: String,
        pub logging: HashMap<String, serde_json::Value>,
        #[serde(rename = "mainClass")]
        pub main_class: String,
        pub libraries: Vec<Library<Downloads>>,
        pub arguments: Arguments,
    }

    /// Structure representing download information for a library
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Downloads {
        pub artifact: Artifact,
    }

    /// Structure representing artifact information
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Artifact {
        pub path: String,
        pub url: String,
        pub sha1: String,
        pub size: u64,
    }

    /// Structure representing arguments for the game
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Arguments {
        pub game: Vec<String>,
        pub jvm: Vec<String>,
    }

    /// Parses a Forge version file from JSON
    pub fn parse_forge_version(json_data: &str) -> Result<ForgeVersion, serde_json::Error> {
        serde_json::from_str(json_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::modern_forge::modern_forge::parse_forge_version;

    /// Test that a valid Forge JSON can be correctly parsed to a ForgeVersion struct
    #[test]
    fn test_parse_forge_version() {
        // Sample Forge version JSON mimicking the real file structure
        let sample_json = r#"
            {
                "_comment": [
                    "Please do not automate the download and installation of Forge.",
                    "Our efforts are supported by ads from the download page."
                ],
                "id": "1.21.1-forge-52.1.1",
                "time": "2025-04-19T12:34:02+00:00",
                "releaseTime": "2025-04-19T12:34:02+00:00",
                "inheritsFrom": "1.21.1",
                "type": "release",
                "logging": {},
                "mainClass": "net.minecraftforge.bootstrap.ForgeBootstrap",
                "libraries": [
                    {
                        "name": "net.minecraftforge:forge:1.21.1-52.1.1:universal",
                        "downloads": {
                            "artifact": {
                                "path": "net/minecraftforge/forge/1.21.1-52.1.1/forge-1.21.1-52.1.1-universal.jar",
                                "url": "https://maven.minecraftforge.net/net/minecraftforge/forge/1.21.1-52.1.1/forge-1.21.1-52.1.1-universal.jar",
                                "sha1": "45c4d190e90f8f28f9ef92793367ff4e302a22b5",
                                "size": 2620063
                            }
                        }
                    }
                ],
                "arguments": {
                    "game": [
                        "--launchTarget",
                        "forge_client"
                    ],
                    "jvm": [
                        "-Djava.net.preferIPv6Addresses=system"
                    ]
                }
            }
            "#;

        // Parse the JSON string into a ForgeVersion struct
        let result = parse_forge_version(sample_json);

        // Check that the parse was successful
        assert!(result.is_ok(), "Failed to parse Forge version JSON");

        // Unwrap the result and verify some key fields
        let forge_version = result.unwrap();

        assert_eq!(forge_version.id, "1.21.1-forge-52.1.1");
        assert_eq!(forge_version.inherits_from, "1.21.1");
        assert_eq!(forge_version.version_type, "release");
        assert_eq!(
            forge_version.main_class,
            "net.minecraftforge.bootstrap.ForgeBootstrap"
        );

        // Verify comments
        assert!(forge_version.comment.is_some());
        let comments = forge_version.comment.unwrap();
        assert_eq!(comments.len(), 2);
        assert_eq!(
            comments[0],
            "Please do not automate the download and installation of Forge."
        );

        // Verify libraries
        assert_eq!(forge_version.libraries.len(), 1);
        let library = &forge_version.libraries[0];
        assert_eq!(
            library.name,
            "net.minecraftforge:forge:1.21.1-52.1.1:universal"
        );
        assert_eq!(library.downloads.as_ref().unwrap().artifact.size, 2620063);

        // Verify arguments
        assert_eq!(forge_version.arguments.game.len(), 2);
        assert_eq!(forge_version.arguments.jvm.len(), 1);
        assert_eq!(forge_version.arguments.game[0], "--launchTarget");
        assert_eq!(
            forge_version.arguments.jvm[0],
            "-Djava.net.preferIPv6Addresses=system"
        );
    }

    /// Test handling of invalid JSON input
    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{ "id": "invalid" "#; // Intentionally malformed JSON
        let result = parse_forge_version(invalid_json);
        assert!(result.is_err(), "Parser should reject invalid JSON");
    }

    /// Test parsing a more complex structure to ensure all fields are properly handled
    #[test]
    fn test_complex_structure() {
        let complex_json = r#"
            {
                "id": "1.21.1-forge-52.1.1",
                "time": "2025-04-19T12:34:02+00:00",
                "releaseTime": "2025-04-19T12:34:02+00:00",
                "inheritsFrom": "1.21.1",
                "type": "release",
                "logging": {
                    "client": {
                        "file": {
                            "id": "client-1.21.1.xml",
                            "url": "https://launcher.mojang.com/v1/objects/abc123/client-1.21.1.xml",
                            "sha1": "abc123",
                            "size": 1234
                        }
                    }
                },
                "mainClass": "net.minecraftforge.bootstrap.ForgeBootstrap",
                "libraries": [
                    {
                        "name": "net.minecraftforge:forge:1.21.1-52.1.1:universal",
                        "downloads": {
                            "artifact": {
                                "path": "net/minecraftforge/forge/1.21.1-52.1.1/forge-1.21.1-52.1.1-universal.jar",
                                "url": "https://maven.minecraftforge.net/net/minecraftforge/forge/1.21.1-52.1.1/forge-1.21.1-52.1.1-universal.jar",
                                "sha1": "45c4d190e90f8f28f9ef92793367ff4e302a22b5",
                                "size": 2620063
                            }
                        }
                    },
                    {
                        "name": "org.ow2.asm:asm:9.7.1",
                        "downloads": {
                            "artifact": {
                                "path": "org/ow2/asm/asm/9.7.1/asm-9.7.1.jar",
                                "url": "https://maven.minecraftforge.net/org/ow2/asm/asm/9.7.1/asm-9.7.1.jar",
                                "sha1": "f0ed132a49244b042cd0e15702ab9f2ce3cc8436",
                                "size": 126093
                            }
                        }
                    }
                ],
                "arguments": {
                    "game": [
                        "--launchTarget",
                        "forge_client"
                    ],
                    "jvm": [
                        "-Djava.net.preferIPv6Addresses=system",
                        "-Xmx2G"
                    ]
                }
            }
            "#;

        let result = parse_forge_version(complex_json);
        assert!(result.is_ok(), "Failed to parse complex Forge version JSON");

        let forge_version = result.unwrap();
        assert_eq!(forge_version.libraries.len(), 2);
        assert_eq!(forge_version.arguments.jvm.len(), 2);
        assert_eq!(forge_version.arguments.jvm[1], "-Xmx2G");
    }
}
