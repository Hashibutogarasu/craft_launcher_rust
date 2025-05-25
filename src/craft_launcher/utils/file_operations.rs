/**
 * File utilities for reading and writing files.
 * This module provides functions to read and write binary and text files,
 * as well as functions to serialize and deserialize data structures to and from JSON.
 * It also includes functions to check if a file or directory exists,
 * and to move, copy, or delete files.
 * The functions are designed to be used in a safe manner, with error handling
 */
pub mod file_utils {
    /// This module use std::fs and std::io for file operations.
    use std::{
        fs::{self},
        io::{Read, Write},
        path::PathBuf,
    };

    /// This module use crate::craft_launcher::utils::directory::directory for directory operations.
    use crate::craft_launcher::utils::directory_operations::directory::Directory;

    /**
     * Check if a file or directory exists.
     * path: The path to check.
     * Returns true if the file or directory exists, false otherwise.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn exists(path: &PathBuf) -> bool {
        path.exists()
    }

    /**
     * Check if a path is a file or a directory.
     * path: The path to check.
     * Returns true if the path is a file, false otherwise.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn is_file(path: &PathBuf) -> bool {
        path.is_file()
    }

    /**
     * Check if a path is a directory.
     * path: The path to check.
     * Returns true if the path is a directory, false otherwise.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn is_dir(path: &PathBuf) -> bool {
        path.is_dir()
    }

    /**
     * Write binary data to a file.
     * path: The path to the file.
     * data: The data to write.
     * Returns Ok(()) if the write was successful, Err(e) otherwise.
     */
    pub fn write_binary(path: &PathBuf, data: &[u8]) -> std::io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }

    /**
     * Read binary data from a file.
     * path: The path to the file.
     * Returns Ok(data) if the read was successful, Err(e) otherwise.
     * The data is returned as a `Vec<u8>`.
     */
    pub fn read_binary(path: &PathBuf) -> std::io::Result<Vec<u8>> {
        let mut file = fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /**
     * Close a file.
     * path: The path to the file.
     * Returns Ok(()) if the close was successful, Err(e) otherwise.
     */
    pub fn close_file(path: &PathBuf) -> std::io::Result<()> {
        if path.exists() && path.is_file() {
            drop(fs::File::options().read(true).open(path)?);
        }
        Ok(())
    }

    /**
     * Close a file.
     * path: The path to the file.
     * Returns 0 if the close was successful, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn close_file_c(path: &PathBuf) -> i32 {
        match close_file(path) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    /**
     * Get the size of a file in bytes.
     * path: The path to the file.
     * Returns Ok(size) with the file size in bytes if successful, Err(e) otherwise.
     */
    pub fn get_file_size(path: &PathBuf) -> std::io::Result<u64> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }

    /**
     * Get the size of a file in bytes.
     * path: The path to the file.
     * Returns the file size in bytes if successful, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn get_file_size_c(path: &PathBuf) -> i64 {
        match get_file_size(path) {
            Ok(size) => size as i64,
            Err(_) => -1,
        }
    }

    /**
     * Write text data to a file.
     * path: The path to the file.
     * data: The data to write.
     * Returns Ok(()) if the write was successful, Err(e) otherwise.
     */
    pub fn write_text(path: &PathBuf, data: &str) -> std::io::Result<()> {
        write_binary(path, data.as_bytes())
    }

    /**
     * Read text data from a file.
     * path: The path to the file.
     * Returns Ok(data) if the read was successful, Err(e) otherwise.
     * The data is returned as a String.
     */
    pub fn read_text(path: &PathBuf) -> std::io::Result<String> {
        let data = read_binary(path)?;
        String::from_utf8(data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /**
     * Write text data to a file.
     * path: The path to the file.
     * data: The data to write.
     * Returns 0 if the write was successful, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn write_text_c(path: &PathBuf, data: *const u8, len: usize) -> i32 {
        let data_slice = unsafe { std::slice::from_raw_parts(data, len) };
        let data_str = match std::str::from_utf8(data_slice) {
            Ok(s) => s,
            Err(_) => return -1,
        };

        match write_text(path, data_str) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    /**
     * Read text data from a file.
     * path: The path to the file.
     * Returns 0 if the read was successful, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn read_text_c(
        path: &PathBuf,
        out_data: *mut *mut u8,
        out_len: *mut usize,
    ) -> i32 {
        match read_text(path) {
            Ok(text) => {
                let bytes = text.into_bytes();
                let len = bytes.len();
                let ptr = Box::into_raw(bytes.into_boxed_slice()) as *mut u8;

                unsafe {
                    *out_data = ptr;
                    *out_len = len;
                }
                0
            }
            Err(_) => -1,
        }
    }

    /**
     * Write a struct to a file as JSON.
     * path: The path to the file.
     * data: The data to write.
     * Returns Ok(()) if the write was successful, Err(e) otherwise.
     * This function uses serde_json to serialize the data to JSON.
     */
    pub fn write_struct_to_file_as_json<T: serde::Serialize>(
        path: &PathBuf,
        data: &T,
    ) -> std::io::Result<()> {
        let json = serde_json::to_string(data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        write_text(path, &json)
    }

    /**
     * Read a struct from a file as JSON.
     * path: The path to the file.
     * Returns Ok(data) if the read was successful, Err(e) otherwise.
     * This function uses serde_json to deserialize the data from JSON.
     * The data is returned as a struct of type T.
     */
    pub fn read_struct_from_file_as_json<T: serde::de::DeserializeOwned>(
        path: &PathBuf,
    ) -> std::io::Result<T> {
        let json = read_text(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /**
     * Write JSON data to a file.
     * path: The path to the file.
     * json_str: The JSON data to write.
     * Returns 0 if the write was successful, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn write_json_c(path: &PathBuf, json_str: *const u8, len: usize) -> i32 {
        let data_slice = unsafe { std::slice::from_raw_parts(json_str, len) };
        let json_data = match std::str::from_utf8(data_slice) {
            Ok(s) => s,
            Err(_) => return -1,
        };

        match write_text(path, json_data) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    /**
     * Read JSON data from a file.
     * path: The path to the file.
     * Returns 0 if the read was successful, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     * The data is returned as a pointer to a byte array and the length of the array.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn read_json_c(
        path: &PathBuf,
        out_data: *mut *mut u8,
        out_len: *mut usize,
    ) -> i32 {
        match read_text(path) {
            Ok(json_str) => {
                let bytes = json_str.into_bytes();
                let len = bytes.len();
                let ptr = Box::into_raw(bytes.into_boxed_slice()) as *mut u8;

                unsafe {
                    *out_data = ptr;
                    *out_len = len;
                }
                0
            }
            Err(_) => -1,
        }
    }

    /**
     * Validate JSON data.
     * json_str: The JSON data to validate.
     * Returns 0 if the JSON is valid, -1 otherwise.
     * This function is unsafe because it uses raw pointers.
     * The data is passed as a pointer to a byte array and the length of the array.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn validate_json_c(json_str: *const u8, len: usize) -> i32 {
        let data_slice = unsafe { std::slice::from_raw_parts(json_str, len) };
        let json_data = match std::str::from_utf8(data_slice) {
            Ok(s) => s,
            Err(_) => return -1,
        };

        match serde_json::from_str::<serde_json::Value>(json_data) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    /**
     * Get directories in the base directory.
     * dir: The base directory.
     * Returns a vector of PathBufs representing the directories.
     */
    pub fn get_files() -> std::io::Result<Vec<PathBuf>> {
        let dir = Directory::default();
        dir.get_entries(|path| path.is_file())
    }

    /**
     * Move a file from src to dest.
     * src: The source file path.
     * dest: The destination file path.
     * Returns true if the move was successful, false otherwise.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn move_file(src: &PathBuf, dest: &PathBuf) -> bool {
        if exists(dest) {
            return false;
        }

        fs::rename(src, dest).is_ok()
    }

    /**
     * Copy a file from src to dest.
     * src: The source file path.
     * dest: The destination file path.
     * force: If true, it will overwrite the destination file if it already exists.
     * Returns true if the copy was successful, false otherwise.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn copy_file(src: &PathBuf, dest: &PathBuf, force: bool) -> bool {
        if src == dest {
            return false;
        }

        if exists(dest) && !force {
            return false;
        }

        fs::copy(src, dest).is_ok()
    }

    /**
     * Delete a file.
     * file: The file to delete.
     * force: If true, it will delete the file even if it doesn't exist.
     * Returns true if the file was deleted successfully, false otherwise.
     * This function is unsafe because it uses raw pointers.
     * The data is passed as a pointer to a byte array and the length of the array.
     */
    #[unsafe(no_mangle)]
    pub extern "C" fn delete_file(file: &PathBuf, force: bool) -> bool {
        if !file.exists() && !force {
            return false;
        }

        fs::remove_file(file).is_ok()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_file_operations_in_temp_dir() {
        use super::file_utils::{delete_file, exists, read_text, write_text};
        use std::fs;

        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("file_operations_test");
        fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory.");

        // Define a test file path
        let test_file = temp_dir.join("test_file.txt");

        // Write to the test file
        let test_content = "Hello, world!";
        write_text(&test_file, test_content).expect("Failed to write to test file.");
        assert!(exists(&test_file), "Test file should exist after writing.");

        // Read from the test file
        let read_content = read_text(&test_file).expect("Failed to read from test file.");
        assert_eq!(
            read_content, test_content,
            "Read content should match written content."
        );

        // Delete the test file
        delete_file(&test_file, true);
        assert!(
            !exists(&test_file),
            "Test file should not exist after deletion."
        );

        // Clean up the temporary directory
        fs::remove_dir_all(&temp_dir).expect("Failed to clean up temporary directory.");
    }
}
