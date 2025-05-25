#[cfg(test)]
mod tests {
    use craft_launcher_rust::craft_launcher::core::assets::assets_parser::assets_parser::AssetsIndex;
    use craft_launcher_rust::craft_launcher::core::manifest::version_manifest_parser::version_manifest_parser::parse_version_manifest_from_file;
    use craft_launcher_rust::craft_launcher::utils::file_operations::file_utils;
    use craft_launcher_rust::craft_launcher::utils::networking::networking;
    use std::path::PathBuf;

    /// Tests downloading a version JSON from the manifest and then downloading assets
    #[test]
    fn test_asset_download_from_version_manifest() {
        // Define paths and constants
        let test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
        let assets_dir = test_data_dir.join("assets");
        let objects_dir = assets_dir.join("objects");
        let versions_dir = test_data_dir.join("versions");

        // Load the version manifest
        let manifest_path = versions_dir.join("version_manifest_v2.json");
        let manifest_result = parse_version_manifest_from_file(manifest_path.to_str().unwrap());
        assert!(
            manifest_result.is_ok(),
            "Failed to parse version manifest: {:?}",
            manifest_result.err()
        );

        let manifest = manifest_result.unwrap();

        // Get a recent version from the manifest (using snapshot as they have more frequent updates)
        let version = manifest
            .get_latest_snapshot()
            .expect("Failed to get latest snapshot version");
        println!("Using version: {} ({})", version.id, version.version_type); // Download the version JSON file
        let version_dir = versions_dir.join(&version.id);
        let version_json_filename = format!("{}.json", version.id);
        let version_json_path = version_dir.join(&version_json_filename);

        // Create version directory if it doesn't exist
        if !version_dir.exists() {
            std::fs::create_dir_all(&version_dir).expect("Failed to create version directory");
        }

        // Only download if it doesn't exist
        if !version_json_path.exists() {
            println!("Downloading version JSON from URL: {}", version.url);
            let download_result = networking::download_file(&version.url, &version_json_path);
            assert!(
                download_result.is_ok(),
                "Failed to download version JSON: {:?}",
                download_result.err()
            );
        }

        // Verify the version JSON was downloaded
        assert!(
            file_utils::exists(&version_json_path),
            "Version JSON file was not downloaded or doesn't exist"
        );

        // Read the version JSON to locate the assets index
        let version_json_content =
            std::fs::read_to_string(&version_json_path).expect("Failed to read version JSON file");

        let version_data: serde_json::Value =
            serde_json::from_str(&version_json_content).expect("Failed to parse version JSON");

        // Extract the assets index information
        let assets_index = version_data
            .get("assetIndex")
            .expect("No assetIndex field in version JSON");

        let assets_id = assets_index
            .get("id")
            .expect("No id field in assetIndex")
            .as_str()
            .expect("assetIndex id is not a string");

        let assets_url = assets_index
            .get("url")
            .expect("No url field in assetIndex")
            .as_str()
            .expect("assetIndex url is not a string");

        println!("Assets index ID: {}", assets_id);
        println!("Assets index URL: {}", assets_url);

        // Download the assets index if needed
        let assets_index_path = assets_dir
            .join("indexes")
            .join(format!("{}.json", assets_id));

        // Create the directory if it doesn't exist
        if !assets_index_path.parent().unwrap().exists() {
            std::fs::create_dir_all(assets_index_path.parent().unwrap())
                .expect("Failed to create assets indexes directory");
        }

        // Download the assets index if it doesn't exist
        if !assets_index_path.exists() {
            println!("Downloading assets index from URL: {}", assets_url);
            let download_result = networking::download_file(assets_url, &assets_index_path);
            assert!(
                download_result.is_ok(),
                "Failed to download assets index: {:?}",
                download_result.err()
            );
        }

        // Verify the assets index was downloaded
        assert!(
            file_utils::exists(&assets_index_path),
            "Assets index file was not downloaded or doesn't exist"
        );

        // Now load the assets index and download a few assets
        let result = AssetsIndex::from_root_dir(&test_data_dir, assets_id);
        assert!(
            result.is_ok(),
            "Failed to parse assets index: {:?}",
            result.err()
        );

        let assets_index = result.unwrap();

        // Select a few assets to test with (limiting to 3 for test speed)
        // These are common assets that should be present in most versions
        let test_assets = vec![
            "minecraft/sounds/random/click.ogg",
            "minecraft/sounds/damage/hit1.ogg",
            "minecraft/lang/en_us.json",
        ];

        // Process each test asset
        for asset_name in test_assets {
            // Get the asset path and object from the index
            let asset_info = assets_index.get_asset_path(asset_name);

            // Skip if this asset doesn't exist in this version
            if asset_info.is_none() {
                println!(
                    "Asset '{}' not found in index {}, skipping",
                    asset_name, assets_id
                );
                continue;
            }

            let (_relative_path, asset_object) = asset_info.unwrap();

            // Construct the destination path
            let hash = &asset_object.hash;
            let first_two_chars = &hash[0..2];
            let dest_dir = objects_dir.join(first_two_chars);
            let dest_path = dest_dir.join(hash);

            // Create the destination directory if it doesn't exist
            if !dest_dir.exists() {
                std::fs::create_dir_all(&dest_dir).expect("Failed to create asset directory");
            }

            // Construct the URL
            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                first_two_chars, hash
            );

            println!("Downloading asset: {} from URL: {}", asset_name, url);
            println!("  -> to path: {}", dest_path.display());

            // Only download if the file doesn't already exist
            if !dest_path.exists() {
                let download_result = networking::download_file(&url, &dest_path);
                assert!(
                    download_result.is_ok(),
                    "Failed to download asset '{}' from URL '{}': {:?}",
                    asset_name,
                    url,
                    download_result.err()
                );
            }

            // Verify the downloaded file
            assert!(
                file_utils::exists(&dest_path),
                "Downloaded asset file does not exist at '{}'",
                dest_path.display()
            );

            let file_size = file_utils::get_file_size(&dest_path).unwrap();
            assert_eq!(
                file_size, asset_object.size,
                "Downloaded asset file size does not match expected size"
            );
        }

        println!(
            "Successfully downloaded and verified assets from version {}",
            version.id
        );
    }
}
