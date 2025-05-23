/**
 * Networking utils for downloading files and reading content from URLs.
 * This module provides functions to download files from the internet and read their content.
 * It uses the reqwest library for making HTTP requests and the file_utils module for file operations.
 */
pub mod networking {
    /// This module uses std::fs and std::path::PathBuf for file system operations.
    use std::path::PathBuf;

    /// This module use crate::craft_launcher::utils::file_utils::file_utils for file operations.
    use crate::craft_launcher::utils::file_operations::file_utils;

    /**
     * Reads a file from a URL and returns its content as a String.
     * url: The URL of the file to read.
     * Returns the content of the file as a String.
     * If an error occurs, it returns a reqwest::Error.
     */
    pub fn read_file_from_url(url: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::blocking::get(url)?;
        let content = response.text()?;
        Ok(content)
    }

    /**
     * Downloads a file from a URL and saves it to the specified destination.
     * url: The URL of the file to download.
     * dest: The destination path where the file will be saved.
     * Returns Ok(()) if the download is successful, or an error if it fails.
     * If the file already exists, it will be overwritten.
     */
    pub fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(url)?;
        let bytes = response.bytes()?;
        file_utils::write_binary(dest, &bytes)?;
        file_utils::close_file(dest)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::networking;
    use crate::craft_launcher::utils::{
        directory_operations::directory::Directory,
        file_operations::file_utils::{self},
    };
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_read_file_from_url() {
        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        let result = networking::read_file_from_url(url);
        assert!(result.is_ok());

        let temp_dir = Directory::create_temp_dir().unwrap();
        let file_path = temp_dir.join("version_manifest_test.json");

        let content = result.unwrap();
        file_utils::write_text(&file_path, &content).unwrap();

        let json_data: Value = file_utils::read_struct_from_file_as_json(&file_path).unwrap();
        assert!(json_data.is_object());
        assert!(json_data.get("versions").is_some());

        if file_path.exists() {
            file_utils::close_file(&file_path).unwrap_or_else(|e| {
                eprintln!("Failed to close file: {}", e);
            });
        }

        Directory::safe_remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_download_file() {
        let temp_dir = Directory::create_temp_dir().unwrap();
        let file_path = temp_dir.join("version_manifest_v2.json");

        if file_path.exists() {
            fs::remove_file(&file_path).unwrap_or_else(|e| {
                eprintln!("Failed to remove existing file: {}", e);
            });
        }

        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        let result = networking::download_file(url, &file_path);

        assert!(result.is_ok());
        assert!(file_utils::exists(&file_path));
        assert!(file_utils::is_file(&file_path));

        let json_data: Result<Value, _> = file_utils::read_struct_from_file_as_json(&file_path);
        assert!(json_data.is_ok());

        let manifest: Value = json_data.unwrap();
        assert!(manifest.get("latest").is_some());
        assert!(manifest.get("versions").is_some());

        let test_path = temp_dir.join("rewritten_manifest.json");
        file_utils::write_struct_to_file_as_json(&test_path, &manifest).unwrap();

        assert!(file_utils::exists(&test_path));
        let reread: Result<Value, _> = file_utils::read_struct_from_file_as_json(&test_path);
        assert!(reread.is_ok());

        file_utils::close_file(&file_path).unwrap_or_else(|e| {
            eprintln!("Failed to close file: {}", e);
        });
        file_utils::close_file(&test_path).unwrap_or_else(|e| {
            eprintln!("Failed to close file: {}", e);
        });

        Directory::safe_remove_dir_all(&temp_dir);
    }
}
