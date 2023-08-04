use super::parse_for_object_from_startpoint;
use crate::{AppResult, YoutubeError};
use regex::Regex;

pub fn parse_for_all_objects(
    html: &str,
    preceding_regex: &str,
) -> AppResult<Vec<serde_json::Value>> {
    let mut result = Vec::new();
    let regex = Regex::new(preceding_regex).map_err(|_| YoutubeError::RegexMatchError {
        caller: "parse_for_all_objects".to_owned(),
        pattern: preceding_regex.to_owned(),
    })?;

    for match_iter in regex.find_iter(html) {
        let start_index = match_iter.end();
        match parse_for_object_from_startpoint(html, start_index) {
            Ok(value) => result.push(value),
            Err(YoutubeError::HTMLParseError { .. }) => continue,
            Err(err) => return Err(err),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_parse_for_all_objects_single_match() {
        let html = r#"
            <script>{"name": "John", "age": 30}</script>
            <div>Hello World!</div>
        "#;
        let preceding_regex = r#"<script>\s*"#;
        let result = parse_for_all_objects(html, preceding_regex).unwrap();
        assert_eq!(result, vec![json!({"name": "John", "age": 30}),]);
    }

    #[test]
    fn test_parse_for_all_objects_multiple_matches() {
        let html = r#"
            <script>{"name": "John", "age": 30}</script>
            <div>Hello World!</div>
            <script>{"city": "New York", "country": "USA"}</script>
            <script>{"color": "red", "size": "medium"}</script>
        "#;
        let preceding_regex = r#"<script>\s*"#;
        let result = parse_for_all_objects(html, preceding_regex).unwrap();
        assert_eq!(
            result,
            vec![
                json!({"name": "John", "age": 30}),
                json!({"city": "New York", "country": "USA"}),
                json!({"color": "red", "size": "medium"})
            ]
        );
    }
}
