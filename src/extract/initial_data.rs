use crate::{parser::parse_for_object, AppResult, YoutubeError};

pub fn initial_data(watch_html: &str) -> AppResult<serde_json::Value> {
    // List of patterns to search for the ytInitialData JSON
    let patterns = vec![
        r#"window\[['"]ytInitialData['"]]\s*=\s*"#,
        r"ytInitialData\s*=\s*",
    ];

    // Iterate over the patterns and attempt to extract the JSON
    for pattern in patterns {
        if let Ok(data) = parse_for_object(watch_html, pattern) {
            return Ok(data);
        }
    }

    // If none of the patterns matched, raise a RegexMatchError
    Err(YoutubeError::RegexMatchError {
        caller: "initial_data".to_string(),
        pattern: "initial_data_pattern".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_initial_data_with_window_yt_initial_data() {
        let watch_html = r#"
            <script>
                window['ytInitialData'] = { "key1": "value1", "key2": "value2" };
                var somethingElse = "xyz";
            </script>
        "#;

        let result = initial_data(watch_html);

        // Expected ytInitialData value
        let expected_data = json!({
            "key1": "value1",
            "key2": "value2"
        });

        assert_eq!(result, Ok(expected_data));
    }

    #[test]
    fn test_initial_data_with_yt_initial_data() {
        let watch_html = r#"
            <script>
                ytInitialData = { "key1": "value1", "key2": "value2" };
                var somethingElse = "xyz";
            </script>
        "#;

        let result = initial_data(watch_html);

        // Expected ytInitialData value
        let expected_data = json!({
            "key1": "value1",
            "key2": "value2"
        });

        assert_eq!(result, Ok(expected_data));
    }

    #[test]
    fn test_initial_data_no_yt_initial_data() {
        let watch_html = r#"
            <script>
                var somethingElse = "xyz";
            </script>
        "#;

        // In this case, the function should return an error
        let result = initial_data(watch_html);

        // Expected error type
        let expected_error = YoutubeError::RegexMatchError {
            caller: "initial_data".to_string(),
            pattern: "initial_data_pattern".to_string(),
        };

        assert_eq!(result, Err(expected_error));
    }
}
