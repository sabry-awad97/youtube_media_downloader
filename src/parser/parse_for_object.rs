use fancy_regex::Regex;

use super::parse_for_object_from_startpoint;
use crate::{AppResult, YoutubeError};

pub fn parse_for_object(html: &str, preceding_regex: &str) -> AppResult<serde_json::Value> {
    let regex = Regex::new(preceding_regex).map_err(|_| YoutubeError::RegexMatchError {
        caller: "parse_for_object".to_owned(),
        pattern: preceding_regex.to_owned(),
    })?;

    let result = regex
        .find(html)
        .map_err(|_| YoutubeError::HTMLParseError {
            error_string: format!("No matches for regex {}", preceding_regex),
        })?
        .ok_or(YoutubeError::HTMLParseError {
            error_string: format!("No matches for regex {}", preceding_regex),
        })?;

    let start_index = result.end();
    parse_for_object_from_startpoint(html, start_index)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_parse_for_object_simple_object() {
        let html = r#"<script>{"name": "John", "age": 30}</script>"#;
        let preceding_regex = r#"<script>\s*"#;
        let result = parse_for_object(html, preceding_regex);
        assert_eq!(result, Ok(json!({"name": "John", "age": 30})));
    }

    #[test]
    fn test_parse_for_object_nested_object() {
        let html = r#"<script>{"name": "John", "address": {"city": "New York", "country": "USA"}}</script>"#;
        let preceding_regex = r#"<script>\s*"#;
        let result = parse_for_object(html, preceding_regex);
        assert_eq!(
            result,
            Ok(json!({"name": "John", "address": {"city": "New York", "country": "USA"}}))
        );
    }
}
