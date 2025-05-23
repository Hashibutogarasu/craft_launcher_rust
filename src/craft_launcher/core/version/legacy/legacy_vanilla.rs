/// Version information for legacy vanilla
pub mod legacy_vanilla {
    use crate::craft_launcher::core::version::base_version::BaseVersion;
    use serde::de::{self, MapAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize};
    use std::collections::HashSet;
    use std::fmt;

    /// Represents a legacy vanilla Minecraft version JSON structure (pre 1.13)
    /// This extends the BaseVersion with additional fields specific to legacy vanilla versions
    #[derive(Debug, Clone, Serialize)]
    pub struct LegacyVanillaVersion {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "javaVersion")]
        pub java_version: Option<JavaVersion>,

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

        /// Windows server executable information
        #[serde(skip_serializing_if = "Option::is_none", rename = "windows_server")]
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
        /// The Java component to use (e.g., "jre-legacy")
        pub component: String,

        /// The major version of Java required
        #[serde(rename = "majorVersion")]
        pub major_version: i32,
    }

    // Implementation of custom deserialization for LegacyVanillaVersion
    impl<'de> Deserialize<'de> for LegacyVanillaVersion {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Define a visitor struct for custom deserialization
            struct LegacyVanillaVersionVisitor;

            impl<'de> Visitor<'de> for LegacyVanillaVersionVisitor {
                type Value = LegacyVanillaVersion;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a valid legacy Minecraft version (pre 1.13)")
                }

                fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    // This will store all the field keys we see
                    let mut fields = HashSet::new();

                    // Store original map values to rebuild the structure
                    let mut map_values = serde_json::Map::new();

                    // Iterate through all fields to detect modern version format
                    while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                        fields.insert(key.clone());
                        map_values.insert(key, value);
                    }

                    // The key check - if "arguments" exists, this is a modern version format (1.13+)
                    // and we should reject it
                    if fields.contains("arguments") {
                        return Err(de::Error::custom(
                            "Modern version format (1.13+) detected with 'arguments' field, incompatible with LegacyVanillaVersion",
                        ));
                    }

                    // If no "minecraftArguments" exists, this is likely not a valid legacy version
                    if !fields.contains("minecraftArguments") {
                        return Err(de::Error::custom(
                            "Missing required 'minecraftArguments' field for legacy version format",
                        ));
                    }

                    // Now deserialize the data using the standard process for the inner struct
                    #[derive(Deserialize)]
                    struct InnerLegacyVanillaVersion {
                        #[serde(flatten)]
                        base: BaseVersion,

                        #[serde(rename = "assetIndex")]
                        asset_index: AssetIndex,

                        assets: String,

                        #[serde(rename = "minimumLauncherVersion")]
                        minimum_launcher_version: i32,

                        downloads: Downloads,

                        #[serde(skip_serializing_if = "Option::is_none", rename = "javaVersion")]
                        java_version: Option<JavaVersion>,

                        #[serde(skip_serializing_if = "Option::is_none")]
                        compliance_level: Option<i32>,
                    }

                    // Convert back to JSON string
                    let json_data = serde_json::Value::Object(map_values);

                    // Deserialize
                    let inner = InnerLegacyVanillaVersion::deserialize(json_data)
                        .map_err(de::Error::custom)?;

                    Ok(LegacyVanillaVersion {
                        base: inner.base,
                        asset_index: inner.asset_index,
                        assets: inner.assets,
                        minimum_launcher_version: inner.minimum_launcher_version,
                        downloads: inner.downloads,
                        java_version: inner.java_version,
                        compliance_level: inner.compliance_level,
                    })
                }
            }

            // Use our visitor to deserialize
            deserializer.deserialize_map(LegacyVanillaVersionVisitor)
        }
    }

    /// Implementation of conversion methods for LegacyVanillaVersion
    impl LegacyVanillaVersion {
        /// Converts a LegacyVanillaVersion to a BaseVersion
        pub fn to_base_version(&self) -> BaseVersion {
            self.base.clone()
        }

        /// Creates a new LegacyVanillaVersion from a BaseVersion and additional parameters
        pub fn from_base_version(
            base: BaseVersion,
            asset_index: AssetIndex,
            assets: String,
            minimum_launcher_version: i32,
            downloads: Downloads,
            java_version: Option<JavaVersion>,
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
        legacy::legacy_vanilla::legacy_vanilla::{
            AssetIndex, DownloadEntry, Downloads, JavaVersion, LegacyVanillaVersion,
        },
    };

    /// Tests successful parsing of a legacy Minecraft version (pre 1.13)
    #[test]
    fn test_legacy_version_parsing() {
        // Example JSON for a legacy version (e.g., 1.7.10)
        let legacy_json = r#"
            {
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
        },
        "windows_server": {
            "sha1": "a79b91ef69b9b4af63d1c7007f60259106869b21",
            "size": 9999270,
            "url": "https://launcher.mojang.com/v1/objects/a79b91ef69b9b4af63d1c7007f60259106869b21/windows_server.exe"
        }
    },
    "id": "1.7.10",
    "javaVersion": {
        "component": "jre-legacy",
        "majorVersion": 8
    },
    "libraries": [
        {
            "downloads": {
                "artifact": {
                    "path": "com/mojang/netty/1.8.8/netty-1.8.8.jar",
                    "sha1": "0a796914d1c8a55b4da9f4a8856dd9623375d8bb",
                    "size": 15966,
                    "url": "https://libraries.minecraft.net/com/mojang/netty/1.8.8/netty-1.8.8.jar"
                }
            },
            "name": "com.mojang:netty:1.8.8"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/lwjgl_util/2.9.1/lwjgl_util-2.9.1.jar",
                    "sha1": "290d7ba8a1bd9566f5ddf16ad06f09af5ec9b20e",
                    "size": 173909,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl_util/2.9.1/lwjgl_util-2.9.1.jar"
                }
            },
            "name": "org.lwjgl.lwjgl:lwjgl_util:2.9.1"
        },
        {
            "downloads": {
                "classifiers": {
                    "natives-linux": {
                        "path": "org/lwjgl/lwjgl/lwjgl-platform/2.9.1/lwjgl-platform-2.9.1-natives-linux.jar",
                        "sha1": "aa9aae879af8eb378e22cfc64db56ec2ca9a44d1",
                        "size": 571424,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.1/lwjgl-platform-2.9.1-natives-linux.jar"
                    },
                    "natives-osx": {
                        "path": "org/lwjgl/lwjgl/lwjgl-platform/2.9.1/lwjgl-platform-2.9.1-natives-osx.jar",
                        "sha1": "2d12c83fdfbc04ecabf02c7bc8cc54d034f0daac",
                        "size": 527196,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.1/lwjgl-platform-2.9.1-natives-osx.jar"
                    },
                    "natives-windows": {
                        "path": "org/lwjgl/lwjgl/lwjgl-platform/2.9.1/lwjgl-platform-2.9.1-natives-windows.jar",
                        "sha1": "4c517eca808522457dd95ee8fc1fbcdbb602efbe",
                        "size": 611334,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.1/lwjgl-platform-2.9.1-natives-windows.jar"
                    }
                }
            },
            "extract": {
                "exclude": [
                    "META-INF/"
                ]
            },
            "name": "org.lwjgl.lwjgl:lwjgl-platform:2.9.1",
            "natives": {
                "linux": "natives-linux",
                "osx": "natives-osx",
                "windows": "natives-windows"
            }
        },
        {
            "downloads": {
                "classifiers": {
                    "natives-linux": {
                        "path": "net/java/jinput/jinput-platform/2.0.5/jinput-platform-2.0.5-natives-linux.jar",
                        "sha1": "7ff832a6eb9ab6a767f1ade2b548092d0fa64795",
                        "size": 10362,
                        "url": "https://libraries.minecraft.net/net/java/jinput/jinput-platform/2.0.5/jinput-platform-2.0.5-natives-linux.jar"
                    },
                    "natives-osx": {
                        "path": "net/java/jinput/jinput-platform/2.0.5/jinput-platform-2.0.5-natives-osx.jar",
                        "sha1": "53f9c919f34d2ca9de8c51fc4e1e8282029a9232",
                        "size": 12186,
                        "url": "https://libraries.minecraft.net/net/java/jinput/jinput-platform/2.0.5/jinput-platform-2.0.5-natives-osx.jar"
                    },
                    "natives-windows": {
                        "path": "net/java/jinput/jinput-platform/2.0.5/jinput-platform-2.0.5-natives-windows.jar",
                        "sha1": "385ee093e01f587f30ee1c8a2ee7d408fd732e16",
                        "size": 155179,
                        "url": "https://libraries.minecraft.net/net/java/jinput/jinput-platform/2.0.5/jinput-platform-2.0.5-natives-windows.jar"
                    }
                }
            },
            "extract": {
                "exclude": [
                    "META-INF/"
                ]
            },
            "name": "net.java.jinput:jinput-platform:2.0.5",
            "natives": {
                "linux": "natives-linux",
                "osx": "natives-osx",
                "windows": "natives-windows"
            }
        },
        {
            "downloads": {
                "artifact": {
                    "path": "tv/twitch/twitch/5.16/twitch-5.16.jar",
                    "sha1": "1f55f009c61637c10c0acfb8b5ffc600f30044b4",
                    "size": 52315,
                    "url": "https://libraries.minecraft.net/tv/twitch/twitch/5.16/twitch-5.16.jar"
                }
            },
            "name": "tv.twitch:twitch:5.16"
        },
        {
            "downloads": {
                "classifiers": {
                    "natives-osx": {
                        "path": "tv/twitch/twitch-platform/5.16/twitch-platform-5.16-natives-osx.jar",
                        "sha1": "62503ee712766cf77f97252e5902786fd834b8c5",
                        "size": 418331,
                        "url": "https://libraries.minecraft.net/tv/twitch/twitch-platform/5.16/twitch-platform-5.16-natives-osx.jar"
                    },
                    "natives-windows-32": {
                        "path": "tv/twitch/twitch-platform/5.16/twitch-platform-5.16-natives-windows-32.jar",
                        "sha1": "7c6affe439099806a4f552da14c42f9d643d8b23",
                        "size": 386792,
                        "url": "https://libraries.minecraft.net/tv/twitch/twitch-platform/5.16/twitch-platform-5.16-natives-windows-32.jar"
                    },
                    "natives-windows-64": {
                        "path": "tv/twitch/twitch-platform/5.16/twitch-platform-5.16-natives-windows-64.jar",
                        "sha1": "39d0c3d363735b4785598e0e7fbf8297c706a9f9",
                        "size": 463390,
                        "url": "https://libraries.minecraft.net/tv/twitch/twitch-platform/5.16/twitch-platform-5.16-natives-windows-64.jar"
                    }
                }
            },
            "extract": {
                "exclude": [
                    "META-INF/"
                ]
            },
            "name": "tv.twitch:twitch-platform:5.16",
            "natives": {
                "linux": "natives-linux",
                "osx": "natives-osx",
                "windows": "natives-windows-${arch}"
            },
            "rules": [
                {
                    "action": "allow"
                },
                {
                    "action": "disallow",
                    "os": {
                        "name": "linux"
                    }
                }
            ]
        },
        {
            "downloads": {
                "classifiers": {
                    "natives-windows-32": {
                        "path": "tv/twitch/twitch-external-platform/4.5/twitch-external-platform-4.5-natives-windows-32.jar",
                        "sha1": "18215140f010c05b9f86ef6f0f8871954d2ccebf",
                        "size": 5654047,
                        "url": "https://libraries.minecraft.net/tv/twitch/twitch-external-platform/4.5/twitch-external-platform-4.5-natives-windows-32.jar"
                    },
                    "natives-windows-64": {
                        "path": "tv/twitch/twitch-external-platform/4.5/twitch-external-platform-4.5-natives-windows-64.jar",
                        "sha1": "c3cde57891b935d41b6680a9c5e1502eeab76d86",
                        "size": 7457619,
                        "url": "https://libraries.minecraft.net/tv/twitch/twitch-external-platform/4.5/twitch-external-platform-4.5-natives-windows-64.jar"
                    }
                }
            },
            "extract": {
                "exclude": [
                    "META-INF/"
                ]
            },
            "name": "tv.twitch:twitch-external-platform:4.5",
            "natives": {
                "windows": "natives-windows-${arch}"
            },
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "windows"
                    }
                }
            ]
        }
    ],
    "logging": {
        "client": {
            "argument": "-Dlog4j.configurationFile=${path}",
            "file": {
                "id": "client-1.7.xml",
                "sha1": "50c9cc4af6d853d9fc137c84bcd153e2bd3a9a82",
                "size": 966,
                "url": "https://launcher.mojang.com/v1/objects/50c9cc4af6d853d9fc137c84bcd153e2bd3a9a82/client-1.7.xml"
            },
            "type": "log4j2-xml"
        }
    },
    "mainClass": "net.minecraft.client.main.Main",
    "minecraftArguments": "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root} --assetIndex ${assets_index_name} --uuid ${auth_uuid} --accessToken ${auth_access_token} --userProperties ${user_properties} --userType ${user_type}",
    "minimumLauncherVersion": 13,
    "releaseTime": "2014-05-14T17:29:23+00:00",
    "time": "2014-05-14T17:29:23+00:00",
    "type": "release"
}"#;
        // This should parse successfully
        let result = serde_json::from_str::<LegacyVanillaVersion>(legacy_json);
        assert!(
            result.is_ok(),
            "Failed to parse legacy version: {:?}",
            result.err()
        );

        let legacy_version = result.unwrap();

        // Validate key fields
        assert_eq!(legacy_version.base.id, "1.7.10");
        assert_eq!(legacy_version.base.type_, "release");
        assert!(legacy_version.base.minecraft_arguments.is_some());
        assert!(legacy_version.base.arguments.is_none());
        assert_eq!(legacy_version.assets, "1.7.10");
        assert_eq!(legacy_version.minimum_launcher_version, 13);
        assert!(legacy_version.java_version.is_some());
        assert_eq!(
            legacy_version.java_version.as_ref().unwrap().component,
            "jre-legacy"
        );
        assert_eq!(
            legacy_version.java_version.as_ref().unwrap().major_version,
            8
        );
    }

    /// Tests that modern versions (1.13+) fail to parse as LegacyVanillaVersion
    #[test]
    fn test_modern_version_parsing_fails() {
        // Example JSON for a modern version (e.g., 1.21.1)
        let modern_json = r#"
            {
    "arguments": {
        "game": [
            "--username",
            "${auth_player_name}",
            "--version",
            "${version_name}",
            "--gameDir",
            "${game_directory}",
            "--assetsDir",
            "${assets_root}",
            "--assetIndex",
            "${assets_index_name}",
            "--uuid",
            "${auth_uuid}",
            "--accessToken",
            "${auth_access_token}",
            "--clientId",
            "${clientid}",
            "--xuid",
            "${auth_xuid}",
            "--userType",
            "${user_type}",
            "--versionType",
            "${version_type}",
            {
                "rules": [
                    {
                        "action": "allow",
                        "features": {
                            "is_demo_user": true
                        }
                    }
                ],
                "value": "--demo"
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "features": {
                            "has_custom_resolution": true
                        }
                    }
                ],
                "value": [
                    "--width",
                    "${resolution_width}",
                    "--height",
                    "${resolution_height}"
                ]
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "features": {
                            "has_quick_plays_support": true
                        }
                    }
                ],
                "value": [
                    "--quickPlayPath",
                    "${quickPlayPath}"
                ]
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "features": {
                            "is_quick_play_singleplayer": true
                        }
                    }
                ],
                "value": [
                    "--quickPlaySingleplayer",
                    "${quickPlaySingleplayer}"
                ]
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "features": {
                            "is_quick_play_multiplayer": true
                        }
                    }
                ],
                "value": [
                    "--quickPlayMultiplayer",
                    "${quickPlayMultiplayer}"
                ]
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "features": {
                            "is_quick_play_realms": true
                        }
                    }
                ],
                "value": [
                    "--quickPlayRealms",
                    "${quickPlayRealms}"
                ]
            }
        ],
        "jvm": [
            {
                "rules": [
                    {
                        "action": "allow",
                        "os": {
                            "name": "osx"
                        }
                    }
                ],
                "value": [
                    "-XstartOnFirstThread"
                ]
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "os": {
                            "name": "windows"
                        }
                    }
                ],
                "value": "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump"
            },
            {
                "rules": [
                    {
                        "action": "allow",
                        "os": {
                            "arch": "x86"
                        }
                    }
                ],
                "value": "-Xss1M"
            },
            "-Djava.library.path=${natives_directory}",
            "-Djna.tmpdir=${natives_directory}",
            "-Dorg.lwjgl.system.SharedLibraryExtractPath=${natives_directory}",
            "-Dio.netty.native.workdir=${natives_directory}",
            "-Dminecraft.launcher.brand=${launcher_name}",
            "-Dminecraft.launcher.version=${launcher_version}",
            "-cp",
            "${classpath}"
        ]
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
    "libraries": [
        {
            "downloads": {
                "artifact": {
                    "path": "ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar",
                    "sha1": "1227f9e0666314f9de41477e3ec277e542ed7f7b",
                    "size": 1330045,
                    "url": "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar"
                }
            },
            "name": "ca.weblite:java-objc-bridge:1.1",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "osx"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "com/github/oshi/oshi-core/6.4.10/oshi-core-6.4.10.jar",
                    "sha1": "b1d8ab82d11d92fd639b56d639f8f46f739dd5fa",
                    "size": 979212,
                    "url": "https://libraries.minecraft.net/com/github/oshi/oshi-core/6.4.10/oshi-core-6.4.10.jar"
                }
            },
            "name": "com.github.oshi:oshi-core:6.4.10"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "com/google/code/gson/gson/2.10.1/gson-2.10.1.jar",
                    "sha1": "b3add478d4382b78ea20b1671390a858002feb6c",
                    "size": 283367,
                    "url": "https://libraries.minecraft.net/com/google/code/gson/gson/2.10.1/gson-2.10.1.jar"
                }
            },
            "name": "com.google.code.gson:gson:2.10.1"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar",
                    "sha1": "1dcf1de382a0bf95a3d8b0849546c88bac1292c9",
                    "size": 4617,
                    "url": "https://libraries.minecraft.net/com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar"
                }
            },
            "name": "com.google.guava:failureaccess:1.0.1"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "io/netty/netty-transport-native-epoll/4.1.97.Final/netty-transport-native-epoll-4.1.97.Final-linux-x86_64.jar",
                    "sha1": "54188f271e388e7f313aea995e82f58ce2cdb809",
                    "size": 38954,
                    "url": "https://libraries.minecraft.net/io/netty/netty-transport-native-epoll/4.1.97.Final/netty-transport-native-epoll-4.1.97.Final-linux-x86_64.jar"
                }
            },
            "name": "io.netty:netty-transport-native-epoll:4.1.97.Final:linux-x86_64",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "linux"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "io/netty/netty-transport-native-unix-common/4.1.97.Final/netty-transport-native-unix-common-4.1.97.Final.jar",
                    "sha1": "d469d84265ab70095b01b40886cabdd433b6e664",
                    "size": 43897,
                    "url": "https://libraries.minecraft.net/io/netty/netty-transport-native-unix-common/4.1.97.Final/netty-transport-native-unix-common-4.1.97.Final.jar"
                }
            },
            "name": "io.netty:netty-transport-native-unix-common:4.1.97.Final"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "io/netty/netty-transport/4.1.97.Final/netty-transport-4.1.97.Final.jar",
                    "sha1": "f37380d23c9bb079bc702910833b2fd532c9abd0",
                    "size": 489624,
                    "url": "https://libraries.minecraft.net/io/netty/netty-transport/4.1.97.Final/netty-transport-4.1.97.Final.jar"
                }
            },
            "name": "io.netty:netty-transport:4.1.97.Final"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-macos.jar",
                    "sha1": "33a6efa288390490ce6eb6c3df47ac21ecf648cf",
                    "size": 60543,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-macos.jar"
                }
            },
            "name": "org.lwjgl:lwjgl:3.3.3:natives-macos",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "osx"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-macos-arm64.jar",
                    "sha1": "226246e75f6bd8d4e1895bdce8638ef87808d114",
                    "size": 48620,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-macos-arm64.jar"
                }
            },
            "name": "org.lwjgl:lwjgl:3.3.3:natives-macos-arm64",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "osx"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows.jar",
                    "sha1": "a5ed18a2b82fc91b81f40d717cb1f64c9dcb0540",
                    "size": 165442,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows.jar"
                }
            },
            "name": "org.lwjgl:lwjgl:3.3.3:natives-windows",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "windows"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar",
                    "sha1": "e9aca8c5479b520a2a7f0d542a118140e812c5e8",
                    "size": 133378,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar"
                }
            },
            "name": "org.lwjgl:lwjgl:3.3.3:natives-windows-arm64",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "windows"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-x86.jar",
                    "sha1": "9e670718e050aeaeea0c2d5b907cffb142f2e58f",
                    "size": 139653,
                    "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-x86.jar"
                }
            },
            "name": "org.lwjgl:lwjgl:3.3.3:natives-windows-x86",
            "rules": [
                {
                    "action": "allow",
                    "os": {
                        "name": "windows"
                    }
                }
            ]
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/lz4/lz4-java/1.8.0/lz4-java-1.8.0.jar",
                    "sha1": "4b986a99445e49ea5fbf5d149c4b63f6ed6c6780",
                    "size": 682804,
                    "url": "https://libraries.minecraft.net/org/lz4/lz4-java/1.8.0/lz4-java-1.8.0.jar"
                }
            },
            "name": "org.lz4:lz4-java:1.8.0"
        },
        {
            "downloads": {
                "artifact": {
                    "path": "org/slf4j/slf4j-api/2.0.9/slf4j-api-2.0.9.jar",
                    "sha1": "7cf2726fdcfbc8610f9a71fb3ed639871f315340",
                    "size": 64579,
                    "url": "https://libraries.minecraft.net/org/slf4j/slf4j-api/2.0.9/slf4j-api-2.0.9.jar"
                }
            },
            "name": "org.slf4j:slf4j-api:2.0.9"
        }
    ],
    "logging": {
        "client": {
            "argument": "-Dlog4j.configurationFile=${path}",
            "file": {
                "id": "client-1.12.xml",
                "sha1": "bd65e7d2e3c237be76cfbef4c2405033d7f91521",
                "size": 888,
                "url": "https://piston-data.mojang.com/v1/objects/bd65e7d2e3c237be76cfbef4c2405033d7f91521/client-1.12.xml"
            },
            "type": "log4j2-xml"
        }
    },
    "mainClass": "net.minecraft.client.main.Main",
    "minimumLauncherVersion": 21,
    "releaseTime": "2024-08-08T12:24:45+00:00",
    "time": "2024-08-08T12:24:45+00:00",
    "type": "release"
}"#;

        // This should fail to parse as LegacyVanillaVersion
        let result = serde_json::from_str::<LegacyVanillaVersion>(modern_json);
        assert!(
            result.is_err(),
            "Modern version incorrectly parsed as legacy version"
        );
    }
    /// Tests that parsing fails gracefully with appropriate error message
    #[test]
    fn test_parsing_error_details() {
        // A simplified modern version JSON that should fail in a predictable way
        let simplified_modern_json = r#"{
                "id": "1.21.1",
                "type": "release",
                "arguments": {
                    "game": ["--username", "${auth_player_name}"],
                    "jvm": ["-Xmx2G", "-Xms512M"]
                },
                "assetIndex": {
                    "id": "3",
                    "sha1": "hash",
                    "size": 12345,
                    "totalSize": 67890,
                    "url": "https://example.com"
                },
                "mainClass": "net.minecraft.client.main.Main"
            }"#;

        // This should fail with a specific error about modern version format
        let result = serde_json::from_str::<LegacyVanillaVersion>(simplified_modern_json);
        assert!(
            result.is_err(),
            "Expected parsing to fail for modern format"
        );

        let error = result.err().unwrap().to_string();
        assert!(
            error.contains("Modern version format") || error.contains("arguments"),
            "Error should indicate modern version format incompatibility: {}",
            error
        );
    }

    /// Tests conversion between BaseVersion and LegacyVanillaVersion
    #[test]
    fn test_conversion_between_types() {
        // Create a minimal BaseVersion
        let base = BaseVersion {
            id: "test-1.7.10".to_string(),
            time: "2023-01-01T00:00:00Z".to_string(),
            release_time: "2023-01-01T00:00:00Z".to_string(),
            type_: "release".to_string(),
            main_class: "net.minecraft.client.main.Main".to_string(),
            libraries: Vec::new(),
            inherits_from: None,
            arguments: None,
            minecraft_arguments: Some("--username ${auth_player_name}".to_string()),
            logging: None,
        };

        // Create asset index
        let asset_index = AssetIndex {
            id: "1.7.10".to_string(),
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
            windows_server: None,
        };

        // Create java version
        let java_version = Some(JavaVersion {
            component: "jre-legacy".to_string(),
            major_version: 8,
        });

        // Create LegacyVanillaVersion
        let legacy = LegacyVanillaVersion::from_base_version(
            base.clone(),
            asset_index,
            "1.7.10".to_string(),
            13,
            downloads,
            java_version,
            Some(0),
        );

        // Convert back to BaseVersion
        let converted_base = legacy.to_base_version();

        // Verify conversions worked correctly
        assert_eq!(converted_base.id, base.id);
        assert_eq!(converted_base.main_class, base.main_class);
        assert_eq!(converted_base.minecraft_arguments, base.minecraft_arguments);
    }
}
