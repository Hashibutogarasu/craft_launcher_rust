/**
 * Directory utility functions.
 * This module provides functions to create, delete, and manage directories.
 * It includes functions to create temporary directories, remove directories safely,
 */
pub mod directory {
    /// This module use std::fs and std::path::PathBuf for file system operations.
    use std::{fs, path::PathBuf, thread, time::Duration};

    /// This module use crate::craft_launcher::utils::file_utils::file_utils for file operations.
    use crate::craft_launcher::utils::file_operations::file_utils::close_file;

    /// A struct representing a directory.
    pub struct Directory {
        pub base_dir: String,
    }

    /// Implementing methods for the Directory struct.
    impl Directory {
        /// A default constructor for Directory.
        pub fn default() -> Self {
            Self {
                base_dir: String::new(),
            }
        }

        /**
         * Get entries files or directories in the base directory.
         * The generic type F is a function that takes a reference to a PathBuf and returns a boolean.
         * This allows for filtering the entries based on a condition.
         */
        pub fn get_entries<F>(&self, filter: F) -> std::io::Result<Vec<PathBuf>>
        where
            F: Fn(&PathBuf) -> bool,
        {
            let mut entries = Vec::new();

            for entry in fs::read_dir(&(self.base_dir.clone()))? {
                let entry = entry?;
                let path = entry.path();
                if filter(&path) {
                    entries.push(path);
                }
            }

            Ok(entries)
        }

        /**
         * Get all files in the current directory.
         * This function returns a vector of PathBufs representing the files.
         */
        pub fn get_files(&self) -> std::io::Result<Vec<PathBuf>> {
            match self.get_entries(|path| path.is_file()) {
                Ok(files) => Ok(files
                    .into_iter()
                    .filter_map(|p| p.canonicalize().ok())
                    .collect()),
                Err(e) => {
                    eprintln!("Error in get_files: {}", e);
                    Err(e)
                }
            }
        }

        /**
         * Get all directories in the current directory.
         * This function returns a vector of PathBufs representing the directories.
         */
        pub fn get_directories(&self) -> std::io::Result<Vec<PathBuf>> {
            match self.get_entries(|path| path.is_dir()) {
                Ok(directories) => Ok(directories
                    .into_iter()
                    .filter_map(|p| p.canonicalize().ok())
                    .collect()),
                Err(e) => {
                    eprintln!("Error in get_directories: {}", e);
                    Err(e)
                }
            }
        }

        /**
         * Create a new directory.
         * dir: The directory to create.
         * force: If true, it will overwrite the directory if it already exists.
         * Returns true if the directory was created successfully, false otherwise.
         */
        #[unsafe(no_mangle)]
        pub extern "C" fn create_dir(dir: &PathBuf, force: bool) -> bool {
            if dir.exists() && !force {
                return false;
            }

            fs::create_dir_all(dir).is_ok()
        }

        /**
         * Delete a directory.
         * dir: The directory to delete.
         * force: If true, it will delete the directory even if it doesn't exist.
         * Returns true if the directory was deleted successfully, false otherwise.
         */
        #[unsafe(no_mangle)]
        pub extern "C" fn delete_dir(dir: &PathBuf, force: bool) -> bool {
            if !dir.exists() && !force {
                return false;
            }

            for _ in 0..3 {
                match fs::remove_dir_all(dir) {
                    Ok(_) => return true,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::PermissionDenied
                            || e.kind() == std::io::ErrorKind::Other
                        {
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            continue;
                        }
                        return false;
                    }
                }
            }
            false
        }

        /**
         * Create a temporary directory.
         * Returns the path of the created temporary directory.
         */
        pub fn create_temp_dir() -> std::io::Result<PathBuf> {
            let temp_dir = std::env::temp_dir();
            let temp_path = temp_dir.join("temp");
            fs::create_dir_all(&temp_path)?;
            Ok(temp_path)
        }

        /**
         * Safe remove a directory.
         * path: The directory to remove.
         * This function attempts to remove the directory up to 3 times.
         * If the directory is not empty, it will attempt to close any open files before retrying.
         */
        pub fn safe_remove_dir_all(path: &std::path::PathBuf) {
            for attempt in 1..=3 {
                if path.exists() {
                    println!("Attempting to remove directory: {:?}", path);
                } else {
                    return;
                }
                match fs::remove_dir_all(path) {
                    Ok(_) => return,
                    Err(e) => {
                        eprintln!(
                            "Remove directory attempt {}: Failed with error: {}",
                            attempt, e
                        );
                        thread::sleep(Duration::from_millis(100));

                        if let Ok(entries) = fs::read_dir(path) {
                            for entry in entries.flatten() {
                                if let Ok(file_type) = entry.file_type() {
                                    if file_type.is_file() {
                                        let _ = close_file(&entry.path());
                                    }
                                }
                            }
                        }

                        if attempt == 3 {
                            if !Directory::delete_dir(path, true) {
                                eprintln!(
                                    "Failed to remove temp directory after multiple attempts: {:?}",
                                    path
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::directory;
    use std::fs;

    #[test]
    fn test_get_files_and_directories() {
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("directory_operations_test");
        fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory.");

        // Create test files and directories
        let test_file = temp_dir.join("test_file.txt");
        let test_dir = temp_dir.join("test_dir");
        fs::write(&test_file, "Hello, world!").expect("Failed to create test file.");
        fs::create_dir_all(&test_dir).expect("Failed to create test directory.");

        // Change the current directory to the temporary directory
        let original_dir = std::env::current_dir().expect("Failed to get current directory.");
        std::env::set_current_dir(&temp_dir).expect("Failed to change current directory.");

        // Create Directory instance with the current directory path
        let dir = directory::Directory {
            base_dir: ".".to_string(),
        };

        // Test get_files
        let files = dir.get_files().expect("Failed to get files.");
        assert_eq!(
            files,
            vec![
                test_file
                    .canonicalize()
                    .expect("Failed to canonicalize test file path.")
            ]
        );

        // Test get_directories
        let directories = dir.get_directories().expect("Failed to get directories.");
        assert_eq!(
            directories,
            vec![
                test_dir
                    .canonicalize()
                    .expect("Failed to canonicalize test directory path.")
            ]
        );

        // Restore the original directory
        std::env::set_current_dir(original_dir).expect("Failed to restore original directory.");

        // Clean up the temporary directory
        fs::remove_dir_all(&temp_dir).expect("Failed to clean up temporary directory.");
    }
}
