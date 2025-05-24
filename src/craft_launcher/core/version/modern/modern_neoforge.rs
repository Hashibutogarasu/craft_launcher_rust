pub mod modern_neoforge {
    use serde::{Deserialize, Serialize};

    // NeoForge version information structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeoForgeVersion {
        pub id: String,
        pub time: String,
        #[serde(rename = "releaseTime")]
        pub release_time: String,
        #[serde(rename = "type")]
        pub version_type: String,
        #[serde(rename = "mainClass")]
        pub main_class: String,
        #[serde(rename = "inheritsFrom")]
        pub inherits_from: String,
        pub arguments: NeoForgeArguments,
        pub libraries: Vec<NeoForgeLibrary>,
    }

    // Arguments structure containing game and JVM arguments
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeoForgeArguments {
        pub game: Vec<String>,
        pub jvm: Vec<String>,
    }

    // Library structure with download information
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeoForgeLibrary {
        pub name: String,
        pub downloads: NeoForgeLibraryDownloads,
    }

    // Library downloads containing artifact information
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeoForgeLibraryDownloads {
        pub artifact: NeoForgeArtifact,
    }

    // Artifact information with download details
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeoForgeArtifact {
        pub sha1: String,
        pub size: u64,
        pub url: String,
        pub path: String,
    }

    // Parse NeoForge version file from a JSON string
    pub fn parse_neoforge_version(json_str: &str) -> Result<NeoForgeVersion, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::modern_neoforge::*;

    // Test for parsing NeoForge version JSON
    #[test]
    fn test_parse_neoforge_version() {
        // Sample NeoForge version JSON data
        let json_str = r#"
        {
            "id": "neoforge-21.5.66-beta",
            "time": "2025-05-11T23:35:42.001627397",
            "releaseTime": "2025-05-11T23:35:42.001627397",
            "type": "release",
            "mainClass": "cpw.mods.bootstraplauncher.BootstrapLauncher",
            "inheritsFrom": "1.21.5",
            "arguments": {
              "game": [
                "--fml.neoForgeVersion",
                "21.5.66-beta",
                "--fml.fmlVersion",
                "7.0.10"
              ],
              "jvm": [
                "-Djava.net.preferIPv6Addresses=system",
                "-DignoreList=client-extra,${version_name}.jar"
              ]
            },
            "libraries": [
              {
                "name": "net.neoforged.fancymodloader:earlydisplay:7.0.10",
                "downloads": {
                  "artifact": {
                    "sha1": "b492de98200980d046ad1bdcf0020971e92b45e5",
                    "size": 269921,
                    "url": "https://maven.neoforged.net/releases/net/neoforged/fancymodloader/earlydisplay/7.0.10/earlydisplay-7.0.10.jar",
                    "path": "net/neoforged/fancymodloader/earlydisplay/7.0.10/earlydisplay-7.0.10.jar"
                  }
                }
              }
            ]
        }
        "#;

        // Parse the JSON string
        let result = parse_neoforge_version(json_str);

        // Check if parsing was successful
        assert!(result.is_ok(), "Failed to parse NeoForge version JSON");

        let version = result.unwrap();

        // Verify the parsed values
        assert_eq!(version.id, "neoforge-21.5.66-beta");
        assert_eq!(version.time, "2025-05-11T23:35:42.001627397");
        assert_eq!(version.release_time, "2025-05-11T23:35:42.001627397");
        assert_eq!(version.version_type, "release");
        assert_eq!(
            version.main_class,
            "cpw.mods.bootstraplauncher.BootstrapLauncher"
        );
        assert_eq!(version.inherits_from, "1.21.5");

        // Verify arguments
        assert_eq!(version.arguments.game.len(), 4);
        assert_eq!(version.arguments.game[0], "--fml.neoForgeVersion");
        assert_eq!(version.arguments.game[1], "21.5.66-beta");

        assert_eq!(version.arguments.jvm.len(), 2);
        assert_eq!(
            version.arguments.jvm[0],
            "-Djava.net.preferIPv6Addresses=system"
        );

        // Verify libraries
        assert_eq!(version.libraries.len(), 1);
        assert_eq!(
            version.libraries[0].name,
            "net.neoforged.fancymodloader:earlydisplay:7.0.10"
        );

        // Verify artifact details
        let artifact = &version.libraries[0].downloads.artifact;
        assert_eq!(artifact.sha1, "b492de98200980d046ad1bdcf0020971e92b45e5");
        assert_eq!(artifact.size, 269921);
        assert_eq!(
            artifact.url,
            "https://maven.neoforged.net/releases/net/neoforged/fancymodloader/earlydisplay/7.0.10/earlydisplay-7.0.10.jar"
        );
        assert_eq!(
            artifact.path,
            "net/neoforged/fancymodloader/earlydisplay/7.0.10/earlydisplay-7.0.10.jar"
        );
    }
}
