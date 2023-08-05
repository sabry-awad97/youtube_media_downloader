use fancy_regex::Regex;

use crate::{AppResult, YoutubeError};

pub fn channel_name(url: &str) -> AppResult<String> {
    let patterns = [
        r"(?:(c)\/([%\d\w_\-]+)(.*)?)",
        r"(?:(channel)\/([%\w\d_\-]+)(\/.*)?)",
        r"(?:(u)\/([%\d\w_\-]+)(\/.*)?)",
        r"(?:(user)\/([%\w\d_\-]+)(\/.*)?)",
    ];

    for pattern in &patterns {
        let regex = Regex::new(pattern).map_err(|_| YoutubeError::RegexMatchError {
            caller: "channel_name".to_owned(),
            pattern: pattern.to_string(),
        })?;

        if let Ok(Some(captures)) = regex.captures(url) {
            let uri_style = captures.get(1).unwrap().as_str();
            let uri_identifier = captures.get(2).unwrap().as_str();
            return Ok(format!("/{}/{}", uri_style, uri_identifier));
        }
    }

    Err(YoutubeError::RegexMatchError {
        caller: "channel_name".to_owned(),
        pattern: "patterns".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_name_from_c_url() {
        let url = "https://youtube.com/c/MyChannelName/videos";
        assert_eq!(channel_name(url), Ok("/c/MyChannelName".to_string()));
    }

    #[test]
    fn test_channel_name_from_channel_url() {
        let url = "https://youtube.com/channel/UCxyz12345/videos";
        assert_eq!(channel_name(url), Ok("/channel/UCxyz12345".to_string()));
    }

    #[test]
    fn test_channel_name_from_u_url() {
        let url = "https://youtube.com/u/AnotherChannelName/videos";
        assert_eq!(channel_name(url), Ok("/u/AnotherChannelName".to_string()));
    }

    #[test]
    fn test_channel_name_from_user_url() {
        let url = "https://youtube.com/user/UserName123/videos";
        assert_eq!(channel_name(url), Ok("/user/UserName123".to_string()));
    }

    #[test]
    fn test_channel_name_invalid_url() {
        // An invalid URL that doesn't match any pattern
        let url = "https://youtube.com/random";
        assert!(channel_name(url).is_err());
    }

    #[test]
    fn test_channel_name_with_query_parameters() {
        // Channel name should be extracted even if there are other query parameters
        let url = "https://youtube.com/c/MyChannelName/videos?feature=share";
        assert_eq!(channel_name(url), Ok("/c/MyChannelName".to_string()));
    }
}
