use crate::{helpers::regex_search, AppResult, YoutubeError};

pub fn get_initial_function_name(js: &str) -> AppResult<String> {
    let function_patterns = [
        r"\b[cs]\s*&&\s*[adf]\.set\([^,]+\s*,\s*encodeURIComponent\s*\(\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r"\b[a-zA-Z0-9]+\s*&&\s*[a-zA-Z0-9]+\.set\([^,]+\s*,\s*encodeURIComponent\s*\(\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r#"(?:\b|[^a-zA-Z0-9$])(?P<sig>[a-zA-Z0-9$]{2})\s*=\s*function\(\s*a\s*\)\s*\{\s*a\s*=\s*a\.split\(\s*""\s*\)"#, // noqa: E501
        r#"(?P<sig>[a-zA-Z0-9$]+)\s*=\s*function\(\s*a\s*\)\s*\{\s*a\s*=\s*a\.split\(\s*""\s*\)"#, // noqa: E501
        r#"(["\'])signature\1\s*,\s*(?P<sig>[a-zA-Z0-9$]+)\("#,
        r"\.sig\|\|(?P<sig>[a-zA-Z0-9$]+)\(",
        r"yt\.akamaized\.net/\)\s*\|\|\s*.*?\s*[cs]\s*&&\s*[adf]\.set\([^,]+\s*,\s*(?:encodeURIComponent\s*\()?\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r"\b[cs]\s*&&\s*[adf]\.set\([^,]+\s*,\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r"\b[a-zA-Z0-9]+\s*&&\s*[a-zA-Z0-9]+\.set\([^,]+\s*,\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r"\bc\s*&&\s*a\.set\([^,]+\s*,\s*\([^)]*\)\s*\(\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r"\bc\s*&&\s*[a-zA-Z0-9]+\.set\([^,]+\s*,\s*\([^)]*\)\s*\(\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
        r"\bc\s*&&\s*[a-zA-Z0-9]+\.set\([^,]+\s*,\s*\([^)]*\)\s*\(\s*(?P<sig>[a-zA-Z0-9$]+)\(", // noqa: E501
    ];

    log::debug!("finding initial function name");
    for pattern in function_patterns.iter() {
        if let Ok(function_match) = regex_search(pattern, js, 1) {
            log::debug!("finished regex search, matched: {}", pattern);
            return Ok(function_match);
        }
    }

    Err(YoutubeError::RegexMatchError {
        caller: "get_initial_function_name".to_owned(),
        pattern: "multiple".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        r#"c && a.set("param1", encodeURIComponent(signature($$sometext123)));"#,
        "signature"
    )]
    #[case(
        r#"var myVar = abc && xyz.set(123, encodeURIComponent(signatureFunction($)));"#,
        "signatureFunction"
    )]
    #[case(
        r#"
            var ab = function(a) {
                a = a.split("");
                return a;
            };
        "#,
        "ab"
    )]
    #[case(
        r#"
            var myFunction = function(a) {
                a = a.split("");
                return a;
            };
        "#,
        "myFunction"
    )]
    fn test_get_initial_function_name(#[case] input: &str, #[case] expected: &str) {
        let result = get_initial_function_name(input);
        assert_eq!(result, Ok(expected.to_string()));
    }
}
