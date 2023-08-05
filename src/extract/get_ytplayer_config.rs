use crate::{parser::parse_for_object, AppResult, YoutubeError};

pub fn get_ytplayer_config(html: &str) -> AppResult<serde_json::Value> {
    // Define the config_patterns
    let config_patterns = vec![
        r"ytplayer\.config\s*=\s*",
        r"ytInitialPlayerResponse\s*=\s*",
    ];

    // Try each config_pattern consecutively
    for regex_pattern in config_patterns {
        if let Ok(config_data) = parse_for_object(html, regex_pattern) {
            return Ok(config_data);
        }
    }

    // Define the setconfig_patterns
    let setconfig_patterns = vec![r#"yt\.setConfig\(.*\s*?['"]PLAYER_CONFIG['"]:\s*"#];

    // Try each setconfig_pattern consecutively
    for regex_pattern in setconfig_patterns {
        if let Ok(setconfig_data) = parse_for_object(html, regex_pattern) {
            return Ok(setconfig_data);
        }
    }

    Err(YoutubeError::RegexMatchError {
        caller: "get_ytplayer_config".to_string(),
        pattern: "config_patterns, setconfig_patterns".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_ytplayer_config_ytplayer_config() {
        let html = r#"
            ytplayer.config = {
                "key1": "value1", 
                "key2": "value2"
            }
        "#;

        let expected_config_data = json!({
            "key1": "value1",
            "key2": "value2"
        });

        let result = get_ytplayer_config(html);
        assert_eq!(result, Ok(expected_config_data));
    }

    #[test]
    fn test_get_ytplayer_config_ytinitialplayerresponse() {
        let html = r#"
            var ytInitialPlayerResponse = {
                "key3": "value3",
                "key4": "value4"
            };
        "#;

        let expected_config_data = json!({
            "key3": "value3",
            "key4": "value4"
        });

        let result = get_ytplayer_config(html).unwrap();
        assert_eq!(result, expected_config_data);
    }

    #[test]
    fn test_get_ytplayer_config_setconfig() {
        let html = r#"
            var yt = {
                setConfig: function(config) {
                    this.PLAYER_CONFIG = config;
                }
            };

            yt.setConfig({"PLAYER_CONFIG": {"key5": "value5", "key6": "value6"} });
        "#;

        let expected_config_data = json!({
            "key5": "value5",
            "key6": "value6"
        });

        let result = get_ytplayer_config(html).unwrap();
        assert_eq!(result, expected_config_data);
    }

    #[test]
    fn test_get_ytplayer_config_failure() {
        let html = r#"
            var some_other_var = {
                "key1": "value1",
                "key2": "value2"
            };
        "#;

        let result = get_ytplayer_config(html);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            YoutubeError::RegexMatchError {
                caller: "get_ytplayer_config".to_string(),
                pattern: "config_patterns, setconfig_patterns".to_string(),
            }
        );
    }
}
