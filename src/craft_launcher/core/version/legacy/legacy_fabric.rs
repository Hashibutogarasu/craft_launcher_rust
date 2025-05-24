pub mod legacy_fabric {
    use serde::{Deserialize, Serialize};

    // Represents a Legacy Fabric manifest file
    // This structure corresponds to the JSON manifest for Legacy Fabric loader
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LegacyFabricManifest {
        /// Minecraft version this Fabric loader inherits from
        #[serde(rename = "inheritsFrom")]
        pub inherits_from: String,

        /// Release timestamp
        #[serde(rename = "releaseTime")]
        pub release_time: String,

        /// Main class for the Fabric loader
        #[serde(rename = "mainClass")]
        pub main_class: String,

        /// Libraries required by the Fabric loader
        pub libraries: Vec<FabricLibrary>,

        /// Identifier of this Fabric loader
        pub id: String,

        /// Creation timestamp
        pub time: String,

        /// Type of release (e.g., "release", "snapshot")
        #[serde(rename = "type")]
        pub release_type: String,
    }

    // Represents a library dependency in the Fabric manifest
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FabricLibrary {
        /// SHA-1 hash of the library (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sha1: Option<String>,

        /// SHA-256 hash of the library (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sha256: Option<String>,

        /// Size of the library in bytes (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<u64>,

        /// Maven coordinate of the library
        pub name: String,

        /// SHA-512 hash of the library (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sha512: Option<String>,

        /// URL from which the library can be downloaded (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,

        /// MD5 hash of the library (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub md5: Option<String>,
    }

    // Functions for working with Legacy Fabric manifests
    impl LegacyFabricManifest {
        /// Parse a Legacy Fabric manifest from JSON string
        pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json)
        }

        /// Convert the manifest to a JSON string
        pub fn to_json(&self) -> Result<String, serde_json::Error> {
            serde_json::to_string(self)
        }

        /// Get the Minecraft version this Fabric loader is based on
        pub fn get_minecraft_version(&self) -> &str {
            &self.inherits_from
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::craft_launcher::core::version::legacy::legacy_fabric::legacy_fabric::LegacyFabricManifest;

    #[test]
    fn test_parse_legacy_fabric_manifest() {
        // Sample JSON based on the attached fabric-loader-0.16.14-1.12.2.json
        let sample_json = r#"{
                "inheritsFrom": "1.12.2",
                "releaseTime": "2025-05-20T11:28:11+0000",
                "mainClass": "net.fabricmc.loader.impl.launch.knot.KnotClient",
                "libraries": [
                    {
                        "sha1": "dc19ecb3f7889b7860697215cae99c0f9b6f6b4b",
                        "sha256": "876eab6a83daecad5ca67eb9fcabb063c97b5aeb8cf1fca7a989ecde17522051",
                        "size": 126113,
                        "name": "org.ow2.asm:asm:9.8",
                        "sha512": "cbd250b9c698a48a835e655f5f5262952cc6dd1a434ec0bc3429a9de41f2ce08fcd3c4f569daa7d50321ca6ad1d32e131e4199aa4fe54bce9e9691b37e45060e",
                        "url": "https://maven.fabricmc.net/",
                        "md5": "f5adf3bfc54fb3d2cd8e3a1f275084bc"
                    },
                    {
                        "name": "net.legacyfabric:intermediary:1.12.2",
                        "url": "https://maven.legacyfabric.net/"
                    }
                ],
                "id": "fabric-loader-0.16.14-1.12.2",
                "time": "2025-05-20T11:28:11+0000",
                "type": "release"
            }"#;

        // Parse the JSON
        let result = LegacyFabricManifest::from_json(sample_json);
        assert!(
            result.is_ok(),
            "Failed to parse Legacy Fabric manifest: {:?}",
            result.err()
        );

        let manifest = result.unwrap();

        // Test basic properties
        assert_eq!(manifest.inherits_from, "1.12.2");
        assert_eq!(
            manifest.main_class,
            "net.fabricmc.loader.impl.launch.knot.KnotClient"
        );
        assert_eq!(manifest.id, "fabric-loader-0.16.14-1.12.2");
        assert_eq!(manifest.release_type, "release");

        // Test libraries
        assert_eq!(manifest.libraries.len(), 2);

        // Test first library with all fields
        let first_lib = &manifest.libraries[0];
        assert_eq!(first_lib.name, "org.ow2.asm:asm:9.8");
        assert_eq!(
            first_lib.sha1,
            Some("dc19ecb3f7889b7860697215cae99c0f9b6f6b4b".to_string())
        );
        assert_eq!(first_lib.size, Some(126113));
        assert_eq!(
            first_lib.url,
            Some("https://maven.fabricmc.net/".to_string())
        );

        // Test second library with minimal fields
        let second_lib = &manifest.libraries[1];
        assert_eq!(second_lib.name, "net.legacyfabric:intermediary:1.12.2");
        assert_eq!(second_lib.sha1, None);
        assert_eq!(second_lib.size, None);

        // Test helper method
        assert_eq!(manifest.get_minecraft_version(), "1.12.2");

        // Test serialization roundtrip
        let serialized = manifest.to_json().expect("Failed to serialize manifest");
        let deserialized =
            LegacyFabricManifest::from_json(&serialized).expect("Failed to deserialize manifest");
        assert_eq!(deserialized.id, manifest.id);
        assert_eq!(deserialized.libraries.len(), manifest.libraries.len());
    }

    #[test]
    fn test_empty_or_invalid_json() {
        // Test with empty string
        let result = LegacyFabricManifest::from_json("");
        assert!(result.is_err());

        // Test with malformed JSON
        let result = LegacyFabricManifest::from_json("{\"inheritsFrom\": \"1.12.2\"");
        assert!(result.is_err());

        // Test with valid JSON missing required fields
        let result = LegacyFabricManifest::from_json("{\"inheritsFrom\": \"1.12.2\"}");
        assert!(result.is_err());
    }
}
