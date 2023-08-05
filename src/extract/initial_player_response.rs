use crate::{parse_for_object, AppResult, YoutubeError};

pub fn initial_player_response(watch_html: &str) -> AppResult<serde_json::Value> {
    let patterns = [
        r#"window\[['"]ytInitialPlayerResponse['"]\]\s*=\s*"#,
        r"ytInitialPlayerResponse\s*=\s*",
    ];

    for pattern in patterns.iter() {
        if let Ok(result) = parse_for_object(watch_html, pattern) {
            return Ok(result);
        }
    }

    Err(YoutubeError::RegexMatchError {
        caller: "initial_player_response".to_owned(),
        pattern: "initial_player_response_pattern".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_player_response_success() {
        let watch_html = r#"window['ytInitialPlayerResponse'] = {
            "videoDetails": {
                "videoId": "VIDEO_ID",
                "title": "VIDEO_TITLE",
                "lengthSeconds": "VIDEO_LENGTH_SECONDS",
                "keywords": ["KEYWORD1", "KEYWORD2"],
                "channelId": "CHANNEL_ID",
                "isOwnerViewing": true,
                "shortDescription": "SHORT_DESCRIPTION",
                "isCrawlable": true,
                "thumbnail": {
                    "thumbnails": [{
                        "url": "THUMBNAIL_URL",
                        "width": 120,
                        "height": 90
                    }]
                },
                "allowRatings": true,
                "viewCount": "VIEW_COUNT",
                "author": "AUTHOR",
                "isPrivate": false,
                "isUnpluggedCorpus": false,
                "isLiveContent": false
            },
            "streamingData": {
                "hlsManifestUrl": "HLS_MANIFEST_URL"
            },
            "playbackTracking": {
                "videostatsPlaybackUrl": {
                    "baseUrl": "VIDEOSTATS_PLAYBACK_URL"
                }
            },
            "captions": {
                "playerCaptionsTracklistRenderer": {
                    "captionTracks": [{
                        "baseUrl": "CAPTION_URL",
                        "languageCode": "en",
                        "name": "CAPTION_NAME",
                        "vssId": "VSS_ID"
                    }]
                }
            }
        };"#;

        let expected_result = serde_json::json!({
            "videoDetails": {
                "videoId": "VIDEO_ID",
                "title": "VIDEO_TITLE",
                "lengthSeconds": "VIDEO_LENGTH_SECONDS",
                "keywords": ["KEYWORD1", "KEYWORD2"],
                "channelId": "CHANNEL_ID",
                "isOwnerViewing": true,
                "shortDescription": "SHORT_DESCRIPTION",
                "isCrawlable": true,
                "thumbnail": {
                    "thumbnails": [{
                        "url": "THUMBNAIL_URL",
                        "width": 120,
                        "height": 90
                    }]
                },
                "allowRatings": true,
                "viewCount": "VIEW_COUNT",
                "author": "AUTHOR",
                "isPrivate": false,
                "isUnpluggedCorpus": false,
                "isLiveContent": false
            },
            "streamingData": {
                "hlsManifestUrl": "HLS_MANIFEST_URL"
            },
            "playbackTracking": {
                "videostatsPlaybackUrl": {
                    "baseUrl": "VIDEOSTATS_PLAYBACK_URL"
                }
            },
            "captions": {
                "playerCaptionsTracklistRenderer": {
                    "captionTracks": [{
                        "baseUrl": "CAPTION_URL",
                        "languageCode": "en",
                        "name": "CAPTION_NAME",
                        "vssId": "VSS_ID"
                    }]
                }
            }
        });

        let result = initial_player_response(watch_html).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_initial_player_response_failure() {
        let watch_html = "This is not a valid watch HTML page";
        let result = initial_player_response(watch_html);
        assert!(result.is_err());
    }
}
