pub mod arguments_merger {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use crate::arguments_builder::arguments_builder::JavaArgumentsBuilder;

    /// A structure to handle merging of Java arguments from multiple sources.
    /// When merging arguments from different sources, newer arguments take precedence over older ones.
    /// This is useful when combining default arguments with user-provided arguments.
    pub struct JavaArgumentsMerger {
        /// Maps of argument name to its value
        arguments_map: HashMap<String, String>,
        /// JVM arguments that don't follow the key=value pattern
        flag_arguments: Vec<String>,
        /// Classpath entries to be merged
        classpath_entries: Vec<PathBuf>,
        /// Program arguments to be appended
        program_args: Vec<String>,
    }

    impl JavaArgumentsMerger {
        /// Creates a new JavaArgumentsMerger with empty maps.
        ///
        /// # Returns
        ///
        /// * A new `JavaArgumentsMerger` instance
        pub fn new() -> Self {
            JavaArgumentsMerger {
                arguments_map: HashMap::new(),
                flag_arguments: Vec::new(),
                classpath_entries: Vec::new(),
                program_args: Vec::new(),
            }
        }
        /// Adds JVM arguments to the merger, overriding any existing arguments with the same key.
        ///
        /// # Arguments
        ///
        /// * `args` - The JVM arguments to add
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_jvm_args<I, S>(&mut self, args: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: Into<String>,
        {
            for arg in args {
                let arg_str = arg.into();

                // Handle key=value type arguments
                if let Some(pos) = arg_str.find('=') {
                    let (key, value) = arg_str.split_at(pos);
                    self.arguments_map
                        .insert(key.to_string(), value[1..].to_string());
                } else {
                    // Handle special JVM flags that should be treated as overridable
                    if arg_str.starts_with("-Xmx")
                        || arg_str.starts_with("-Xms")
                        || arg_str.starts_with("-Xss")
                    {
                        // Extract the prefix (-Xmx, -Xms, -Xss)
                        let prefix = &arg_str[0..4];

                        // Remove any existing arguments with the same prefix
                        self.flag_arguments
                            .retain(|existing| !existing.starts_with(prefix));
                        // Add the new argument
                        self.flag_arguments.push(arg_str);
                    } else if arg_str.starts_with("-XX:") {
                        // Handle -XX: flags
                        // Extract the feature name (everything up to the first : or =)
                        if let Some(colon_pos) = arg_str[4..].find(':') {
                            let feature = &arg_str[0..(4 + colon_pos + 1)]; // Include the colon

                            // Remove any existing arguments with the same feature
                            self.flag_arguments
                                .retain(|existing| !existing.starts_with(feature));
                            // Add the new argument
                            self.flag_arguments.push(arg_str);
                        } else if let Some(equals_pos) = arg_str[4..].find('=') {
                            let feature = &arg_str[0..(4 + equals_pos + 1)]; // Include the equals

                            // Remove any existing arguments with the same feature
                            self.flag_arguments
                                .retain(|existing| !existing.starts_with(feature));
                            // Add the new argument
                            self.flag_arguments.push(arg_str);
                        } else if arg_str.contains('+') || arg_str.contains('-') {
                            // Handle boolean flags like -XX:+UseG1GC or -XX:-UseConcMarkSweepGC
                            // Extract the feature name without the + or - sign
                            let plus_pos = arg_str[4..].find('+');
                            let minus_pos = arg_str[4..].find('-');

                            if let Some(pos) = plus_pos.or(minus_pos) {
                                let feature_base = &arg_str[0..(4 + pos)]; // Without the + or -

                                // Remove any existing arguments with the same feature base (with either + or -)
                                self.flag_arguments.retain(|existing| {
                                    !(existing.starts_with(feature_base)
                                        && (existing.chars().nth(4 + pos) == Some('+')
                                            || existing.chars().nth(4 + pos) == Some('-')))
                                });
                                // Add the new argument
                                self.flag_arguments.push(arg_str);
                            } else {
                                // For other -XX flags that don't match the patterns above
                                self.flag_arguments.push(arg_str);
                            }
                        } else {
                            // For other -XX flags that don't match the patterns above
                            self.flag_arguments.push(arg_str);
                        }
                    } else {
                        // For all other flags that don't need special handling
                        self.flag_arguments.push(arg_str);
                    }
                }
            }
            self
        }

        /// Adds classpath entries to the merger.
        ///
        /// # Arguments
        ///
        /// * `paths` - Paths to add to the classpath
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_classpath_entries<I>(&mut self, paths: I) -> &mut Self
        where
            I: IntoIterator<Item = PathBuf>,
        {
            for path in paths {
                self.classpath_entries.push(path);
            }
            self
        }

        /// Adds program arguments to the merger.
        ///
        /// # Arguments
        ///
        /// * `args` - The program arguments to add
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn add_program_args<I, S>(&mut self, args: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: Into<String>,
        {
            for arg in args {
                self.program_args.push(arg.into());
            }
            self
        }
        /// Merges another JavaArgumentsMerger into this one, with the other merger's arguments taking precedence.
        ///
        /// # Arguments
        ///
        /// * `other` - Another JavaArgumentsMerger to merge from
        ///
        /// # Returns
        ///
        /// * Self for method chaining
        pub fn merge(&mut self, other: &JavaArgumentsMerger) -> &mut Self {
            // Merge key-value arguments (newer overrides older)
            for (key, value) in &other.arguments_map {
                self.arguments_map.insert(key.clone(), value.clone());
            }

            // Process flag arguments - use add_jvm_args to ensure proper overriding
            self.add_jvm_args(other.flag_arguments.clone());

            // Append classpath entries
            // Remove duplicates to avoid double entries
            for path in &other.classpath_entries {
                if !self.classpath_entries.contains(path) {
                    self.classpath_entries.push(path.clone());
                }
            }

            // Append program arguments
            self.program_args.extend(other.program_args.clone());

            self
        }

        /// Merges multiple JavaArgumentsMerger instances into a new one,
        /// with newer instances (later in the list) taking precedence over older ones.
        ///
        /// # Arguments
        ///
        /// * `mergers` - A list of JavaArgumentsMerger instances to merge in order from oldest to newest
        ///
        /// # Returns
        ///
        /// * A new JavaArgumentsMerger with all arguments merged
        pub fn merge_multiple(mergers: Vec<&JavaArgumentsMerger>) -> Self {
            let mut result = JavaArgumentsMerger::new();

            // Merge each merger in order (oldest to newest)
            for merger in mergers {
                result.merge(merger);
            }

            result
        }

        /// Creates a new JavaArgumentsMerger from a list of arguments lists,
        /// with newer lists (later in the vector) taking precedence over older ones.
        ///
        /// # Arguments
        ///
        /// * `jvm_args_list` - A list of JVM arguments lists in chronological order (oldest to newest)
        /// * `classpath_list` - A list of classpath entry lists in chronological order (oldest to newest)
        /// * `program_args_list` - A list of program arguments lists in chronological order (oldest to newest)
        ///
        /// # Returns
        ///
        /// * A new JavaArgumentsMerger with all arguments merged
        pub fn merge_from_lists(
            jvm_args_list: Vec<Vec<String>>,
            classpath_list: Vec<Vec<PathBuf>>,
            program_args_list: Vec<Vec<String>>,
        ) -> Self {
            let mut result = JavaArgumentsMerger::new();

            // Process JVM arguments
            for args in jvm_args_list {
                result.add_jvm_args(args);
            }

            // Process classpath entries
            for paths in classpath_list {
                result.add_classpath_entries(paths);
            }

            // Process program arguments
            for args in program_args_list {
                result.add_program_args(args);
            }

            result
        }

        /// Builds a vector of JVM arguments from the merged state.
        ///
        /// # Returns
        ///
        /// * A vector of JVM argument strings
        pub fn build_jvm_args(&self) -> Vec<String> {
            let mut result = Vec::new();

            // Add all flag arguments
            result.extend(self.flag_arguments.clone());

            // Add all key-value arguments
            for (key, value) in &self.arguments_map {
                result.push(format!("{}={}", key, value));
            }

            result
        }

        /// Gets the merged classpath entries.
        ///
        /// # Returns
        ///
        /// * A vector of PathBuf for classpath entries
        pub fn get_classpath_entries(&self) -> Vec<PathBuf> {
            self.classpath_entries.clone()
        }

        /// Gets the merged program arguments.
        ///
        /// # Returns
        ///
        /// * A vector of program argument strings
        pub fn get_program_args(&self) -> Vec<String> {
            self.program_args.clone()
        }

        /// Applies the merged arguments to a JavaArgumentsBuilder.
        ///
        /// # Arguments
        ///
        /// * `builder` - The JavaArgumentsBuilder to apply arguments to
        ///
        /// # Returns
        ///
        /// * The updated JavaArgumentsBuilder
        pub fn apply_to_builder(&self, builder: JavaArgumentsBuilder) -> JavaArgumentsBuilder {
            let mut result = builder;

            // Add all JVM arguments
            result = result.add_jvm_args(self.build_jvm_args());

            // Add all classpath entries
            result = result.add_classpath_entries(self.classpath_entries.clone());

            // Add all program arguments
            result = result.add_program_args(self.program_args.clone());

            result
        }
    }

    /// Creates a shortcut to initialize a new JavaArgumentsMerger
    ///
    /// # Returns
    ///
    /// * A new `JavaArgumentsMerger` instance
    pub fn java_args_merger() -> JavaArgumentsMerger {
        JavaArgumentsMerger::new()
    }
}

#[cfg(test)]
mod tests {
    use super::arguments_merger::{JavaArgumentsMerger, java_args_merger};
    use std::path::PathBuf;

