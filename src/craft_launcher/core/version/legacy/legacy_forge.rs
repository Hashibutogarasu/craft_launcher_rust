pub mod legacy_forge {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use crate::library_struct::Library;

    /// Structure that represents a legacy Forge version file
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LegacyForgeVersion {
        /// Comments from Forge developers
        #[serde(rename = "_comment_")]
        pub comments: Option<Vec<String>>,

        /// Unique identifier for this version
        pub id: String,

        /// Time when this version file was created
        pub time: String,

        /// Time when this version was released
        #[serde(rename = "releaseTime")]
        pub release_time: String,

        /// Type of the release (e.g. "release", "snapshot")
        #[serde(rename = "type")]
        pub release_type: String,

        /// The main class to be launched
        #[serde(rename = "mainClass")]
        pub main_class: String,

        /// The base version this forge version inherits from
        #[serde(rename = "inheritsFrom")]
        pub inherits_from: String,

        /// Logging configuration
        pub logging: HashMap<String, serde_json::Value>,

        /// Arguments to pass to Minecraft when launching
        #[serde(rename = "minecraftArguments")]
        pub minecraft_arguments: String,

        /// Libraries required by this version
        pub libraries: Vec<Library<Downloads>>,
    }

    /// Structure representing download information for libraries
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Downloads {
        /// Artifact download information
        pub artifact: Option<Artifact>,
    }

    /// Structure representing a single downloadable artifact
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Artifact {
        /// Relative path of the artifact
        pub path: String,

        /// Download URL
        pub url: String,

        /// SHA1 hash for integrity verification
        pub sha1: String,

        /// Size of the artifact in bytes
        pub size: u64,
    }

    /// Parse a legacy forge version from JSON
    pub fn parse_legacy_forge(json_content: &str) -> Result<LegacyForgeVersion, serde_json::Error> {
        serde_json::from_str(json_content)
    }
}

#[cfg(test)]
mod tests {
    use super::legacy_forge::*;

    /// Test parsing a mock legacy forge version JSON
    #[test]
    fn test_parse_legacy_forge() {
        // Mock JSON data representing a simplified forge version file
        let mock_json = r#"
        {
            "_comment_": [
                "Test comment"
            ],
            "id": "1.12.2-forge-14.23.5.2860",
            "time": "2021-12-13T04:40:03+00:00",
            "releaseTime": "2021-12-13T04:40:03+00:00",
            "type": "release",
            "mainClass": "net.minecraft.launchwrapper.Launch",
            "inheritsFrom": "1.12.2",
            "logging": {},
            "minecraftArguments": "--username ${auth_player_name} --version ${version_name} --tweakClass net.minecraftforge.fml.common.launcher.FMLTweaker",
            "libraries": [
                {
                    "name": "net.minecraftforge:forge:1.12.2-14.23.5.2860",
                    "downloads": {
                        "artifact": {
                            "path": "net/minecraftforge/forge/1.12.2-14.23.5.2860/forge-1.12.2-14.23.5.2860.jar",
                            "url": "https://maven.example.com/path/to/forge.jar",
                            "sha1": "029250575d3aa2cf80b56dffb66238a1eeaea2ac",
                            "size": 4466148
                        }
                    }
                },
                {
                    "name": "org.ow2.asm:asm-debug-all:5.2",
                    "downloads": {
                        "artifact": {
                            "path": "org/ow2/asm/asm-debug-all/5.2/asm-debug-all-5.2.jar",
                            "url": "https://maven.example.com/path/to/asm.jar",
                            "sha1": "3354e11e2b34215f06dab629ab88e06aca477c19",
                            "size": 387903
                        }
                    }
                }
            ]
        }"#;

        // Parse the mock JSON
        let result = parse_legacy_forge(mock_json);

        // Assert that parsing was successful
        assert!(result.is_ok());

        // Get the parsed version
        let forge_version = result.unwrap();

        // Validate basic fields
        assert_eq!(forge_version.id, "1.12.2-forge-14.23.5.2860");
        assert_eq!(
            forge_version.main_class,
            "net.minecraft.launchwrapper.Launch"
        );
        assert_eq!(forge_version.inherits_from, "1.12.2");
        assert_eq!(forge_version.release_type, "release");

        // Check that we have the expected number of libraries
        assert_eq!(forge_version.libraries.len(), 2);

        // Validate first library
        let first_lib = &forge_version.libraries[0];
        assert_eq!(
            first_lib.name,
            "net.minecraftforge:forge:1.12.2-14.23.5.2860"
        );

        // Validate first library downloads
        let downloads = first_lib.downloads.as_ref().unwrap();
        let artifact = downloads.artifact.as_ref().unwrap();
        assert_eq!(artifact.sha1, "029250575d3aa2cf80b56dffb66238a1eeaea2ac");
        assert_eq!(artifact.size, 4466148);
    }
}
