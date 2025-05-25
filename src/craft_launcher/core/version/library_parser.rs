pub mod library_parser {
    use crate::craft_launcher::core::version::base_version::Library as BaseLibrary;
    use crate::craft_launcher::core::version::version_parser::version_parser::MinecraftVersion;

    /// Library information with unified interface for different library types
    ///
    /// Supports two variants:
    /// - Base: Wraps a BaseLibrary object
    /// - Generic: Contains library metadata directly
    #[derive(Debug, Clone)]
    pub enum LibraryInfo {
        /// Base library variant that wraps a BaseLibrary object
        Base(BaseLibrary),

        /// Generic library variant with direct metadata
        Generic {
            /// Maven artifact name
            name: String,

            /// Path for the library artifact
            path: Option<String>,

            /// URL for downloading the library
            url: Option<String>,

            /// SHA1 hash of the library
            sha1: Option<String>,

            /// Size of the library in bytes
            size: i64,
        },
    }

    impl LibraryInfo {
        /// Get the name of the library
        pub fn name(&self) -> &str {
            match self {
                LibraryInfo::Base(lib) => &lib.name,
                LibraryInfo::Generic { name, .. } => name,
            }
        }
    }

    /// Extracts library artifact path, download URL, SHA1 hash, and size from a BaseLibrary object
    ///
    /// Returns None if the library has no artifact path or URL
    /// Returns (path, url, sha1, size) if successful
    pub fn extract_base_library_path(
        library: &BaseLibrary,
    ) -> Option<(String, String, String, i64)> {
        // First try to use the downloads.artifact path if available
        if let Some(downloads) = &library.downloads {
            if let Some(artifact) = &downloads.artifact {
                return Some((
                    artifact.path.clone(),
                    artifact.url.clone(),
                    artifact.sha1.clone(),
                    artifact.size,
                ));
            }
        }

        // If no downloads section, try to construct the path from the library name and URL
        // This is common for Forge and Fabric libraries
        if let Some(url) = &library.url {
            let path = maven_name_to_path(&library.name);

            // We don't have SHA1 and size in this case, using placeholders
            return Some((
                path.clone(),
                format!("{}{}", url, path),
                String::from("unknown"),
                0,
            ));
        }

        None
    }

    /// Converts a Maven artifact name to a path
    ///
    /// Example: "com.example:foo:1.0" becomes "com/example/foo/1.0/foo-1.0.jar"
    pub fn maven_name_to_path(name: &str) -> String {
        let parts: Vec<&str> = name.split(':').collect();
        let (group_id, artifact_id, version) = match parts.len() {
            3 => (parts[0], parts[1], parts[2]),
            4 => (parts[0], parts[1], parts[2]), // Ignore classifier in this simple implementation
            _ => return String::new(),           // Invalid format
        };

        let group_path = group_id.replace('.', "/");
        format!(
            "{}/{}/{}/{}-{}.jar",
            group_path, artifact_id, version, artifact_id, version
        )
    }    /// Convert MinecraftVersion to a vector of LibraryInfo objects
    ///
    /// This function handles the conversion of libraries from different Minecraft version types
    /// to a unified LibraryInfo structure that can be uniformly processed.
    pub fn convert_version_to_libraries(version: MinecraftVersion) -> Vec<LibraryInfo> {
        match version {
            MinecraftVersion::ModernVanilla(v) => v
                .base
                .libraries
                .into_iter()
                .map(|lib| base_library_to_library_info(lib))
                .collect(),
            MinecraftVersion::LegacyVanilla(v) => v
                .base
                .libraries
                .into_iter()
                .map(|lib| base_library_to_library_info(lib))
                .collect(),
            MinecraftVersion::ModernForge(v) => v
                .libraries
                .into_iter()
                .map(|lib| {
                    if let Some(downloads) = &lib.downloads {
                        LibraryInfo::Generic {
                            name: lib.name,
                            path: Some(downloads.artifact.path.clone()),
                            url: Some(downloads.artifact.url.clone()),
                            sha1: Some(downloads.artifact.sha1.clone()),
                            size: downloads.artifact.size as i64,
                        }
                    } else {
                        // If no downloads section, create with minimal info
                        LibraryInfo::Generic {
                            name: lib.name,
                            path: None,
                            url: None,
                            sha1: None,
                            size: 0,
                        }
                    }
                })
                .collect(),
            MinecraftVersion::LegacyForge(v) => v
                .libraries
                .into_iter()
                .map(|lib| {
                    // For LegacyForge libraries with downloads
                    if let Some(downloads) = &lib.downloads {
                        if let Some(artifact) = &downloads.artifact {
                            return LibraryInfo::Generic {
                                name: lib.name.clone(),
                                path: Some(artifact.path.clone()),
                                url: Some(artifact.url.clone()),
                                sha1: Some(artifact.sha1.clone()),
                                size: artifact.size as i64,
                            };
                        }
                    }

                    // Handle legacy forge libraries without downloads
                    LibraryInfo::Generic {
                        name: lib.name.clone(),
                        path: None,
                        url: None,
                        sha1: None,
                        size: 0,
                    }
                })
                .collect(),
            MinecraftVersion::ModernFabric(v) => v
                .libraries
                .into_iter()
                .map(|lib| {
                    // Handle fabric libraries
                    LibraryInfo::Generic {
                        name: lib.name.clone(),
                        path: None,
                        url: lib.url.clone(),
                        sha1: lib.sha1.clone(),
                        size: lib.size.unwrap_or(0) as i64,
                    }
                })
                .collect(),
            MinecraftVersion::LegacyFabric(v) => v
                .libraries
                .into_iter()
                .map(|lib| {
                    // Handle legacy fabric libraries
                    LibraryInfo::Generic {
                        name: lib.name.clone(),
                        path: None,
                        url: None,
                        sha1: None,
                        size: 0,
                    }
                })
                .collect(),
            MinecraftVersion::NeoForge(v) => v
                .libraries
                .into_iter()
                .map(|lib| {
                    // Handle NeoForge libraries
                    LibraryInfo::Generic {
                        name: lib.name.clone(),
                        path: Some(lib.downloads.artifact.path.clone()),
                        url: Some(lib.downloads.artifact.url.clone()),
                        sha1: Some(lib.downloads.artifact.sha1.clone()),
                        size: lib.downloads.artifact.size as i64,
                    }
                })
                .collect(),
        }
    }

    /// Convert BaseLibrary to LibraryInfo
    ///
    /// Helper function to convert BaseLibrary to the unified LibraryInfo structure
    fn base_library_to_library_info(lib: BaseLibrary) -> LibraryInfo {
        if let Some(info) = extract_base_library_path(&lib) {
            let (path, url, sha1, size) = info;
            LibraryInfo::Generic {
                name: lib.name,
                path: Some(path),
                url: Some(url),
                sha1: Some(sha1),
                size,
            }
        } else {
            LibraryInfo::Generic {
                name: lib.name,
                path: None,
                url: None,
                sha1: None,
                size: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {}
