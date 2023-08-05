use crate::{regex_search, AppResult, YoutubeError};

pub fn get_ytplayer_js(html: &str) -> AppResult<String> {
    let js_url_patterns = vec![r"(/s/player/[\w\d]+/[\w\d_/.]+/base\.js)"];

    for pattern in js_url_patterns {
        if let Ok(result) = regex_search(pattern, html, 1) {
            return Ok(result);
        }
    }

    Err(YoutubeError::RegexMatchError {
        caller: "get_ytplayer_js".to_string(),
        pattern: "js_url_patterns".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ytplayer_js_valid() {
        let html = r#"
            <html>
                <head>
                    <script src="/s/player/youtube/abc123/base.js"></script>
                </head>
                <body>
                    <script src="/s/player/youtube/xyz456/script.js"></script>
                </body>
            </html>
        "#;

        let expected_js_path = "/s/player/youtube/abc123/base.js";
        let result = get_ytplayer_js(html);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_js_path);
    }

    #[test]
    fn test_get_ytplayer_js_invalid() {
        let html = r#"
            <html>
                <head>
                    <script src="/s/player/youtube/abc123/script.js"></script>
                </head>
                <body>
                    <script src="/s/player/youtube/xyz456/script.js"></script>
                </body>
            </html>
        "#;

        let result = get_ytplayer_js(html);
        assert!(result.is_err());
    }
}
