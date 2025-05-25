pub mod library_extractor {
    use crate::craft_launcher::core::version::library_parser::library_parser::{
        LibraryInfo, convert_version_to_libraries,
    };
    use crate::craft_launcher::core::version::version_parser::version_parser::parse_version_from_file;
    use crate::craft_launcher::utils::file_operations::file_utils;
    use crate::craft_launcher::utils::networking::networking;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{self, Cursor, Read, Write};
    use std::path::{Path, PathBuf};

    /// Error type for library extraction operations
    #[derive(Debug)]
    pub enum LibraryExtractionError {
        IoError(io::Error),
        NetworkingError(String),
        VersionParsingError(String),
        NoNativeLibrariesFound,
        ZipError(zip::result::ZipError),
    }

    impl From<io::Error> for LibraryExtractionError {
        fn from(error: io::Error) -> Self {
            LibraryExtractionError::IoError(error)
        }
    }

    impl From<zip::result::ZipError> for LibraryExtractionError {
        fn from(error: zip::result::ZipError) -> Self {
            LibraryExtractionError::ZipError(error)
        }
    }

    /// Extracts native library files for a specific Minecraft version
    ///
    /// This function downloads and extracts native library files for the specified Minecraft version
    /// and the current operating system. It creates a zip archive containing all native libraries
    /// and then extracts them to a directory named after the SHA-256 hash of the zip file.
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Path to the root directory where game data is stored
    /// * `version_id` - The Minecraft version ID (e.g., "1.16.5")
    ///
    /// # Returns
    ///
    /// * `Result<PathBuf, LibraryExtractionError>` - The path to the directory containing extracted native libraries
    pub fn extract_native_libraries(
        root_dir: &Path,
        version_id: &str,
    ) -> Result<PathBuf, LibraryExtractionError> {
        // Define paths
        let libraries_dir = root_dir.join("libraries");
        let versions_dir = root_dir.join("versions");
        let bin_dir = root_dir.join("bin");

        // Create bin directory if it doesn't exist
        if !bin_dir.exists() {
            fs::create_dir_all(&bin_dir)?;
        }

        // Construct version JSON path
        let version_dir = versions_dir.join(version_id);
        let version_json_path = version_dir.join(format!("{}.json", version_id));

        // Check if the version JSON exists
        if !file_utils::exists(&version_json_path) {
            return Err(LibraryExtractionError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Version JSON file not found at: {}",
                    version_json_path.display()
                ),
            )));
        }

        // Parse the version JSON
        let version = parse_version_from_file(&version_json_path).map_err(|e| {
            LibraryExtractionError::VersionParsingError(format!(
                "Failed to parse version JSON: {:?}",
                e
            ))
        })?;

        // Convert version to libraries
        let libraries = convert_version_to_libraries(version);

        // Get the current OS name
        let os_name = std::env::consts::OS;

        // Map OS name to classifier prefix
        let classifier_prefix = match os_name {
            "windows" => "natives-windows",
            "macos" => "natives-macos",
            "linux" => "natives-linux",
            _ => {
                return Err(LibraryExtractionError::NoNativeLibrariesFound);
            }
        };

        // Collect native libraries for current OS
        let mut native_libraries = Vec::new();
        for library in &libraries {
            match library {
                LibraryInfo::Base(base_lib) => {
                    // Process native libraries
                    if let Some(natives) = &base_lib.natives {
                        if let Some(classifier) = natives.get(os_name) {
                            if let Some(downloads) = &base_lib.downloads {
                                if let Some(classifiers) = &downloads.classifiers {
                                    // Check if classifier equals the classifier_prefix or contains it
                                    if classifier == classifier_prefix
                                        || classifiers.contains_key(classifier_prefix)
                                    {
                                        // Try to get the classifier directly first
                                        let artifact =
                                            if classifiers.contains_key(classifier_prefix) {
                                                classifiers.get(classifier_prefix)
                                            } else {
                                                classifiers.get(classifier)
                                            };

                                        if let Some(artifact) = artifact {
                                            let native_lib_path =
                                                libraries_dir.join(&artifact.path);
                                            let download_url = &artifact.url;

                                            // Download native library if needed
                                            if !native_lib_path.exists() {
                                                if let Some(parent) = native_lib_path.parent() {
                                                    if !parent.exists() {
                                                        fs::create_dir_all(parent)?;
                                                    }
                                                }
                                                let result = networking::download_file(
                                                    download_url,
                                                    &native_lib_path,
                                                );
                                                if let Err(e) = result {
                                                    return Err(
                                                        LibraryExtractionError::NetworkingError(
                                                            format!(
                                                                "Failed to download native library: {:?}",
                                                                e
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }

                                            // Add to our list of native libraries
                                            native_libraries
                                                .push((native_lib_path, base_lib.extract.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                LibraryInfo::Generic { .. } => {
                    // Most generic libraries don't have native components
                }
            }
        }

        // Some Minecraft versions may not require native libraries
        if native_libraries.is_empty() {
            return Err(LibraryExtractionError::NoNativeLibrariesFound);
        }

        // Create a temporary buffer for our zip file
        let mut zip_buffer = Vec::new();
        {
            // Create a new zip file
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_buffer));
            let options = zip::write::FileOptions::<()>::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);

            let mut extracted_files = HashMap::new();

            // Extract and add each native library to the zip
            for (lib_path, extract_info) in native_libraries {
                // Open the JAR file (which is a zip file)
                let jar_file = File::open(&lib_path)?;
                let mut jar_archive = zip::ZipArchive::new(jar_file)?;

                // Get list of files to exclude from extraction
                let excluded_files = if let Some(extract) = extract_info {
                    extract.exclude
                } else {
                    vec![]
                };

                // Process each file in the JAR
                for i in 0..jar_archive.len() {
                    let mut file = jar_archive.by_index(i)?;
                    let outpath = file.name().to_string();

                    // Skip directories and excluded files
                    if file.is_dir()
                        || outpath.contains("META-INF")
                        || excluded_files.iter().any(|ex| outpath.contains(ex))
                    {
                        continue;
                    }

                    // Only include DLL, SO, and DYLIB files
                    let is_native_lib = outpath.ends_with(".dll")
                        || outpath.ends_with(".so")
                        || outpath.ends_with(".dylib");

                    if is_native_lib {
                        // Extract just the filename
                        let filename = Path::new(&outpath)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or_else(|| &outpath);

                        // Skip if we already have this file (prevent duplicates)
                        if extracted_files.contains_key(filename) {
                            continue;
                        }

                        // Read the file data
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)?;

                        // Add to our zip
                        zip.start_file(filename, options)?;
                        zip.write_all(&buffer)?;

                        // Mark as extracted
                        extracted_files.insert(filename.to_string(), true);
                    }
                }
            }

            // Finalize the zip
            zip.finish()?;
        }

        // Hash the zip file
        let mut hasher = Sha256::new();
        hasher.update(&zip_buffer);
        let hash = format!("{:x}", hasher.finalize());

        // Save the zip file (optional, but useful for debugging)
        let zip_path = bin_dir.join(format!("native-libs-{}.zip", version_id));
        fs::write(&zip_path, &zip_buffer)?;

        // Create hash directory
        let hash_dir = bin_dir.join(&hash);
        if hash_dir.exists() {
            fs::remove_dir_all(&hash_dir)?;
        }
        fs::create_dir_all(&hash_dir)?;

        // Extract ZIP to hash directory
        let mut zip_archive = zip::ZipArchive::new(Cursor::new(zip_buffer))?;
        for i in 0..zip_archive.len() {
            let mut file = zip_archive.by_index(i)?;
            let outpath = hash_dir.join(file.name());

            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;

            // Set file permissions (Unix only)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }

        Ok(hash_dir)
    }
}
