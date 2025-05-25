use std::path::PathBuf;

use craft_launcher_rust::version_parser::version_parser::{
    MinecraftVersion, parse_version_from_root_dir,
};

// Helper function to build path to test data directory
fn get_test_data_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data")
}

/**
 * Extract the assets index ID from a Minecraft version.
 * For vanilla versions, returns the ID directly.
 * For modded versions like Forge, Fabric, and NeoForge,
 * looks for the inheritsFrom field to get the asset info from the parent version.
 *
 * @param version The Minecraft version to extract assets index ID from
 * @return Option<String> containing the assets index ID if found
 */
fn extract_assets_index_id(version: &MinecraftVersion) -> Option<String> {
    match version {
        MinecraftVersion::ModernVanilla(v) => Some(v.asset_index.id.clone()),
        MinecraftVersion::LegacyVanilla(v) => v.assets.clone(),
        MinecraftVersion::ModernForge(v) => {
            // For Forge, we need to check the inheritsFrom field and parse that version
            let inherits_from = &v.inherits_from;
            let parent_version = inherits_from;
            let root_dir = get_test_data_path();
            if let Ok(inherited_version) = parse_version_from_root_dir(&root_dir, parent_version) {
                return extract_assets_index_id(&inherited_version);
            }
            None
        }

        MinecraftVersion::LegacyForge(v) => {
            // For Legacy Forge, we need to check the inheritsFrom field
            let inherits_from = &v.inherits_from;
            let parent_version = inherits_from;
            let root_dir = get_test_data_path();
            if let Ok(inherited_version) = parse_version_from_root_dir(&root_dir, parent_version) {
                return extract_assets_index_id(&inherited_version);
            }
            None
        }

        MinecraftVersion::ModernFabric(v) => {
            // For Fabric, we need to check the inheritsFrom field and parse that version
            let inherits_from = &v.inherits_from;
            let parent_version = inherits_from;
            let root_dir = get_test_data_path();
            if let Ok(inherited_version) = parse_version_from_root_dir(&root_dir, parent_version) {
                return extract_assets_index_id(&inherited_version);
            }
            None
        }

        MinecraftVersion::LegacyFabric(v) => {
            // Similar approach for Legacy Fabric
            let inherits_from = &v.inherits_from;
            let parent_version = inherits_from;
            let root_dir = get_test_data_path();
            if let Ok(inherited_version) = parse_version_from_root_dir(&root_dir, parent_version) {
                return extract_assets_index_id(&inherited_version);
            }
            None
        }

        MinecraftVersion::NeoForge(v) => {
            // For NeoForge, also check inheritsFrom
            let inherits_from = &v.inherits_from;
            let parent_version = inherits_from;
            let root_dir = get_test_data_path();
            if let Ok(inherited_version) = parse_version_from_root_dir(&root_dir, parent_version) {
                return extract_assets_index_id(&inherited_version);
            }
            None
        }
    }
}

// Test that we can get assets from inherited versions

#[cfg(test)]
mod tests {
    use std::error::Error;

    use craft_launcher_rust::{
        assets_parser::assets_parser::AssetsIndex,
        version_parser::version_parser::parse_version_from_root_dir,
    };

    use crate::{extract_assets_index_id, get_test_data_path};

    #[test]
    fn test_get_assets_from_inherited_version() -> Result<(), Box<dyn Error>> {
        // Get root directory
        let root_dir = get_test_data_path();

        // Parse a Fabric version that inherits from a vanilla version
        let fabric_version_id = "fabric-loader-0.14.24-1.20.2";
        let fabric_result = parse_version_from_root_dir(&root_dir, fabric_version_id)?;

        // Extract the assets index ID from the fabric version
        let assets_index_id = extract_assets_index_id(&fabric_result)
            .ok_or("Failed to extract assets index ID from Fabric version")?;

        println!("Assets index ID from Fabric version: {}", assets_index_id);

        // Load the assets index using the extracted ID
        let assets_result = AssetsIndex::get_json_from_root(&root_dir, &assets_index_id)?;
        let assets = AssetsIndex::from_json(&assets_result)?;

        // Verify that we loaded valid assets data
        assert!(
            assets.get_asset_count() > 0,
            "Assets index should contain objects"
        );
        println!(
            "Found {} assets with total size {}",
            assets.get_asset_count(),
            assets.get_total_size()
        );

        // Check if a common asset exists in the loaded assets
        let common_asset_path = "minecraft/sounds/random/bow.ogg";
        let asset_path = assets.get_asset_path(common_asset_path);
        assert!(
            asset_path.is_some(),
            "Common asset '{}' should exist in assets",
            common_asset_path
        );

        if let Some((path, obj)) = asset_path {
            println!(
                "Asset path: {}, Hash: {}, Size: {}",
                path, obj.hash, obj.size
            );
            assert!(!path.is_empty(), "Asset path should not be empty");
        }

        Ok(())
    }

