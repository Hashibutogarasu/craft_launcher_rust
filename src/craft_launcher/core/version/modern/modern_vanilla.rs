/// Version information for modern vanilla Minecraft (1.13+)
pub mod modern_vanilla {
    use crate::craft_launcher::core::version::base_version::BaseVersion;
    use serde::de::{self, MapAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize};
    use std::collections::HashSet;
    use std::fmt;

    /// Represents a modern vanilla Minecraft version JSON structure (1.13 and newer)
    /// This extends the BaseVersion with additional fields specific to modern vanilla versions
    #[derive(Debug, Clone, Serialize)]
    pub struct ModernVanillaVersion {
        // BaseVersion fields
        #[serde(flatten)]
        pub base: BaseVersion,

        /// The assets index information
        #[serde(rename = "assetIndex")]
        pub asset_index: AssetIndex,

        /// The assets directory name
        pub assets: String,

        /// The minimum launcher version required to run this version
        #[serde(rename = "minimumLauncherVersion")]
        pub minimum_launcher_version: i32,

        /// Download information for client and server JARs
        pub downloads: Downloads,

        /// Java version requirements
        #[serde(rename = "javaVersion")]
        pub java_version: JavaVersion,

        /// Compliance level for the game
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compliance_level: Option<i32>,
    }

    /// Asset index information for a Minecraft version
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AssetIndex {
        /// The ID of the asset index
        pub id: String,

        /// The SHA-1 hash of the asset index file
        pub sha1: String,

        /// The size of the asset index file
        pub size: i64,

        /// The total size of all assets
        #[serde(rename = "totalSize")]
        pub total_size: i64,

        /// The URL to download the asset index
        pub url: String,
    }

    /// Download information for a Minecraft version
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Downloads {
        /// Client JAR information
        pub client: DownloadEntry,

        /// Server JAR information
        pub server: DownloadEntry,

        /// Client mappings information
        #[serde(rename = "client_mappings", skip_serializing_if = "Option::is_none")]
        pub client_mappings: Option<DownloadEntry>,

        /// Server mappings information
        #[serde(rename = "server_mappings", skip_serializing_if = "Option::is_none")]
        pub server_mappings: Option<DownloadEntry>,

        /// Windows server executable information
        #[serde(rename = "windows_server", skip_serializing_if = "Option::is_none")]
        pub windows_server: Option<DownloadEntry>,
    }

    /// A single download entry with hash, size and URL
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DownloadEntry {
        /// The SHA-1 hash of the file
        pub sha1: String,

        /// The size of the file in bytes
        pub size: i64,

        /// The URL to download the file
        pub url: String,
    }

    /// Java version requirements
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JavaVersion {
        /// The Java component to use (e.g., "java-runtime-alpha")
        pub component: String,

        /// The major version of Java required
        #[serde(rename = "majorVersion")]
        pub major_version: i32,
    } // Implementation of custom deserialization for ModernVanillaVersion
    impl<'de> Deserialize<'de> for ModernVanillaVersion {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Define a visitor struct for custom deserialization
            struct ModernVanillaVersionVisitor;

            impl<'de> Visitor<'de> for ModernVanillaVersionVisitor {
                type Value = ModernVanillaVersion;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a valid modern Minecraft version (1.13+)")
                }
                fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    // This will store all the field keys we see
                    let mut fields = HashSet::new();

                    // Using a more stable approach with standard collections
                    let mut id = None;
                    let mut time = None;
                    let mut release_time = None;
                    let mut type_ = None;
                    let mut main_class = None;
                    let mut libraries = None;
                    let mut inherits_from = None;
                    let mut arguments = None;
                    let mut minecraft_arguments = None;
                    let mut logging = None;
                    let mut asset_index = None;
                    let mut assets = None;
                    let mut minimum_launcher_version = None;
                    let mut downloads = None;
                    let mut java_version = None;
                    let mut compliance_level = None;

                    // Process fields one by one to check version format
                    while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                        fields.insert(key.clone());

