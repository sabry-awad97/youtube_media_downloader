use crate::{AppResult, YoutubeError};

pub fn find_object_from_startpoint(html: &str, start_point: usize) -> AppResult<String> {
    let html = &html[start_point..];
    if !html.starts_with('{') && !html.starts_with('[') {
        return Err(YoutubeError::HTMLParseError {
            error_string: format!("Invalid start point. Start of HTML:\n{}", &html[..20]),
        });
    }

    let mut last_char: char = '{';
    let mut curr_char: char = ' ';
    let mut stack: Vec<char> = vec![html.chars().next().unwrap()];
    let mut i: usize = 1;

    let context_closers: std::collections::HashMap<char, char> = [
        ('{', '}'),
        ('[', ']'),
        ('"', '"'),
        ('/', '/'), // javascript regex
    ]
    .iter()
    .cloned()
    .collect();

    while i < html.len() {
        if stack.is_empty() {
            break;
        }
        if !curr_char.is_whitespace() {
            last_char = curr_char;
        }
        curr_char = html.chars().nth(i).unwrap();
        let curr_context = *stack.last().unwrap();

        // If we've reached a context closer, we can remove an element off the stack
        if curr_char == context_closers[&curr_context] {
            stack.pop();
            i += 1;
            continue;
        }

        // Strings and regex expressions require special context handling because they can contain
        // context openers *and* closers
        if curr_context == '"' || curr_context == '/' {
            // If there's a backslash in a string or regex expression, we skip a character
            if curr_char == '\\' {
                i += 2;
                continue;
            }
        } else {
            // Non-string contexts are when we need to look for context openers.
            if context_closers.contains_key(&curr_char) {
                // Slash starts a regular expression depending on context
                if !(curr_char == '/'
                    || ['(', ',', '=', ':', '[', '!', '&', '|', '?', '{', '}', ';']
                        .contains(&last_char))
                {
                    stack.push(curr_char);
                }
            }
        }

        i += 1;
    }

    let full_obj = &html[..i];
    Ok(full_obj.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_object() {
        let html = r#"{ "name": "John", "age": 30 }"#;
        let start_point = 0;
        assert_eq!(
            find_object_from_startpoint(html, start_point),
            Ok(r#"{ "name": "John", "age": 30 }"#.to_string())
        );
    }

    #[test]
    fn test_valid_nested_object() {
        let html = r#"{ "person": { "name": "John", "age": 30 } }"#;
        let start_point = 0;
        assert_eq!(
            find_object_from_startpoint(html, start_point),
            Ok(r#"{ "person": { "name": "John", "age": 30 } }"#.to_string())
        );
    }

    #[test]
    fn test_valid_object_with_string() {
        let html = r#"{ "name": "John \"Doe\"", "age": 30 }"#;
        let start_point = 0;
        assert_eq!(
            find_object_from_startpoint(html, start_point),
            Ok(r#"{ "name": "John \"Doe\"", "age": 30 }"#.to_string())
        );
    }

    #[test]
    fn test_valid_object_with_empty_object() {
        let html = r#"{}"#;
        let start_point = 0;
        assert_eq!(
            find_object_from_startpoint(html, start_point),
            Ok("{}".to_string())
        );
    }

    #[test]
    fn test_valid_object_with_nested_array() {
        let html = r#"{ "name": "John", "hobbies": ["reading", "gaming", "coding"] }"#;
        let start_point = 0;
        let result = find_object_from_startpoint(html, start_point);
        assert_eq!(
            result,
            Ok(r#"{ "name": "John", "hobbies": ["reading", "gaming", "coding"] }"#.to_string())
        );
    }
}
