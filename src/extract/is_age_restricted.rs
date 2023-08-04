use crate::regex_search;

pub fn is_age_restricted(watch_html: &str) -> bool {
    regex_search(r"og:restrictions:age", watch_html, 0).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_age_restricted_true() {
        let watch_html =
            "<html><head><meta property=\"og:restrictions:age\" content=\"true\" /></head></html>";
        assert!(is_age_restricted(watch_html));
    }

    #[test]
    fn test_is_age_restricted_false() {
        let watch_html = "<html><head></head></html>";
        assert!(!is_age_restricted(watch_html));
    }
}
