pub mod arguments_builder {
    use std::path::{Path, PathBuf};

    /// A builder for Java command line arguments.
    /// This struct helps to construct command line arguments for Java applications
    /// including executable path, JVM arguments, classpath, main class and program arguments.
    pub struct JavaArgumentsBuilder {
        /// The Java executable path (java or javaw)
        executable: String,
        /// JVM arguments that come before the classpath
        pre_classpath_args: Vec<String>,
        /// Files and directories to be included in the classpath
        classpath_entries: Vec<PathBuf>,
        /// Whether to check if classpath entries exist before adding them
        check_exists: bool,
        /// Main class to execute
        main_class: Option<String>,
        /// Program arguments that come after the main class
        program_args: Vec<String>,
    }

    impl JavaArgumentsBuilder {
        /// Creates a new JavaArgumentsBuilder with default values.
        ///
        /// # Returns
        ///
        /// * A new `JavaArgumentsBuilder` instance with "java" as the default executable
        pub fn new() -> Self {
            JavaArgumentsBuilder {
                executable: "java".to_string(),
                pre_classpath_args: Vec::new(),
                classpath_entries: Vec::new(),
                check_exists: true,
                main_class: None,
                program_args: Vec::new(),
            }
        }

        /// Sets the Java executable to use.
        ///
        /// # Arguments
        ///
        /// * `executable` - The executable name or path (e.g., "java", "javaw", or full path to Java executable)
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn with_executable<S: Into<String>>(mut self, executable: S) -> Self {
            self.executable = executable.into();
            self
        }

        /// Adds a pre-classpath JVM argument.
        ///
        /// # Arguments
        ///
        /// * `arg` - The JVM argument to add (e.g., "-Xmx2G")
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_jvm_arg<S: Into<String>>(mut self, arg: S) -> Self {
            self.pre_classpath_args.push(arg.into());
            self
        }