    #[test]
    /// Tests merging JVM arguments with overriding
    fn test_merge_jvm_args() {
        let mut merger1 = java_args_merger();
        merger1.add_jvm_args(vec!["-Xmx1G", "-Xms512M", "-Duser.home=/home/user1"]);

        let mut merger2 = java_args_merger();
        merger2.add_jvm_args(vec!["-Xmx2G", "-XX:+UseG1GC"]);

        merger1.merge(&merger2);
        let args = merger1.build_jvm_args();

        // Check that -Xmx2G overrides -Xmx1G
        assert!(args.contains(&"-Xmx2G".to_string()));
        assert!(!args.contains(&"-Xmx1G".to_string()));

        // Check other arguments are preserved
        assert!(args.contains(&"-Xms512M".to_string()));
        assert!(args.contains(&"-XX:+UseG1GC".to_string()));
        assert!(args.contains(&"-Duser.home=/home/user1".to_string()));
    }

    #[test]
    /// Tests merging classpath entries
    fn test_merge_classpath() {
        let mut merger1 = java_args_merger();
        merger1.add_classpath_entries(vec![
            PathBuf::from("/path/to/lib1.jar"),
            PathBuf::from("/path/to/lib2.jar"),
        ]);

        let mut merger2 = java_args_merger();
        merger2.add_classpath_entries(vec![PathBuf::from("/path/to/lib3.jar")]);

        merger1.merge(&merger2);
        let classpath = merger1.get_classpath_entries();

        assert_eq!(classpath.len(), 3);
        assert!(classpath.contains(&PathBuf::from("/path/to/lib1.jar")));
        assert!(classpath.contains(&PathBuf::from("/path/to/lib2.jar")));
        assert!(classpath.contains(&PathBuf::from("/path/to/lib3.jar")));
    }