    // Test getting assets from vanilla version directly
    #[test]
    fn test_get_assets_from_vanilla_version() -> Result<(), Box<dyn Error>> {
        // Get root directory
        let root_dir = get_test_data_path();

        // Parse a vanilla version
        let vanilla_version_id = "1.16.5";
        let vanilla_result = parse_version_from_root_dir(&root_dir, vanilla_version_id)?;

        // Extract the assets index ID from vanilla version
        let assets_index_id = extract_assets_index_id(&vanilla_result)
            .ok_or("Failed to extract assets index ID from vanilla version")?;

        println!("Assets index ID from Vanilla version: {}", assets_index_id);

        // Load the assets index using the extracted ID
        let assets_result = AssetsIndex::get_json_from_root(&root_dir, &assets_index_id)?;
        let assets = AssetsIndex::from_json(&assets_result)?;

        // Verify that we loaded valid assets data
        assert!(
            assets.get_asset_count() > 0,
            "Assets index should contain objects"
        );
        println!(
            "Found {} assets with total size {}",
            assets.get_asset_count(),
            assets.get_total_size()
        );

        // Check if we can get paths for specific assets
        let asset_names = vec![
            "minecraft/sounds/random/bow.ogg",
            "minecraft/sounds/random/click.ogg",
        ];

        for asset_name in asset_names {
            if let Some((path, obj)) = assets.get_asset_path(asset_name) {
                println!("Asset: {}, Path: {}, Size: {}", asset_name, path, obj.size);
                assert!(!path.is_empty(), "Asset path should not be empty");
            } else {
                panic!("Should find {} in assets", asset_name);
            }
        }

        Ok(())
    }

    // Test that assets can be retrieved from different loader versions
    #[test]
    fn test_get_assets_from_different_loaders() -> Result<(), Box<dyn Error>> {
        let root_dir = get_test_data_path();
        let loader_versions = vec![
            "1.21-forge-51.0.33",
            "fabric-loader-0.14.24-1.20.2",
            "neoforge-21.5.66-beta",
        ];

        for version_id in loader_versions {
            println!("Testing assets from loader: {}", version_id);
            let version_result = parse_version_from_root_dir(&root_dir, version_id)?;

            let assets_index_id = extract_assets_index_id(&version_result).ok_or(format!(
                "Failed to extract assets index ID from version: {}",
                version_id
            ))?;

            println!("Assets index ID: {}", assets_index_id);

            // Load the assets index
            let assets_result = AssetsIndex::get_json_from_root(&root_dir, &assets_index_id)?;
            let assets = AssetsIndex::from_json(&assets_result)?;

            // Verify that we loaded valid assets data
            assert!(
                assets.get_asset_count() > 0,
                "Assets index should contain objects"
            );
            println!(
                "Found {} assets with total size {}",
                assets.get_asset_count(),
                assets.get_total_size()
            );
        }

        Ok(())
    }

    // Test handling of versions that don't have assets
    #[test]
    fn test_handle_missing_assets() {
        let root_dir = get_test_data_path();

        // Create a non-existent version ID
        let non_existent_id = "non-existent-version";
        let result = parse_version_from_root_dir(&root_dir, non_existent_id);

        // This should error since the version doesn't exist
        assert!(result.is_err());

        // Now test with a valid version but invalid assets ID
        if let Ok(_version) = parse_version_from_root_dir(&root_dir, "1.16.5") {
            // Try to load assets with an invalid ID
            let invalid_assets_id = "invalid-assets-id";
            let assets_result = AssetsIndex::get_json_from_root(&root_dir, invalid_assets_id);

            // This should error since the assets index doesn't exist
            assert!(assets_result.is_err());
        }
    }
}
