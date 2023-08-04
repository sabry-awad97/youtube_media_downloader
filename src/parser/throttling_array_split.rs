use super::find_object_from_startpoint;
use crate::{AppResult, YoutubeError};
use regex::Regex;

pub fn throttling_array_split(js_array: &str) -> AppResult<Vec<String>> {
    let mut results = Vec::new();
    let mut curr_substring = &js_array[1..js_array.len() - 1];

    let comma_regex = Regex::new(r",").map_err(|_| YoutubeError::RegexMatchError {
        caller: "throttling_array_split".to_owned(),
        pattern: r",".to_owned(),
    })?;

    let func_regex =
        Regex::new(r"function\([^)]*\)").map_err(|_| YoutubeError::RegexMatchError {
            caller: "throttling_array_split".to_owned(),
            pattern: r"function\([^)]*\)".to_owned(),
        })?;

    while !curr_substring.is_empty() {
        if curr_substring.starts_with("function") {
            // Handle functions separately. These can contain commas
            let match_result = func_regex.find(curr_substring);
            if let Some(match_result) = match_result {
                let match_end = match_result.end();
                let function_text = find_object_from_startpoint(curr_substring, match_end)?;
                let full_function_def = &curr_substring[..match_end + function_text.len()];
                results.push(full_function_def.to_string());
                curr_substring = &curr_substring[full_function_def.len() + 1..];
            }
        } else {
            let match_result = comma_regex.find(curr_substring);
            match match_result {
                Some(match_result) => {
                    let match_start = match_result.start();
                    let curr_el = &curr_substring[..match_start];
                    results.push(curr_el.to_string());
                    curr_substring = &curr_substring[match_start + 1..];
                }
                None => {
                    let curr_el = curr_substring;
                    results.push(curr_el.to_string());
                    curr_substring = "";
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throttling_array_split() {
        let js_array = r#"[1,2,3]"#;
        let expected_result = vec!["1", "2", "3"];
        assert_eq!(throttling_array_split(js_array).unwrap(), expected_result);
    }
}
