use super::find_object_from_startpoint;
use crate::{AppResult, YoutubeError};

pub fn parse_for_object_from_startpoint(
    html: &str,
    start_point: usize,
) -> AppResult<serde_json::Value> {
    let full_obj = find_object_from_startpoint(html, start_point)?;
    match serde_json::from_str(&full_obj) {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(YoutubeError::HTMLParseError {
            error_string: format!(
                "Could not parse object: {}\nSerde Error: {}",
                full_obj, error
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_valid_json_object() {
        let html = r#"{ "name": "John", "age": 30 }"#;
        let start_point = 0;
        let expected_result = json!({"name": "John", "age": 30});

        assert_eq!(
            parse_for_object_from_startpoint(html, start_point),
            Ok(expected_result)
        );
    }
}
