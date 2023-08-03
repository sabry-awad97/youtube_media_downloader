pub fn recording_available(watch_html: &str) -> bool {
    let unavailable_strings = ["This live stream recording is not available."];

    for string in &unavailable_strings {
        if watch_html.contains(string) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_available_with_available_recording() {
        let watch_html = "This is a live stream recording.";
        assert!(recording_available(watch_html));
    }

    #[test]
    fn test_recording_available_with_unavailable_recording() {
        let watch_html = "This live stream recording is not available.";
        assert!(!recording_available(watch_html));
    }

    #[test]
    fn test_recording_available_with_multiple_unavailable_strings() {
        let watch_html = "This live stream recording is not available. Please try again later.";
        assert!(!recording_available(watch_html));
    }

    #[test]
    fn test_recording_available_with_empty_string() {
        let watch_html = "";
        assert!(recording_available(watch_html));
    }
}
