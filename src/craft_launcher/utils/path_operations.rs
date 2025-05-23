/**
 * Path operation utils
 * This module provides utility functions for path operations.
 */
pub mod path_operations {
    /// This module use std::fs and std::path::PathBuf for file system operations.
    use std::path::PathBuf;

    /**
     * Get the path to the Minecraft directory.
     * This function returns the path to the Minecraft directory.
     * It uses the standard library's env module to get the home directory.
     */
    pub fn get_temporary_dir() -> PathBuf {
        let temp_dir = std::env::temp_dir();
        temp_dir.join("temp")
    }

    /// Cleans up the temporary directory created by `get_temporary_dir`.
    /// This function removes the "temp" directory inside the system's temporary directory.
    pub fn cleanup_temporary_dir() {
        let temp_dir = std::env::temp_dir().join("temp");
        if temp_dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&temp_dir) {
                eprintln!("Failed to clean up temporary directory: {}", e);
            }
        }
    }

    /// Creates the temporary directory if it does not exist.
    /// This function ensures the "temp" directory inside the system's temporary directory is created.
    pub fn create_temporary_dir() {
        let temp_dir = std::env::temp_dir().join("temp");
        if !temp_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&temp_dir) {
                eprintln!("Failed to create temporary directory: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_and_cleanup_nested_directory() {
        use std::fs;

        // Create the base temporary directory
        super::path_operations::create_temporary_dir();
        let base_temp_dir = super::path_operations::get_temporary_dir();
        assert!(
            base_temp_dir.exists(),
            "Base temporary directory should exist."
        );

        // Create a nested directory inside the temporary directory
        let nested_dir = base_temp_dir.join("path/to/dir");
        fs::create_dir_all(&nested_dir).expect("Failed to create nested directory.");
        assert!(
            nested_dir.exists(),
            "Nested directory should exist after creation."
        );

        // Clean up the base temporary directory
        super::path_operations::cleanup_temporary_dir();
        assert!(
            !base_temp_dir.exists(),
            "Base temporary directory should not exist after cleanup."
        );
        assert!(
            !nested_dir.exists(),
            "Nested directory should not exist after cleanup."
        );
    }

    #[test]
    fn test_create_temporary_dir_isolated() {
        use std::fs;

        // Create a unique temporary directory for this test
        let unique_temp_dir = std::env::temp_dir().join("temp_test_isolated");
        if !unique_temp_dir.exists() {
            fs::create_dir_all(&unique_temp_dir)
                .expect("Failed to create unique temporary directory.");
        }
        assert!(
            unique_temp_dir.exists(),
            "Unique temporary directory should exist."
        );

        // Perform cleanup
        if unique_temp_dir.exists() {
            fs::remove_dir_all(&unique_temp_dir)
                .expect("Failed to clean up unique temporary directory.");
        }
        assert!(
            !unique_temp_dir.exists(),
            "Unique temporary directory should not exist after cleanup."
        );
    }
}
