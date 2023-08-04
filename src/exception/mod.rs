use std::fmt;

#[derive(Debug, PartialEq)]
pub enum YoutubeError {
    MaxRetriesExceeded,
    HTMLParseError {
        error_string: String,
    },
    ExtractError,
    RegexMatchError {
        caller: String,
        pattern: String,
    },
    VideoUnavailable {
        video_id: String,
        error_string: String,
    },
    AgeRestrictedError {
        video_id: String,
        error_string: String,
    },
    LiveStreamError {
        video_id: String,
        error_string: String,
    },
    VideoPrivate {
        video_id: String,
        error_string: String,
    },
    RecordingUnavailable {
        video_id: String,
        error_string: String,
    },
    MembersOnly {
        video_id: String,
        error_string: String,
    },
    VideoRegionBlocked {
        video_id: String,
        error_string: String,
    },
}

impl fmt::Display for YoutubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YoutubeError::MaxRetriesExceeded => write!(f, "Maximum number of retries exceeded."),
            YoutubeError::HTMLParseError { error_string } => {
                write!(f, "HTML could not be parsed. {}", error_string)
            }
            YoutubeError::ExtractError => write!(f, "Data extraction based exception."),
            YoutubeError::RegexMatchError { caller, pattern } => {
                write!(f, "{}: could not find match for {}", caller, pattern)
            }
            YoutubeError::VideoUnavailable {
                video_id,
                error_string,
            } => {
                write!(f, "{} is unavailable: {}", video_id, error_string)
            }
            YoutubeError::AgeRestrictedError {
                video_id,
                error_string,
            } => {
                write!(
                    f,
                    "{} is age restricted, and can't be accessed without logging in: {}",
                    video_id, error_string
                )
            }
            YoutubeError::LiveStreamError {
                video_id,
                error_string,
            } => {
                write!(
                    f,
                    "{} is streaming live and cannot be loaded: {}",
                    video_id, error_string
                )
            }
            YoutubeError::VideoPrivate {
                video_id,
                error_string,
            } => {
                write!(f, "{} is a private video: {}", video_id, error_string)
            }
            YoutubeError::RecordingUnavailable {
                video_id,
                error_string,
            } => {
                write!(
                    f,
                    "{} does not have a live stream recording available: {}",
                    video_id, error_string
                )
            }
            YoutubeError::MembersOnly {
                video_id,
                error_string,
            } => {
                write!(f, "{} is a members-only video: {}", video_id, error_string)
            }
            YoutubeError::VideoRegionBlocked {
                video_id,
                error_string,
            } => {
                write!(
                    f,
                    "{} is not available in your region: {}",
                    video_id, error_string
                )
            }
        }
    }
}

impl std::error::Error for YoutubeError {}
