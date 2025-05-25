pub mod string_utils {
    use regex::Regex;
    use std::collections::HashMap;

    /// Parses command line arguments in the format "--key value" or "--key ${value}".
    /// Returns a HashMap where the keys are the argument names without the "--" prefix
    /// and the values are the corresponding argument values.
    pub fn parse_arguments(args: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();

        // Process the arguments without using lookahead in regex
        // Split the arguments string into parts by "--" prefix
        let parts: Vec<&str> = args.split("--").filter(|s| !s.trim().is_empty()).collect();

        for part in parts {
            let mut part_iter = part.trim().splitn(2, ' ');
            if let Some(key) = part_iter.next() {
                if let Some(value) = part_iter.next() {
                    result.insert(key.to_string(), value.trim().to_string());
                }
            }
        }

        // If we have no matches yet, try an alternative approach
        if result.is_empty() {
            // Split by "--" first, then by whitespace
            let arg_chunks: Vec<&str> = args.split("--").filter(|s| !s.trim().is_empty()).collect();

            for chunk in arg_chunks {
                let parts: Vec<&str> = chunk.trim().splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    result.insert(key, value);
                }
            }
        }

        result
    }

    /// Checks if a string contains any variable placeholders in the format ${variable_name}.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to check for placeholders
    ///
    /// # Returns
    ///
    /// * `bool` - True if the input contains any placeholders, false otherwise
    pub fn has_placeholders(input: &str) -> bool {
        let re = Regex::new(r"\$\{[^}]+\}").unwrap();
        re.is_match(input)
    }

    /// Extracts all variable placeholders in the format ${variable_name} from a string.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to extract placeholders from
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - A vector of extracted placeholder names (without the ${} delimiters)
    pub fn extract_placeholders(input: &str) -> Vec<String> {
        let re = Regex::new(r"\$\{([^}]+)\}").unwrap();
        re.captures_iter(input)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::string_utils::parse_arguments;

    #[test]
    /// Tests parsing of multi-argument command line strings into key-value pairs
    fn test_minecraft_arguments_parsing() {
        // The original arguments from Minecraft launch parameters
        let args = "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root} --assetIndex ${assets_index_name} --uuid ${auth_uuid} --accessToken ${auth_access_token} --clientId ${clientid} --xuid ${auth_xuid} --userType ${user_type} --versionType ${version_type}";

        let parsed = parse_arguments(args);

        // Verify all key-value pairs were correctly extracted
        assert_eq!(
            parsed.get("username"),
            Some(&"${auth_player_name}".to_string())
        );
        assert_eq!(parsed.get("version"), Some(&"${version_name}".to_string()));
        assert_eq!(
            parsed.get("gameDir"),
            Some(&"${game_directory}".to_string())
        );
        assert_eq!(parsed.get("assetsDir"), Some(&"${assets_root}".to_string()));
        assert_eq!(
            parsed.get("assetIndex"),
            Some(&"${assets_index_name}".to_string())
        );
        assert_eq!(parsed.get("uuid"), Some(&"${auth_uuid}".to_string()));
        assert_eq!(
            parsed.get("accessToken"),
            Some(&"${auth_access_token}".to_string())
        );
        assert_eq!(parsed.get("clientId"), Some(&"${clientid}".to_string()));
        assert_eq!(parsed.get("xuid"), Some(&"${auth_xuid}".to_string()));
        assert_eq!(parsed.get("userType"), Some(&"${user_type}".to_string()));
        assert_eq!(
            parsed.get("versionType"),
            Some(&"${version_type}".to_string())
        );

        // Verify the number of extracted arguments matches the expected count
        assert_eq!(parsed.len(), 11);
    }

    #[test]
    /// Tests parsing arguments with a mix of placeholders and literal values
    fn test_mixed_arguments_parsing() {
        let args = "--username player1 --version ${version_name} --gameDir C:\\Games\\Minecraft --assetsDir ${assets_root}";

        let parsed = parse_arguments(args);

        assert_eq!(parsed.get("username"), Some(&"player1".to_string()));
        assert_eq!(parsed.get("version"), Some(&"${version_name}".to_string()));
        assert_eq!(
            parsed.get("gameDir"),
            Some(&"C:\\Games\\Minecraft".to_string())
        );
        assert_eq!(parsed.get("assetsDir"), Some(&"${assets_root}".to_string()));

        assert_eq!(parsed.len(), 4);
    }

    #[test]
    /// Tests parsing arguments from a list of strings into key-value pairs
    fn test_arguments_from_string_list() {
        // Arguments as they might appear in a list/array
        let arg_list = vec![
            "--username",
            "${auth_player_name}",
            "--version",
            "${version_name}",
            "--gameDir",
            "${game_directory}",
            "--assetsDir",
            "${assets_root}",
            "--assetIndex",
            "${assets_index_name}",
            "--uuid",
            "${auth_uuid}",
            "--accessToken",
            "${auth_access_token}",
            "--clientId",
            "${clientid}",
            "--xuid",
            "${auth_xuid}",
            "--userType",
            "${user_type}",
            "--versionType",
            "${version_type}",
        ];

        // Join the arguments into a single line with spaces
        let args_single_line = arg_list.join(" ");

        // Parse the arguments
        let parsed = parse_arguments(&args_single_line);

        // Verify all key-value pairs were correctly extracted
        assert_eq!(
            parsed.get("username"),
            Some(&"${auth_player_name}".to_string())
        );
        assert_eq!(parsed.get("version"), Some(&"${version_name}".to_string()));
        assert_eq!(
            parsed.get("gameDir"),
            Some(&"${game_directory}".to_string())
        );
        assert_eq!(parsed.get("assetsDir"), Some(&"${assets_root}".to_string()));
        assert_eq!(
            parsed.get("assetIndex"),
            Some(&"${assets_index_name}".to_string())
        );
        assert_eq!(parsed.get("uuid"), Some(&"${auth_uuid}".to_string()));
        assert_eq!(
            parsed.get("accessToken"),
            Some(&"${auth_access_token}".to_string())
        );
        assert_eq!(parsed.get("clientId"), Some(&"${clientid}".to_string()));
        assert_eq!(parsed.get("xuid"), Some(&"${auth_xuid}".to_string()));
        assert_eq!(parsed.get("userType"), Some(&"${user_type}".to_string()));
        assert_eq!(
            parsed.get("versionType"),
            Some(&"${version_type}".to_string())
        );

        // Verify the number of extracted arguments matches the expected count
        assert_eq!(parsed.len(), 11);
    }
}