                        // Store each field's value
                        match key.as_str() {
                            "id" => {
                                id = Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "time" => {
                                time =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "releaseTime" => {
                                release_time =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "type" => {
                                type_ =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "mainClass" => {
                                main_class =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "libraries" => {
                                libraries =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "inheritsFrom" => {
                                inherits_from =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "arguments" => {
                                arguments =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "minecraftArguments" => {
                                minecraft_arguments =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "logging" => {
                                logging =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "assetIndex" => {
                                asset_index =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "assets" => {
                                assets =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "minimumLauncherVersion" => {
                                minimum_launcher_version =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "downloads" => {
                                downloads =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "javaVersion" => {
                                java_version =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            "complianceLevel" => {
                                compliance_level =
                                    Some(serde_json::from_value(value).map_err(de::Error::custom)?)
                            }
                            _ => {} // Ignore unknown fields
                        }
                    }

                    // The key check - if "minecraftArguments" exists but "arguments" does not,
                    // this is a legacy version format (pre-1.13) and we should reject it
                    if fields.contains("minecraftArguments") && !fields.contains("arguments") {
                        return Err(de::Error::custom(
                            "Legacy version format detected. Use LegacyVanillaVersion instead.",
                        ));
                    }

                    // If "arguments" doesn't exist, it's not a valid modern version
                    if !fields.contains("arguments") {
                        return Err(de::Error::custom(
                            "Not a valid modern version format. Missing 'arguments' field.",
                        ));
                    }

                    // Construct the BaseVersion
                    let base = BaseVersion {
                        id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                        time: time.ok_or_else(|| de::Error::missing_field("time"))?,
                        release_time: release_time
                            .ok_or_else(|| de::Error::missing_field("releaseTime"))?,
                        type_: type_.ok_or_else(|| de::Error::missing_field("type"))?,
                        main_class: main_class
                            .ok_or_else(|| de::Error::missing_field("mainClass"))?,
                        libraries: libraries.unwrap_or_default(),
                        inherits_from,
                        arguments,
                        minecraft_arguments,
                        logging,
                    };

                    // Construct the ModernVanillaVersion
                    Ok(ModernVanillaVersion {
                        base,
                        asset_index: asset_index
                            .ok_or_else(|| de::Error::missing_field("assetIndex"))?,
                        assets: assets.ok_or_else(|| de::Error::missing_field("assets"))?,
                        minimum_launcher_version: minimum_launcher_version
                            .ok_or_else(|| de::Error::missing_field("minimumLauncherVersion"))?,
                        downloads: downloads
                            .ok_or_else(|| de::Error::missing_field("downloads"))?,
                        java_version: java_version
                            .ok_or_else(|| de::Error::missing_field("javaVersion"))?,
                        compliance_level,
                    })
                }
            }

            // Use our visitor to deserialize
            deserializer.deserialize_map(ModernVanillaVersionVisitor)
        }
    }

    /// Implementation of conversion methods for ModernVanillaVersion
    impl ModernVanillaVersion {
        /// Converts a ModernVanillaVersion to a BaseVersion
        pub fn to_base_version(&self) -> BaseVersion {
            self.base.clone()
        }

        /// Creates a new ModernVanillaVersion from a BaseVersion and additional parameters
        pub fn from_base_version(
            base: BaseVersion,
            asset_index: AssetIndex,
            assets: String,
            minimum_launcher_version: i32,
            downloads: Downloads,
            java_version: JavaVersion,
            compliance_level: Option<i32>,
        ) -> Self {
            Self {
                base,
                asset_index,
                assets,
                minimum_launcher_version,
                downloads,
                java_version,
                compliance_level,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::craft_launcher::core::version::{
        base_version::BaseVersion,
        modern::modern_vanilla::modern_vanilla::{
            AssetIndex, DownloadEntry, Downloads, JavaVersion, ModernVanillaVersion,
        },
    };
    /// Tests successful parsing of a modern Minecraft version (1.13+)
    #[test]
    fn test_modern_version_parsing() {
        // Example JSON for a modern version (e.g., 1.21.1)
        // Using a simplified version with only the essential fields for testing
        let modern_json = r#"{
    "arguments": {
        "game": ["--username", "${auth_player_name}"],
        "jvm": ["-Xmx2G", "-XX:+UnlockExperimentalVMOptions"]
    },
    "assetIndex": {
        "id": "17",
        "sha1": "6eed1aafbdcde3797f19273b37ea1f4cf94c55d2",
        "size": 448666,
        "totalSize": 805914804,
        "url": "https://piston-meta.mojang.com/v1/packages/6eed1aafbdcde3797f19273b37ea1f4cf94c55d2/17.json"
    },
    "assets": "17",
    "complianceLevel": 1,
    "downloads": {
        "client": {
            "sha1": "30c73b1c5da787909b2f73340419fdf13b9def88",
            "size": 26836906,
            "url": "https://piston-data.mojang.com/v1/objects/30c73b1c5da787909b2f73340419fdf13b9def88/client.jar"
        },
        "client_mappings": {
            "sha1": "2244b6f072256667bcd9a73df124d6c58de77992",
            "size": 9598610,
            "url": "https://piston-data.mojang.com/v1/objects/2244b6f072256667bcd9a73df124d6c58de77992/client.txt"
        },
        "server": {
            "sha1": "59353fb40c36d304f2035d51e7d6e6baa98dc05c",
            "size": 51627615,
            "url": "https://piston-data.mojang.com/v1/objects/59353fb40c36d304f2035d51e7d6e6baa98dc05c/server.jar"
        },
        "server_mappings": {
            "sha1": "03f8985492bda0afc0898465341eb0acef35f570",
            "size": 7456063,
            "url": "https://piston-data.mojang.com/v1/objects/03f8985492bda0afc0898465341eb0acef35f570/server.txt"
        }
    },
    "id": "1.21.1",
    "javaVersion": {
        "component": "java-runtime-delta",
        "majorVersion": 21
    },
    "libraries": [],
    "mainClass": "net.minecraft.client.main.Main",
    "minimumLauncherVersion": 21,
    "releaseTime": "2024-08-08T12:24:45+00:00",
    "time": "2024-08-08T12:24:45+00:00",
    "type": "release"
}"#;

        // This should parse successfully
        let result = serde_json::from_str::<ModernVanillaVersion>(modern_json);
        assert!(
            result.is_ok(),
            "Failed to parse modern version: {:?}",
            result.err()
        );

        let modern_version = result.unwrap();

        // Validate key fields
        assert_eq!(modern_version.base.id, "1.21.1");
        assert_eq!(modern_version.base.type_, "release");
        assert!(modern_version.base.minecraft_arguments.is_none());
        assert!(modern_version.base.arguments.is_some());
        assert_eq!(modern_version.assets, "17");
        assert_eq!(modern_version.minimum_launcher_version, 21);
        assert_eq!(modern_version.java_version.component, "java-runtime-delta");
        assert_eq!(modern_version.java_version.major_version, 21);
        assert!(modern_version.downloads.client_mappings.is_some());
        assert!(modern_version.downloads.server_mappings.is_some());
    }
    /// Tests that legacy versions (pre 1.13) fail to parse as ModernVanillaVersion
    #[test]
    fn test_legacy_version_parsing_fails() {
        // Example JSON for a legacy version (e.g., 1.7.10) - simplified for testing
        let legacy_json = r#"{
    "assetIndex": {
        "id": "1.7.10",
        "sha1": "1863782e33ce7b584fc45b037325a1964e095d3e",
        "size": 72996,
        "totalSize": 112396854,
        "url": "https://launchermeta.mojang.com/v1/packages/1863782e33ce7b584fc45b037325a1964e095d3e/1.7.10.json"
    },
    "assets": "1.7.10",
    "complianceLevel": 0,
    "downloads": {
        "client": {
            "sha1": "e80d9b3bf5085002218d4be59e668bac718abbc6",
            "size": 5256245,
            "url": "https://launcher.mojang.com/v1/objects/e80d9b3bf5085002218d4be59e668bac718abbc6/client.jar"
        },
        "server": {
            "sha1": "952438ac4e01b4d115c5fc38f891710c4941df29",
            "size": 9605030,
            "url": "https://launcher.mojang.com/v1/objects/952438ac4e01b4d115c5fc38f891710c4941df29/server.jar"
        }
    },
    "id": "1.7.10",
    "javaVersion": {
        "component": "jre-legacy",
        "majorVersion": 8
    },
    "libraries": [],
    "mainClass": "net.minecraft.client.main.Main",
    "minecraftArguments": "--username ${auth_player_name} --version ${version_name}",
    "minimumLauncherVersion": 13,
    "releaseTime": "2014-05-14T17:29:23+00:00",
    "time": "2014-05-14T17:29:23+00:00",
    "type": "release"
}"#;

        // This should fail to parse as ModernVanillaVersion
        let result = serde_json::from_str::<ModernVanillaVersion>(legacy_json);
        assert!(
            result.is_err(),
            "Legacy version incorrectly parsed as modern version"
        );

        let error = result.err().unwrap().to_string();
        assert!(
            error.contains("Legacy version format") || error.contains("minecraftArguments"),
            "Error should indicate legacy version format incompatibility: {}",
            error
        );
    }

    /// Tests conversion between BaseVersion and ModernVanillaVersion
    #[test]
    fn test_conversion_between_types() {
        // Create a minimal BaseVersion
        let base = BaseVersion {
            id: "test-1.21.1".to_string(),
            time: "2023-01-01T00:00:00Z".to_string(),
            release_time: "2023-01-01T00:00:00Z".to_string(),
            type_: "release".to_string(),
            main_class: "net.minecraft.client.main.Main".to_string(),
            libraries: Vec::new(),
            inherits_from: None,
            arguments: Some(
                crate::craft_launcher::core::version::base_version::Arguments {
                    game: Some(vec![]),
                    jvm: Some(vec![]),
                },
            ),
            minecraft_arguments: None,
            logging: None,
        };

        // Create asset index
        let asset_index = AssetIndex {
            id: "17".to_string(),
            sha1: "abcdef".to_string(),
            size: 1000,
            total_size: 10000,
            url: "https://example.com/assets".to_string(),
        };

        // Create download info
        let downloads = Downloads {
            client: DownloadEntry {
                sha1: "clienthash".to_string(),
                size: 5000,
                url: "https://example.com/client.jar".to_string(),
            },
            server: DownloadEntry {
                sha1: "serverhash".to_string(),
                size: 6000,
                url: "https://example.com/server.jar".to_string(),
            },
            client_mappings: Some(DownloadEntry {
                sha1: "clientmappingshash".to_string(),
                size: 7000,
                url: "https://example.com/client.txt".to_string(),
            }),
            server_mappings: Some(DownloadEntry {
                sha1: "servermappingshash".to_string(),
                size: 8000,
                url: "https://example.com/server.txt".to_string(),
            }),
            windows_server: None,
        };

        // Create java version
        let java_version = JavaVersion {
            component: "java-runtime-delta".to_string(),
            major_version: 21,
        };

        // Create ModernVanillaVersion
        let modern = ModernVanillaVersion::from_base_version(
            base.clone(),
            asset_index,
            "17".to_string(),
            21,
            downloads,
            java_version,
            Some(1),
        );

        // Convert back to BaseVersion
        let converted_base = modern.to_base_version();

        // Verify conversions worked correctly
        assert_eq!(converted_base.id, base.id);
        assert_eq!(converted_base.main_class, base.main_class);
        assert_eq!(converted_base.arguments.is_some(), true);
        assert_eq!(converted_base.minecraft_arguments.is_none(), true);
    }
}
