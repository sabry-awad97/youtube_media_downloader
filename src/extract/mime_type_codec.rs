use regex::Regex;

use crate::{AppResult, YoutubeError};

pub fn mime_type_codec(mime_type_codec: &str) -> AppResult<(&str, Vec<&str>)> {
    let pattern = r#"(\w+/\w+);\scodecs="([a-zA-Z-0-9.,\s]*)""#;
    let regex = Regex::new(pattern).map_err(|_| YoutubeError::RegexMatchError {
        caller: "mime_type_codec".to_string(),
        pattern: pattern.to_string(),
    })?;

    if let Some(captures) = regex.captures(mime_type_codec) {
        let mime_type = captures.get(1).unwrap().as_str();
        let codecs = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(str::trim)
            .collect();
        return Ok((mime_type, codecs));
    };

    Err(YoutubeError::RegexMatchError {
        caller: "mime_type_codec".to_string(),
        pattern: pattern.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_type_codec_valid() {
        let test_cases = vec![
            (
                "audio/webm; codecs=\"opus, vorbis\"",
                ("audio/webm", vec!["opus", "vorbis"]),
            ),
            (
                "video/mp4; codecs=\"avc1.42E01E, mp4a.40.2\"",
                ("video/mp4", vec!["avc1.42E01E", "mp4a.40.2"]),
            ),
            ("audio/mpeg; codecs=\"mp3\"", ("audio/mpeg", vec!["mp3"])),
            ("text/plain; codecs=\"\"", ("text/plain", vec![""])),
        ];

        for (input, expected_output) in test_cases {
            let result = mime_type_codec(input);
            assert!(result.is_ok(), "Failed on input: {}", input);
            let (mime_type, codecs) = result.unwrap();
            assert_eq!(
                mime_type, expected_output.0,
                "Mismatched mime type on input: {}",
                input
            );
            assert_eq!(
                codecs, expected_output.1,
                "Mismatched codecs on input: {}",
                input
            );
        }
    }

    #[test]
    fn test_mime_type_codec_invalid() {
        let test_cases = vec![
            "audio/webm; codecs=opus, vorbis",
            "text/plain; codecs=",
            "invalid_input_string",
        ];

        for input in test_cases {
            let result = mime_type_codec(input);
            assert!(
                result.is_err(),
                "Expected an error on invalid input: {}",
                input
            );
        }
    }
}
