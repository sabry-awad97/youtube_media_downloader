use crate::{AppResult, YoutubeError};

fn find_js_array_from_startpoint(html: &str, start_point: usize) -> AppResult<&str> {
    // Find the opening bracket '[' after the start point
    let opening_bracket_index =
        html[start_point..]
            .find('[')
            .ok_or(YoutubeError::HTMLParseError {
                error_string: "No opening bracket found".to_string(),
            })?;

    // Find the closing bracket ']' after the opening bracket
    let mut bracket_count = 1;
    let mut current_index = start_point + opening_bracket_index + 1;

    while bracket_count > 0 && current_index < html.len() {
        match html.chars().nth(current_index) {
            Some('[') => bracket_count += 1,
            Some(']') => bracket_count -= 1,
            _ => {}
        }

        current_index += 1;
    }

    // If there is a corresponding closing bracket, return the substring between the brackets
    if bracket_count == 0 {
        Ok(&html[start_point + opening_bracket_index..current_index])
    } else {
        Err(YoutubeError::HTMLParseError {
            error_string: "No corresponding closing bracket found".to_string(),
        })
    }
}

fn find_js_object_from_startpoint(html: &str, start_point: usize) -> AppResult<&str> {
    // Find the opening brace '{' after the start point
    let opening_brace_index =
        html[start_point..]
            .find('{')
            .ok_or(YoutubeError::HTMLParseError {
                error_string: "No opening brace found".to_string(),
            })?;

    // Find the closing brace '}' after the opening brace
    let mut brace_count = 1;
    let mut current_index = start_point + opening_brace_index + 1;

    while brace_count > 0 && current_index < html.len() {
        match html.chars().nth(current_index) {
            Some('{') => brace_count += 1,
            Some('}') => brace_count -= 1,
            _ => {}
        }

        current_index += 1;
    }

    // If there is a corresponding closing brace, return the substring between the braces
    if brace_count == 0 {
        Ok(&html[start_point + opening_brace_index..current_index])
    } else {
        Err(YoutubeError::HTMLParseError {
            error_string: "No corresponding closing brace found".to_string(),
        })
    }
}

pub fn find_js_value_from_startpoint(html: &str, start_point: usize) -> AppResult<&str> {
    // Find the opening brace '{' or bracket '[' after the start point
    let opening_brace_index = html[start_point..].find('{');
    let opening_bracket_index = html[start_point..].find('[');

    match (opening_brace_index, opening_bracket_index) {
        (Some(brace_index), Some(bracket_index)) => {
            if brace_index < bracket_index {
                // Opening brace '{' is found first, it's an object
                find_js_object_from_startpoint(html, start_point)
            } else {
                // Opening bracket '[' is found first, it's an array
                find_js_array_from_startpoint(html, start_point)
            }
        }
        (Some(_), None) => {
            // Only opening brace '{' is found, it's an object
            find_js_object_from_startpoint(html, start_point)
        }
        (None, Some(_)) => {
            // Only opening bracket '[' is found, it's an array
            find_js_array_from_startpoint(html, start_point)
        }
        _ => Err(YoutubeError::HTMLParseError {
            error_string: "No opening brace or bracket found".to_string(),
        }), // Neither opening brace nor bracket is found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_js_array_from_startpoint() {
        let html = "<div id=\"my-div\">[1, 2, 3]</div>";
        let start_point = html.find("id=\"my-div\"").unwrap() + 10;
        let expected = "[1, 2, 3]";
        let result = find_js_array_from_startpoint(html, start_point);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_find_js_object_from_startpoint() {
        let html = "<div id=\"my-div\">{\"name\": \"John\", \"age\": 30}</div>";
        let start_point = html.find("id=\"my-div\"").unwrap() + 10;
        let expected = "{\"name\": \"John\", \"age\": 30}";
        let result = find_js_object_from_startpoint(html, start_point);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_find_js_value_from_startpoint() {
        let html = "<div id=\"my-div\">[1, 2, 3]</div><div id=\"my-div2\">{\"name\": \"John\", \"age\": 30}</div>";
        let start_point = html.find("id=\"my-div\"").unwrap() + 10;
        let expected = "[1, 2, 3]";
        let result = find_js_value_from_startpoint(html, start_point);
        assert_eq!(result, Ok(expected));

        let start_point = html.find("id=\"my-div2\"").unwrap() + 10;
        let expected = "{\"name\": \"John\", \"age\": 30}";
        let result = find_js_value_from_startpoint(html, start_point);
        assert_eq!(result, Ok(expected));
    }
}
