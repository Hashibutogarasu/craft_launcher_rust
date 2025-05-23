pub mod json_structs {
    pub mod launcher {
        use serde::{Deserialize, Serialize};

        pub struct VanillaVersion {}

        pub struct LegacyForgeVersion {}

        pub struct ForgeVersion {}

        pub struct LegacyNeoForgeVersion {}

        pub struct NeoForgeVersion {}

        pub struct LegacyFabricVersion {}

        pub struct FabricVersion {}

        /// Profile structure representing a Minecraft launcher profile
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Profile {
            /// Timestamp when the profile was created
            #[serde(skip_serializing_if = "Option::is_none")]
            pub created: Option<String>,

            /// Custom game directory path
            #[serde(skip_serializing_if = "Option::is_none", rename = "gameDir")]
            pub game_dir: Option<String>,

            /// Profile icon identifier or base64 encoded image
            #[serde(skip_serializing_if = "Option::is_none")]
            pub icon: Option<String>,

            /// Custom Java executable path
            #[serde(skip_serializing_if = "Option::is_none", rename = "javaDir")]
            pub java_dir: Option<String>,

            /// Java VM arguments
            #[serde(skip_serializing_if = "Option::is_none", rename = "javaArgs")]
            pub java_args: Option<String>,

            /// Timestamp when the profile was last used
            #[serde(skip_serializing_if = "Option::is_none", rename = "lastUsed")]
            pub last_used: Option<String>,

            /// Version ID of the Minecraft version to use
            #[serde(skip_serializing_if = "Option::is_none", rename = "lastVersionId")]
            pub last_version_id: Option<String>,

            /// Profile display name
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,

            /// Whether to skip Java Runtime Edition version check
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = "skipJreVersionCheck"
            )]
            pub skip_jre_version_check: Option<bool>,

            /// Profile type (e.g. "custom", "latest-release", "latest-snapshot")
            #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
            pub type_: Option<String>,

            /// Custom resolution width
            #[serde(skip_serializing_if = "Option::is_none", rename = "resolution.width")]
            pub resolution_width: Option<u32>,

            /// Custom resolution height
            #[serde(skip_serializing_if = "Option::is_none", rename = "resolution.height")]
            pub resolution_height: Option<u32>,

            /// Whether to launch in fullscreen mode
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fullscreen: Option<bool>,
        }

        /// Root structure for the launcher_profiles.json file
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LauncherProfiles {
            /// Map of profile IDs to profile objects
            pub profiles: std::collections::HashMap<String, Profile>,

            /// Selected profile ID
            #[serde(skip_serializing_if = "Option::is_none", rename = "selectedProfileId")]
            pub selected_profile_id: Option<String>,

            /// Launcher version
            #[serde(skip_serializing_if = "Option::is_none", rename = "launcherVersion")]
            pub launcher_version: Option<serde_json::Value>,

            /// Launcher settings
            #[serde(skip_serializing_if = "Option::is_none")]
            pub settings: Option<serde_json::Value>,

            /// Authentication database
            #[serde(
                skip_serializing_if = "Option::is_none",
                rename = "authenticationDatabase"
            )]
            pub authentication_database: Option<serde_json::Value>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::json_structs::launcher::{LauncherProfiles, Profile};
    use crate::craft_launcher::utils::file_operations::file_utils;
    use crate::craft_launcher::utils::path_operations::path_operations;
    use std::collections::HashMap;

    /**
     * Test serializing and deserializing a LauncherProfiles structure.
     * This test creates a sample launcher_profiles.json file in a temporary directory,
     * reads it back into a LauncherProfiles structure, and verifies that the data matches.
     */
    #[test]
    fn test_launcher_profiles_serialization() {
        // Create a temporary directory for testing
        path_operations::create_temporary_dir();
        let temp_dir = path_operations::get_temporary_dir();

        // Create sample Profile objects
        let mut profiles = HashMap::new();

        // Add a release profile
        let release_profile = Profile {
            created: Some("2023-01-15T08:30:00.000Z".to_string()),
            game_dir: None,
            icon: Some("Grass".to_string()),
            java_dir: Some("C:\\Program Files\\Java\\jdk-17\\bin\\javaw.exe".to_string()),
            java_args: Some("-Xmx2G -XX:+UnlockExperimentalVMOptions".to_string()),
            last_used: Some("2023-05-20T14:45:30.123Z".to_string()),
            last_version_id: Some("1.19.3".to_string()),
            name: Some("Release".to_string()),
            skip_jre_version_check: Some(false),
            type_: Some("latest-release".to_string()),
            resolution_width: Some(1280),
            resolution_height: Some(720),
            fullscreen: Some(false),
        };
        profiles.insert("release_profile_id".to_string(), release_profile);

        // Add a custom profile
        let custom_profile = Profile {
            created: Some("2023-02-10T10:15:00.000Z".to_string()),
            game_dir: Some("C:\\Minecraft\\ModPacks\\MyModPack".to_string()),
            icon: Some("Furnace".to_string()),
            java_dir: None,
            java_args: Some("-Xmx4G".to_string()),
            last_used: Some("2023-05-21T18:30:12.456Z".to_string()),
            last_version_id: Some("fabric-loader-0.14.24-1.19.3".to_string()),
            name: Some("Fabric Mods".to_string()),
            skip_jre_version_check: Some(true),
            type_: Some("custom".to_string()),
            resolution_width: Some(1920),
            resolution_height: Some(1080),
            fullscreen: Some(true),
        };
        profiles.insert("custom_profile_id".to_string(), custom_profile);

        // Create sample LauncherProfiles structure
        let launcher_profiles = LauncherProfiles {
            profiles,
            selected_profile_id: Some("custom_profile_id".to_string()),
            launcher_version: Some(serde_json::json!({
                "name": "2.3.4",
                "format": 21
            })),
            settings: Some(serde_json::json!({
                "enableSnapshots": true,
                "enableReleases": true,
                "keepLauncherOpen": true,
                "showGameLog": false
            })),
            authentication_database: None,
        };

        // Serialize the structure to JSON and write to a file in the temporary directory
        let profiles_path = temp_dir.join("launcher_profiles.json");
        file_utils::write_struct_to_file_as_json(&profiles_path, &launcher_profiles)
            .expect("Failed to write launcher_profiles.json");

        // Read the JSON file back into a structure
        let read_profiles: LauncherProfiles =
            file_utils::read_struct_from_file_as_json(&profiles_path)
                .expect("Failed to read launcher_profiles.json");

        // Verify that the structures match
        assert_eq!(
            read_profiles.profiles.len(),
            launcher_profiles.profiles.len()
        );
        assert_eq!(
            read_profiles.selected_profile_id,
            launcher_profiles.selected_profile_id
        );

        // Verify specific profile data
        let read_release_profile = read_profiles
            .profiles
            .get("release_profile_id")
            .expect("Release profile not found");
        assert_eq!(read_release_profile.name, Some("Release".to_string()));
        assert_eq!(
            read_release_profile.last_version_id,
            Some("1.19.3".to_string())
        );
        assert_eq!(read_release_profile.resolution_width, Some(1280));
        assert_eq!(read_release_profile.resolution_height, Some(720));

        let read_custom_profile = read_profiles
            .profiles
            .get("custom_profile_id")
            .expect("Custom profile not found");
        assert_eq!(read_custom_profile.name, Some("Fabric Mods".to_string()));
        assert_eq!(
            read_custom_profile.game_dir,
            Some("C:\\Minecraft\\ModPacks\\MyModPack".to_string())
        );
        assert_eq!(read_custom_profile.fullscreen, Some(true));

        // Clean up the temporary directory
        path_operations::cleanup_temporary_dir();
    }

    /**
     * Test parsing a realistic Minecraft launcher profile format.
     * This test creates a JSON file that mimics the actual format used by the Minecraft launcher
     * and verifies that it can be properly parsed into our structures.
     */
    #[test]
    fn test_realistic_launcher_profiles_format() {
        // Create a temporary directory for testing
        path_operations::create_temporary_dir();
        let temp_dir = path_operations::get_temporary_dir();

        // Create a realistic launcher_profiles.json content
        let json_content = r#"{
            "profiles": {
                "44bbfaf23fa3a58ba7b9856e7b630b47": {
                    "created": "1970-01-02T00:00:00.000Z",
                    "icon": "Grass",
                    "javaDir": "C:\\Program Files\\Java\\jdk-21\\bin\\javaw.exe",
                    "lastUsed": "2025-03-27T16:23:24.011Z",
                    "lastVersionId": "latest-release",
                    "name": "Latest Release",
                    "skipJreVersionCheck": false,
                    "type": "latest-release"
                },
                "589215635c462ae02e42ec3eb049d726": {
                    "created": "2023-11-03T17:19:05.772Z",
                    "gameDir": "C:\\Users\\Player\\AppData\\Roaming\\.minecraft\\mod_versions\\fabric_23w44a",
                    "icon": "TNT",
                    "lastUsed": "2023-11-03T17:35:18.641Z",
                    "lastVersionId": "fabric-loader-0.14.24-23w44a",
                    "name": "Fabric 23w44a",
                    "type": "custom"
                },
                "64159ea5ce8cb0ae6710ae7ee7cf67da": {
                    "created": "2023-01-13T13:09:57.132Z",
                    "icon": "Furnace",
                    "javaArgs": "-Xmx3G -XX:+UnlockExperimentalVMOptions",
                    "lastUsed": "2023-04-23T04:49:54.864Z",
                    "lastVersionId": "1.12.2",
                    "name": "1.12.2",
                    "type": "custom",
                    "resolution.width": 1280,
                    "resolution.height": 720,
                    "fullscreen": false
                }
            },
            "selectedProfileId": "44bbfaf23fa3a58ba7b9856e7b630b47",
            "settings": {
                "crashAssistance": false,
                "enableAdvanced": false,
                "enableAnalytics": true,
                "enableHistorical": false,
                "enableReleases": true,
                "enableSnapshots": false,
                "keepLauncherOpen": true,
                "profileSorting": "ByLastPlayed",
                "showGameLog": false,
                "showMenu": false,
                "soundOn": false
            },
            "version": 4
        }"#;

        // Write the JSON to a file in the temporary directory
        let profiles_path = temp_dir.join("realistic_launcher_profiles.json");
        file_utils::write_text(&profiles_path, json_content)
            .expect("Failed to write realistic launcher_profiles.json");

        // Read the JSON file into our structure
        let read_profiles: LauncherProfiles =
            file_utils::read_struct_from_file_as_json(&profiles_path)
                .expect("Failed to parse realistic launcher_profiles.json");

        // Verify the structure
        assert_eq!(read_profiles.profiles.len(), 3);
        assert_eq!(
            read_profiles.selected_profile_id,
            Some("44bbfaf23fa3a58ba7b9856e7b630b47".to_string())
        );

        // Verify specific profile data
        let latest_release_profile = read_profiles
            .profiles
            .get("44bbfaf23fa3a58ba7b9856e7b630b47")
            .expect("Latest release profile not found");
        assert_eq!(
            latest_release_profile.name,
            Some("Latest Release".to_string())
        );
        assert_eq!(
            latest_release_profile.type_,
            Some("latest-release".to_string())
        );
        assert_eq!(latest_release_profile.skip_jre_version_check, Some(false));

        let fabric_profile = read_profiles
            .profiles
            .get("589215635c462ae02e42ec3eb049d726")
            .expect("Fabric profile not found");
        assert_eq!(fabric_profile.name, Some("Fabric 23w44a".to_string()));
        assert_eq!(
            fabric_profile.game_dir,
            Some(
                "C:\\Users\\Player\\AppData\\Roaming\\.minecraft\\mod_versions\\fabric_23w44a"
                    .to_string()
            )
        );
        assert_eq!(fabric_profile.type_, Some("custom".to_string()));

        let older_profile = read_profiles
            .profiles
            .get("64159ea5ce8cb0ae6710ae7ee7cf67da")
            .expect("Older profile not found");
        assert_eq!(older_profile.name, Some("1.12.2".to_string()));
        assert_eq!(older_profile.last_version_id, Some("1.12.2".to_string()));
        assert_eq!(older_profile.resolution_width, Some(1280));
        assert_eq!(older_profile.resolution_height, Some(720));
        assert_eq!(older_profile.fullscreen, Some(false));
        assert_eq!(
            older_profile.java_args,
            Some("-Xmx3G -XX:+UnlockExperimentalVMOptions".to_string())
        );

        // Verify settings
        if let Some(settings) = &read_profiles.settings {
            if let Some(keep_launcher_open) = settings.get("keepLauncherOpen") {
                assert!(keep_launcher_open.as_bool().unwrap());
            } else {
                panic!("keepLauncherOpen setting not found");
            }

            if let Some(enable_snapshots) = settings.get("enableSnapshots") {
                assert!(!enable_snapshots.as_bool().unwrap());
            } else {
                panic!("enableSnapshots setting not found");
            }
        } else {
            panic!("Settings not found");
        }

        // Clean up the temporary directory
        path_operations::cleanup_temporary_dir();
    }
}
