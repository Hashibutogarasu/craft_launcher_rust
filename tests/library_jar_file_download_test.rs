#[cfg(test)]
mod tests {
    use craft_launcher_rust::craft_launcher::core::version::library_parser::library_parser::{
        LibraryInfo, convert_version_to_libraries, extract_base_library_path, maven_name_to_path,
    };
    use craft_launcher_rust::craft_launcher::core::version::version_parser::version_parser::parse_version_from_file;
    use craft_launcher_rust::craft_launcher::utils::file_operations::file_utils;
    use craft_launcher_rust::craft_launcher::utils::networking::networking;
    use std::path::PathBuf;

    /// Tests downloading library JAR files from a Minecraft version JSON
    #[test]
    fn test_library_jar_download() {
        // Define paths and constants
        let test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
        let libraries_dir = test_data_dir.join("libraries");
        let versions_dir = test_data_dir.join("versions");

        // Choose a version to test with - using 1.16.5 as it's stable and has a good set of libraries
        let version_id = "1.16.5";
        let version_dir = versions_dir.join(version_id);
        let version_json_path = version_dir.join(format!("{}.json", version_id));

        // Check if the version JSON exists
        assert!(
            file_utils::exists(&version_json_path),
            "Version JSON file does not exist at: {}",
            version_json_path.display()
        );

        // Parse the version JSON
        let version_result = parse_version_from_file(&version_json_path);
        assert!(
            version_result.is_ok(),
            "Failed to parse version JSON: {:?}",
            version_result.err()
        );

        let version = version_result.unwrap();

        // Convert version to libraries using the library_parser
        let libraries = convert_version_to_libraries(version);

        println!(
            "Found {} libraries in version {}",
            libraries.len(),
            version_id
        );

        // Limit the number of libraries to download for testing purposes
        let max_libraries_to_download = 3;
        let mut downloaded_count = 0;

        // Process each library
        for library in libraries {
            // Skip if we've already downloaded enough libraries
            if downloaded_count >= max_libraries_to_download {
                break;
            }

            // Get the library name and path information
            let (library_name, artifact_path_result) = match &library {
                LibraryInfo::Base(base_lib) => {
                    (base_lib.name.clone(), extract_base_library_path(base_lib))
                }
                LibraryInfo::Generic {
                    name,
                    path,
                    url,
                    sha1,
                    size,
                } => {
                    let name_clone = name.clone();
                    if let (Some(path), Some(url), Some(sha1)) = (path, url, sha1) {
                        (
                            name_clone,
                            Some((path.clone(), url.clone(), sha1.clone(), *size as i64)),
                        )
                    } else if url.is_some() {
                        // Try to derive path from Maven name
                        let derived_path = maven_name_to_path(&name);
                        let url_str = url.as_ref().unwrap();
                        (
                            name_clone,
                            Some((
                                derived_path.clone(),
                                format!("{}{}", url_str, derived_path),
                                String::from("unknown"),
                                0,
                            )),
                        )
                    } else {
                        // Try to derive path from Maven name with default URL
                        let derived_path = maven_name_to_path(&name);
                        (
                            name_clone,
                            Some((
                                derived_path.clone(),
                                format!("https://maven.minecraftforge.net/{}", derived_path),
                                String::from("unknown"),
                                0,
                            )),
                        )
                    }
                }
            };

            if artifact_path_result.is_none() {
                println!("Skipping library with no artifact path: {}", library_name);
                continue;
            }

            let (artifact_path, download_url, _expected_sha1, expected_size) =
                artifact_path_result.unwrap();

            // Create full local path
            let local_path = libraries_dir.join(&artifact_path);

            println!("Processing library: {}", library_name);
            println!("  -> Local path: {}", local_path.display());
            println!("  -> URL: {}", download_url);

            // Create directory structure if it doesn't exist
            if let Some(parent) = local_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .expect("Failed to create library directory structure");
                }
            }

            // Download if needed
            if !local_path.exists() {
                println!("  -> Downloading...");
                let download_result = networking::download_file(&download_url, &local_path);
                assert!(
                    download_result.is_ok(),
                    "Failed to download library '{}' from URL '{}': {:?}",
                    library.name(),
                    download_url,
                    download_result.err()
                );
                downloaded_count += 1;
            }

            // Verify the downloaded file
            assert!(
                file_utils::exists(&local_path),
                "Downloaded library file does not exist at '{}'",
                local_path.display()
            ); // Check file size
            let file_size = file_utils::get_file_size(&local_path).unwrap();

            // Only check file size if we have a valid expected size (greater than 0)
            if expected_size > 0 {
                assert_eq!(
                    file_size as i64, expected_size,
                    "Downloaded library file size does not match expected size"
                );
            } // In a real application, you would also verify the SHA1 hash here
            println!("  -> Successfully verified library: {}", library.name());
        }

        println!(
            "Successfully downloaded and verified {} libraries from version {}",
            downloaded_count, version_id
        );
    }
}
