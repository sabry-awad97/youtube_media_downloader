use std::collections::HashMap;
use url::Url;

pub fn playlist_id(url: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(url) {
        let query_params: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();

        if let Some(playlist_id) = query_params.get("list") {
            return Some(playlist_id.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist_id_from_playlist_url() {
        let url = "https://youtube.com/playlist?list=PL1234567890";
        assert_eq!(playlist_id(url), Some("PL1234567890".to_string()));
    }

    #[test]
    fn test_playlist_id_from_watch_url() {
        let url = "https://youtube.com/watch?v=abcd1234&list=PL9876543210";
        assert_eq!(playlist_id(url), Some("PL9876543210".to_string()));
    }

    #[test]
    fn test_playlist_id_invalid_url() {
        // An invalid URL that doesn't match any pattern
        let url = "https://youtube.com/random";
        assert_eq!(playlist_id(url), None);
    }

    #[test]
    fn test_playlist_id_missing_list_parameter() {
        // URL without the required list parameter
        let url = "https://youtube.com/watch?v=abcd1234";
        assert_eq!(playlist_id(url), None);
    }

    #[test]
    fn test_playlist_id_with_query_parameters() {
        // Playlist id should be extracted even if there are other query parameters
        let url = "https://youtube.com/playlist?list=PL1234567890&feature=share";
        assert_eq!(playlist_id(url), Some("PL1234567890".to_string()));
    }

    #[test]
    fn test_playlist_id_with_encoded_characters() {
        // Playlist id with encoded characters in the URL should be handled correctly
        let url = "https://youtube.com/watch?v=abcd1234&list=PL%2B%2F%3D";
        assert_eq!(playlist_id(url), Some("PL+/=".to_string()));
    }
}
