use crate::regex_search;

pub fn video_id(url: &str) -> Option<String> {
    let pattern = r#"(?:v=|)([0-9A-Za-z_-]{11}).*"#;
    match regex_search(pattern, url, 1) {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_id_from_watch_url() {
        let url = "https://youtube.com/watch?v=dQw4w9WgXcQ";
        assert_eq!(video_id(url), Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_video_id_from_embed_url() {
        let url = "https://youtube.com/embed/JwYX52BP2Sk";
        assert_eq!(video_id(url), Some("JwYX52BP2Sk".to_string()));
    }

    #[test]
    fn test_video_id_from_short_url() {
        let url = "https://youtu.be/9bZkp7q19f0";
        assert_eq!(video_id(url), Some("9bZkp7q19f0".to_string()));
    }

    #[test]
    fn test_video_id_invalid_url() {
        // An invalid URL that doesn't match any pattern
        let url = "https://youtube.com/random";
        assert_eq!(video_id(url), None);
    }

    #[test]
    fn test_video_id_with_query_parameters() {
        // Video id should be extracted even if there are query parameters
        let url = "https://youtube.com/watch?v=dQw4w9WgXcQ&feature=share";
        assert_eq!(video_id(url), Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_video_id_with_trailing_slash() {
        // Video id should be extracted even with a trailing slash
        let url = "https://youtube.com/watch?v=dQw4w9WgXcQ/";
        assert_eq!(video_id(url), Some("dQw4w9WgXcQ".to_string()));
    }
}