    #[test]
    /// Tests merging program arguments
    fn test_merge_program_args() {
        let mut merger1 = java_args_merger();
        merger1.add_program_args(vec!["--user", "player1"]);

        let mut merger2 = java_args_merger();
        merger2.add_program_args(vec!["--server", "minecraft.example.com"]);

        merger1.merge(&merger2);
        let args = merger1.get_program_args();

        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "--user");
        assert_eq!(args[1], "player1");
        assert_eq!(args[2], "--server");
        assert_eq!(args[3], "minecraft.example.com");
    }

    #[test]
    /// Tests complex merging of multiple argument types
    fn test_complex_merge() {
        let mut old_args = java_args_merger();
        old_args.add_jvm_args(vec!["-Xmx1G", "-Duser.dir=/old/path", "-XX:+UseParallelGC"]);
        old_args.add_classpath_entries(vec![
            PathBuf::from("/old/lib1.jar"),
            PathBuf::from("/shared/lib.jar"),
        ]);
        old_args.add_program_args(vec!["--old-option", "value"]);

        let mut new_args = java_args_merger();
        new_args.add_jvm_args(vec!["-Xmx2G", "-Duser.dir=/new/path"]);
        new_args.add_classpath_entries(vec![
            PathBuf::from("/new/lib2.jar"),
            PathBuf::from("/shared/lib.jar"), // Duplicate entry
        ]);
        new_args.add_program_args(vec!["--new-option", "value"]);

        old_args.merge(&new_args);

        let jvm_args = old_args.build_jvm_args();
        let classpath = old_args.get_classpath_entries();
        let program_args = old_args.get_program_args();

        // Check JVM args are properly merged with overrides
        assert!(jvm_args.contains(&"-Xmx2G".to_string()));
        assert!(!jvm_args.contains(&"-Xmx1G".to_string()));
        assert!(jvm_args.contains(&"-Duser.dir=/new/path".to_string()));
        assert!(!jvm_args.contains(&"-Duser.dir=/old/path".to_string()));
        assert!(jvm_args.contains(&"-XX:+UseParallelGC".to_string()));
        // Check classpath entries are all present (without duplicates)
        assert_eq!(classpath.len(), 3);
        assert!(classpath.contains(&PathBuf::from("/old/lib1.jar")));
        assert!(classpath.contains(&PathBuf::from("/new/lib2.jar")));
        assert!(classpath.contains(&PathBuf::from("/shared/lib.jar")));
        // Duplicates should be removed
        let shared_lib_count = classpath
            .iter()
            .filter(|p| p == &&PathBuf::from("/shared/lib.jar"))
            .count();
        assert_eq!(shared_lib_count, 1);

        // Check program args are simply appended
        assert_eq!(program_args.len(), 4);
        assert_eq!(program_args[0], "--old-option");
        assert_eq!(program_args[1], "value");
        assert_eq!(program_args[2], "--new-option");
        assert_eq!(program_args[3], "value");
    }

