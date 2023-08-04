pub fn is_private(watch_html: &str) -> bool {
    let private_strings = [
        "This is a private video. Please sign in to verify that you may see it.",
        "\"simpleText\":\"Private video\"",
        "This video is private.",
    ];

    for string in &private_strings {
        if watch_html.contains(string) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private_with_private_video() {
        let watch_html = "This is a private video. Please sign in to verify that you may see it.";
        assert!(is_private(watch_html));
    }

    #[test]
    fn test_is_private_with_simple_text() {
        let watch_html = "\"simpleText\":\"Private video\"";
        assert!(is_private(watch_html));
    }

    #[test]
    fn test_is_private_with_video_is_private() {
        let watch_html = "This video is private.";
        assert!(is_private(watch_html));
    }

    #[test]
    fn test_is_private_with_public_video() {
        let watch_html = "This is a public video.";
        assert!(!is_private(watch_html));
    }
}