        /// Adds multiple pre-classpath JVM arguments.
        ///
        /// # Arguments
        ///
        /// * `args` - The JVM arguments to add
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_jvm_args<I, S>(mut self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: Into<String>,
        {
            for arg in args {
                self.pre_classpath_args.push(arg.into());
            }
            self
        }

        /// Sets whether to check if classpath entries exist before adding them.
        ///
        /// # Arguments
        ///
        /// * `check` - If true, will verify file/directory existence before adding to classpath
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn check_exists(mut self, check: bool) -> Self {
            self.check_exists = check;
            self
        }

        /// Adds a classpath entry if it exists (when check_exists is true) or unconditionally (when check_exists is false).
        ///
        /// # Arguments
        ///
        /// * `path` - Path to add to the classpath
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_classpath_entry<P: AsRef<Path>>(mut self, path: P) -> Self {
            let path_ref = path.as_ref();

            if !self.check_exists || path_ref.exists() {
                self.classpath_entries.push(path_ref.to_path_buf());
            }

            self
        }

        /// Adds multiple classpath entries.
        ///
        /// # Arguments
        ///
        /// * `paths` - Paths to add to the classpath
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_classpath_entries<I, P>(mut self, paths: I) -> Self
        where
            I: IntoIterator<Item = P>,
            P: AsRef<Path>,
        {
            for path in paths {
                let path_ref = path.as_ref();

                if !self.check_exists || path_ref.exists() {
                    self.classpath_entries.push(path_ref.to_path_buf());
                }
            }

            self
        }

        /// Sets the main class to execute.
        ///
        /// # Arguments
        ///
        /// * `main_class` - The fully qualified name of the main class
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn with_main_class<S: Into<String>>(mut self, main_class: S) -> Self {
            self.main_class = Some(main_class.into());
            self
        }

        /// Adds a program argument.
        ///
        /// # Arguments
        ///
        /// * `arg` - The program argument to add
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_program_arg<S: Into<String>>(mut self, arg: S) -> Self {
            self.program_args.push(arg.into());
            self
        }

        /// Adds multiple program arguments.
        ///
        /// # Arguments
        ///
        /// * `args` - The program arguments to add
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_program_args<I, S>(mut self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: Into<String>,
        {
            for arg in args {
                self.program_args.push(arg.into());
            }
            self
        }

        /// Builds the complete command line as a vector of strings.
        ///
        /// # Returns
        ///
        /// * A vector of command line arguments starting with the executable
        pub fn build(&self) -> Vec<String> {
            let mut command: Vec<String> = Vec::new();

            // Add the executable
            command.push(self.executable.clone());

            // Add JVM arguments
            command.extend(self.pre_classpath_args.clone());

            // Add classpath if there are any entries
            if !self.classpath_entries.is_empty() {
                command.push("-cp".to_string());

                // Join classpath entries with appropriate separator
                #[cfg(target_os = "windows")]
                let separator = ";";
                #[cfg(not(target_os = "windows"))]
                let separator = ":";

                let classpath = self
                    .classpath_entries
                    .iter()
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(separator);

                command.push(classpath);
            }

            // Add main class if specified
            if let Some(ref main_class) = self.main_class {
                command.push(main_class.clone());
            }

            // Add program arguments
            command.extend(self.program_args.clone());

            command
        }

        /// Builds the complete command line as a single string.
        ///
        /// # Returns
        ///
        /// * A command line string with all arguments properly quoted
        pub fn build_string(&self) -> String {
            // Get the command components
            let command_parts = self.build();

            // Process each part to add quotes when needed
            let quoted_parts: Vec<String> = command_parts
                .iter()
                .map(|part| {
                    if part.contains(' ') || part.contains('\t') {
                        // For Windows, escape quotes inside the string and wrap in quotes
                        #[cfg(target_os = "windows")]
                        {
                            format!("\"{}\"", part.replace("\"", "\\\""))
                        }

                        // For Unix-like systems
                        #[cfg(not(target_os = "windows"))]
                        {
                            format!("\"{}\"", part.replace("\"", "\\\""))
                        }
                    } else {
                        part.clone()
                    }
                })
                .collect();

            // Join all parts with spaces
            quoted_parts.join(" ")
        }
    }

    /// Creates a shortcut to initialize a new JavaArgumentsBuilder
    ///
    /// # Returns
    ///
    /// * A new `JavaArgumentsBuilder` instance
    pub fn java_args() -> JavaArgumentsBuilder {
        JavaArgumentsBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::arguments_builder::java_args;

    #[test]
    /// Tests building basic Java command with a main class
    fn test_basic_java_command() {
        let command = java_args().with_main_class("com.example.Main").build();

        assert_eq!(command[0], "java");
        assert_eq!(command[1], "com.example.Main");
        assert_eq!(command.len(), 2);
    }

    #[test]
    /// Tests building command with JVM arguments
    fn test_jvm_args() {
        let command = java_args()
            .add_jvm_arg("-Xmx2G")
            .add_jvm_arg("-Xms1G")
            .with_main_class("com.example.Main")
            .build();

        assert_eq!(command[0], "java");
        assert_eq!(command[1], "-Xmx2G");
        assert_eq!(command[2], "-Xms1G");
        assert_eq!(command[3], "com.example.Main");
        assert_eq!(command.len(), 4);
    }

    #[test]
    /// Tests building command with classpath
    fn test_classpath() {
        let temp_dir = std::env::temp_dir();
        let command = java_args()
            .check_exists(false) // Don't check existence for testing
            .add_classpath_entry(temp_dir.join("test1.jar"))
            .add_classpath_entry(temp_dir.join("test2.jar"))
            .with_main_class("com.example.Main")
            .build();

        assert_eq!(command[0], "java");
        assert_eq!(command[1], "-cp");

        // The third element should be the classpath string
        #[cfg(target_os = "windows")]
        assert!(command[2].contains(";"));
        #[cfg(not(target_os = "windows"))]
        assert!(command[2].contains(":"));

        assert_eq!(command[3], "com.example.Main");
        assert_eq!(command.len(), 4);
    }

    #[test]
    /// Tests building command with program arguments
    fn test_program_args() {
        let command = java_args()
            .with_main_class("com.example.Main")
            .add_program_arg("--config")
            .add_program_arg("config.json")
            .build();

        assert_eq!(command[0], "java");
        assert_eq!(command[1], "com.example.Main");
        assert_eq!(command[2], "--config");
        assert_eq!(command[3], "config.json");
        assert_eq!(command.len(), 4);
    }

    #[test]
    /// Tests building a complete command with all argument types
    fn test_complete_command() {
        let temp_dir = std::env::temp_dir();
        let command = java_args()
            .with_executable("javaw")
            .add_jvm_arg("-Xmx4G")
            .check_exists(false) // Don't check existence for testing
            .add_classpath_entry(temp_dir.join("lib1.jar"))
            .add_classpath_entry(temp_dir.join("lib2.jar"))
            .with_main_class("com.example.MainClass")
            .add_program_arg("--user")
            .add_program_arg("player1")
            .build();

        assert_eq!(command[0], "javaw");
        assert_eq!(command[1], "-Xmx4G");
        assert_eq!(command[2], "-cp");
        // command[3] is the classpath
        assert_eq!(command[4], "com.example.MainClass");
        assert_eq!(command[5], "--user");
        assert_eq!(command[6], "player1");
        assert_eq!(command.len(), 7);
    }

    #[test]
    /// Tests building a command line string with proper quoting
    fn test_build_string() {
        let temp_dir = std::env::temp_dir();
        let command_str = java_args()
            .check_exists(false) // Don't check existence for testing
            .add_jvm_arg("-Duser.dir=C:\\Program Files\\Java")
            .add_classpath_entry(temp_dir.join("my lib.jar"))
            .with_main_class("com.example.Main")
            .build_string();

        // Verify the command contains quoted paths with spaces
        assert!(command_str.contains("\""));
        assert!(command_str.contains("-Duser.dir=C:\\Program Files\\Java"));
        assert!(command_str.contains("my lib.jar"));
    }
}
