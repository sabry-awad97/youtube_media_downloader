use crate::{AppResult, YoutubeError};
use regex::Regex;

pub fn regex_search(pattern: &str, string: &str, group: usize) -> AppResult<String> {
    let regex = Regex::new(pattern).map_err(|_| YoutubeError::RegexMatchError {
        caller: "regex_search".to_string(),
        pattern: pattern.to_string(),
    })?;

    if let Some(captures) = regex.captures(string) {
        if let Some(matched_group) = captures.get(group) {
            return Ok(matched_group.as_str().to_string());
        }
    }

    Err(YoutubeError::RegexMatchError {
        caller: "regex_search".to_string(),
        pattern: pattern.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_search_success() {
        let pattern = r"\b\d{3}-\d{2}-\d{4}\b";
        let string = "123-45-6789";
        let expected_match = "123-45-6789".to_string();

        let result = regex_search(pattern, string, 0);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_match);
    }

    #[test]
    fn test_regex_search_failure() {
        let pattern = r"\b\d{3}-\d{2}-\d{4}\b";
        let string = "123-45-67890";

        let result = regex_search(pattern, string, 0);

        assert!(result.is_err());
    }

    #[test]
    fn test_regex_search_group() {
        let pattern = r"(\d{3})-(\d{2})-(\d{4})";
        let string = "123-45-6789";
        let expected_group = "123".to_string();

        let result = regex_search(pattern, string, 1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_group);
    }
}