    #[test]
    /// Tests merging multiple JavaArgumentsMerger instances
    fn test_merge_multiple() {
        let mut merger1 = java_args_merger();
        merger1.add_jvm_args(vec!["-Xmx1G", "-XX:+UseConcMarkSweepGC"]);
        merger1.add_classpath_entries(vec![PathBuf::from("/lib1.jar")]);

        let mut merger2 = java_args_merger();
        merger2.add_jvm_args(vec!["-Xmx2G"]);
        merger2.add_classpath_entries(vec![PathBuf::from("/lib2.jar")]);
        merger2.add_program_args(vec!["--server", "localhost"]);

        let mut merger3 = java_args_merger();
        merger3.add_jvm_args(vec!["-Xmx4G", "-Duser.home=/home/player"]);
        merger3.add_program_args(vec!["--user", "player1"]);

        let result = JavaArgumentsMerger::merge_multiple(vec![&merger1, &merger2, &merger3]);

        // Check JVM arguments are properly merged with overrides
        let jvm_args = result.build_jvm_args();
        assert!(jvm_args.contains(&"-Xmx4G".to_string())); // Latest value
        assert!(!jvm_args.contains(&"-Xmx2G".to_string())); // Overridden
        assert!(!jvm_args.contains(&"-Xmx1G".to_string())); // Overridden
        assert!(jvm_args.contains(&"-XX:+UseConcMarkSweepGC".to_string()));
        assert!(jvm_args.contains(&"-Duser.home=/home/player".to_string()));

        // Check classpath entries are all present
        let classpath = result.get_classpath_entries();
        assert_eq!(classpath.len(), 2);
        assert!(classpath.contains(&PathBuf::from("/lib1.jar")));
        assert!(classpath.contains(&PathBuf::from("/lib2.jar")));

        // Check program arguments are appended in order
        let program_args = result.get_program_args();
        assert_eq!(program_args.len(), 4);
        assert_eq!(program_args[0], "--server");
        assert_eq!(program_args[1], "localhost");
        assert_eq!(program_args[2], "--user");
        assert_eq!(program_args[3], "player1");
    }

    #[test]
    /// Tests merging from lists of arguments
    fn test_merge_from_lists() {
        let jvm_args_list = vec![
            vec!["-Xmx1G".to_string(), "-XX:+UseParallelGC".to_string()],
            vec!["-Xmx2G".to_string(), "-XX:+UseG1GC".to_string()],
            vec!["-Xms1G".to_string()],
        ];

        let classpath_list = vec![
            vec![PathBuf::from("/old/lib.jar")],
            vec![PathBuf::from("/new/lib.jar")],
        ];

        let program_args_list = vec![
            vec!["--mode".to_string(), "creative".to_string()],
            vec!["--server".to_string(), "play.example.com".to_string()],
        ];

        let result =
            JavaArgumentsMerger::merge_from_lists(jvm_args_list, classpath_list, program_args_list);

        // Check JVM arguments are properly merged with overrides
        let jvm_args = result.build_jvm_args();
        assert!(jvm_args.contains(&"-Xmx2G".to_string())); // Latest -Xmx value
        assert!(!jvm_args.contains(&"-Xmx1G".to_string())); // Overridden
        assert!(jvm_args.contains(&"-Xms1G".to_string()));
        assert!(jvm_args.contains(&"-XX:+UseG1GC".to_string())); // Latest -XX value
        assert!(!jvm_args.contains(&"-XX:+UseParallelGC".to_string())); // Overridden

        // Check classpath entries are all present
        let classpath = result.get_classpath_entries();
        assert_eq!(classpath.len(), 2);
        assert!(classpath.contains(&PathBuf::from("/old/lib.jar")));
        assert!(classpath.contains(&PathBuf::from("/new/lib.jar")));

        // Check program arguments are appended in order
        let program_args = result.get_program_args();
        assert_eq!(program_args.len(), 4);
        assert_eq!(program_args[0], "--mode");
        assert_eq!(program_args[1], "creative");
        assert_eq!(program_args[2], "--server");
        assert_eq!(program_args[3], "play.example.com");
    }
}
