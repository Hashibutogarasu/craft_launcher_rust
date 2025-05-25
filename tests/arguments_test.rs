#[cfg(test)]
mod test {
    use craft_launcher_rust::craft_launcher::java::arguments_builder::arguments_builder::java_args;
    use craft_launcher_rust::craft_launcher::utils::string_utils::string_utils::{
        extract_placeholders, has_placeholders, parse_arguments,
    };
    use std::collections::HashMap;

    #[test]
    /// Tests replacing placeholders in Java arguments with actual values
    fn test_placeholder_replacement_in_arguments() {
        // Define a set of arguments with placeholders
        let original_args = "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root}";

        // Parse the arguments
        let parsed_args = parse_arguments(original_args);

        // Define the values to replace the placeholders with
        let mut placeholder_values = HashMap::new();
        placeholder_values.insert("auth_player_name".to_string(), "Player123".to_string());
        placeholder_values.insert("version_name".to_string(), "1.21.5".to_string());
        placeholder_values.insert(
            "game_directory".to_string(),
            "C:\\Games\\Minecraft".to_string(),
        );
        placeholder_values.insert(
            "assets_root".to_string(),
            "C:\\Games\\Minecraft\\assets".to_string(),
        );

        // Create a new map with replaced values
        let mut replaced_args = HashMap::new();
        for (key, value) in parsed_args.iter() {
            if has_placeholders(value) {
                let placeholders = extract_placeholders(value);
                let mut new_value = value.clone();

                for placeholder in placeholders {
                    if let Some(replacement) = placeholder_values.get(&placeholder) {
                        new_value =
                            new_value.replace(&format!("${{{}}}", placeholder), replacement);
                    }
                }

                replaced_args.insert(key.clone(), new_value);
            } else {
                replaced_args.insert(key.clone(), value.clone());
            }
        }

        // Verify that placeholders were replaced correctly
        assert_eq!(
            replaced_args.get("username"),
            Some(&"Player123".to_string())
        );
        assert_eq!(replaced_args.get("version"), Some(&"1.21.5".to_string()));
        assert_eq!(
            replaced_args.get("gameDir"),
            Some(&"C:\\Games\\Minecraft".to_string())
        );
        assert_eq!(
            replaced_args.get("assetsDir"),
            Some(&"C:\\Games\\Minecraft\\assets".to_string())
        );
    }

    #[test]
    /// Tests integration of placeholder replacement with Java arguments builder
    fn test_java_args_with_placeholders() {
        // Define a set of program arguments with placeholders
        let minecraft_args =
            "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory}";
        let parsed_args = parse_arguments(minecraft_args);

        // Define the values to replace the placeholders with
        let mut placeholder_values = HashMap::new();
        placeholder_values.insert("auth_player_name".to_string(), "Steve".to_string());
        placeholder_values.insert("version_name".to_string(), "1.21".to_string());
        placeholder_values.insert("game_directory".to_string(), "C:\\Minecraft".to_string());

        // Build program arguments with replaced values
        let mut program_args = Vec::new();
        for (key, value) in parsed_args.iter() {
            let mut arg_value = value.clone();

            if has_placeholders(&arg_value) {
                let placeholders = extract_placeholders(&arg_value);

                for placeholder in placeholders {
                    if let Some(replacement) = placeholder_values.get(&placeholder) {
                        arg_value =
                            arg_value.replace(&format!("${{{}}}", placeholder), replacement);
                    }
                }
            }

            program_args.push(format!("--{}", key));
            program_args.push(arg_value);
        }

        // Build Java command with replaced arguments
        let command = java_args()
            .with_executable("javaw")
            .add_jvm_arg("-Xmx2G")
            .with_main_class("net.minecraft.client.main.Main")
            .add_program_args(program_args)
            .build();

        // Verify the command structure is correct
        assert_eq!(command[0], "javaw");
        assert_eq!(command[1], "-Xmx2G");
        assert_eq!(command[2], "net.minecraft.client.main.Main");

        // Verify the replaced arguments are present in the command
        let command_str = command.join(" ");
        assert!(command_str.contains("--username Steve"));
        assert!(command_str.contains("--version 1.21"));
        assert!(command_str.contains("--gameDir C:\\Minecraft"));
    }

    #[test]
    /// Tests complex placeholder replacement with nested values and mixed content
    fn test_complex_placeholder_replacement() {
        // Create arguments with complex placeholders
        let complex_args =
            "--jvm ${jvm_args} --mainClass ${main_class} --gameDir ${user.home}/games/${game.id}";
        let parsed_args = parse_arguments(complex_args);

        // Define nested placeholder values
        let mut placeholder_values = HashMap::new();
        placeholder_values.insert(
            "jvm_args".to_string(),
            "-Xmx${memory} -Xms${min_memory}".to_string(),
        );
        placeholder_values.insert("memory".to_string(), "4G".to_string());
        placeholder_values.insert("min_memory".to_string(), "2G".to_string());
        placeholder_values.insert(
            "main_class".to_string(),
            "net.minecraft.client.main.Main".to_string(),
        );
        placeholder_values.insert("user.home".to_string(), "C:\\Users\\Player".to_string());
        placeholder_values.insert("game.id".to_string(), "minecraft".to_string());

        // Function to recursively replace placeholders
        fn replace_placeholders_recursive(text: &str, values: &HashMap<String, String>) -> String {
            let mut result = text.to_string();
            let mut previous_result;

            // Keep replacing until no more changes (handles nested placeholders)
            loop {
                previous_result = result.clone();

                if has_placeholders(&result) {
                    let placeholders = extract_placeholders(&result);

                    for placeholder in placeholders {
                        if let Some(replacement) = values.get(&placeholder) {
                            result = result.replace(&format!("${{{}}}", placeholder), replacement);
                        }
                    }
                }

                // Stop when no more replacements are made
                if result == previous_result {
                    break;
                }
            }

            result
        }

        // Replace placeholders in each argument
        let mut replaced_args = HashMap::new();
        for (key, value) in parsed_args {
            let new_value = replace_placeholders_recursive(&value, &placeholder_values);
            replaced_args.insert(key, new_value);
        }

        // Verify the replacements
        assert_eq!(replaced_args.get("jvm"), Some(&"-Xmx4G -Xms2G".to_string()));
        assert_eq!(
            replaced_args.get("mainClass"),
            Some(&"net.minecraft.client.main.Main".to_string())
        );
        assert_eq!(
            replaced_args.get("gameDir"),
            Some(&"C:\\Users\\Player/games/minecraft".to_string())
        );
    }
}
