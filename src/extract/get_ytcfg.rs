use crate::{parse_for_all_objects, AppResult, YoutubeError};
use serde_json::Value;

pub fn get_ytcfg(html: &str) -> AppResult<Value> {
    let mut ytcfg = serde_json::Map::new();
    let ytcfg_patterns = vec![r"ytcfg\s=\s", r"ytcfg\.set\("];

    for pattern in &ytcfg_patterns {
        if let Ok(found_objects) = parse_for_all_objects(html, pattern) {
            for obj in found_objects {
                if let Some(map) = obj.as_object() {
                    ytcfg.extend(map.clone());
                }
            }
        }
    }

    if !ytcfg.is_empty() {
        return Ok(Value::Object(ytcfg));
    }

    Err(YoutubeError::RegexMatchError {
        caller: "get_ytcfg".to_string(),
        pattern: ytcfg_patterns.join(", "),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_ytcfg_with_ytcfg_set() {
        let html = r#"
            <script>
                ytcfg.set({
                    "CSI_SERVICE_NAME": "youtube",
                    "INNERTUBE_CONTEXT_CLIENT_NAME": "WEB",
                    "INNERTUBE_CONTEXT_CLIENT_VERSION": "2.20210721.00.00",
                    "INNERTUBE_CONTEXT_GL": "US",
                    "INNERTUBE_CONTEXT_HL": "en",
                    "INNERTUBE_CONTEXT_MOBILE": "false",
                    "INNERTUBE_CONTEXT_URL_GEN": "https://www.youtube.com/youtubei/v1",
                    "INNERTUBE_CONTEXT_VERSION": "2.0",
                    "LOAD_SERVICE_NAME": "YouTube",
                    "YFID": "jNQXAC9IVRw",
                    "innertube_api_key": "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8",
                    "innertube_context_client_name": "WEB",
                    "innertube_context_client_version": "2.20210721.00.00",
                    "innertube_context_gl": "US",
                    "innertube_context_hl": "en",
                    "innertube_context_mobile": "false",
                    "innertube_context_url_gen": "https://www.youtube.com/youtubei/v1",
                    "innertube_context_version": "2.0",
                    "innertube_api_version": "v1",
                    "innertube_client_name": "WEB",
                    "innertube_client_version": "2.20210721.00.00",
                    "innertube_hl": "en",
                    "innertube_sws_version": "0.0.1",
                    "use_encrypted_video_url": "true"
                });
            </script>
        "#;

        let expected_ytcfg = json!({
            "CSI_SERVICE_NAME": "youtube",
            "INNERTUBE_CONTEXT_CLIENT_NAME": "WEB",
            "INNERTUBE_CONTEXT_CLIENT_VERSION": "2.20210721.00.00",
            "INNERTUBE_CONTEXT_GL": "US",
            "INNERTUBE_CONTEXT_HL": "en",
            "INNERTUBE_CONTEXT_MOBILE": "false",
            "INNERTUBE_CONTEXT_URL_GEN": "https://www.youtube.com/youtubei/v1",
            "INNERTUBE_CONTEXT_VERSION": "2.0",
            "LOAD_SERVICE_NAME": "YouTube",
            "YFID": "jNQXAC9IVRw",
            "innertube_api_key": "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8",
            "innertube_context_client_name": "WEB",
            "innertube_context_client_version": "2.20210721.00.00",
            "innertube_context_gl": "US",
            "innertube_context_hl": "en",
            "innertube_context_mobile": "false",
            "innertube_context_url_gen": "https://www.youtube.com/youtubei/v1",
            "innertube_context_version": "2.0",
            "innertube_api_version": "v1",
            "innertube_client_name": "WEB",
            "innertube_client_version": "2.20210721.00.00",
            "innertube_hl": "en",
            "innertube_sws_version": "0.0.1",
            "use_encrypted_video_url": "true"
        });

        assert_eq!(get_ytcfg(html), Ok(expected_ytcfg))
    }

    #[test]
    fn test_get_ytcfg_with_ytcfg_assignment() {
        let html = r#"
            <script>
                var ytcfg = { "key1": "value1", "key2": "value2" };
                var somethingElse = "xyz";
            </script>
        "#;

        let result = get_ytcfg(html);

        // Expected ytcfg value
        let expected_ytcfg = json!({
            "key1": "value1",
            "key2": "value2",
        });

        assert_eq!(result, Ok(expected_ytcfg));
    }

    #[test]
    fn test_get_ytcfg_no_ytcfg() {
        let html = r#"
            <script>
                var somethingElse = "xyz";
            </script>
        "#;

        // In this case, the function should return an error
        let result = get_ytcfg(html);

        // Expected error type
        let expected_error = YoutubeError::RegexMatchError {
            caller: "get_ytcfg".to_string(),
            pattern: "ytcfg\\s=\\s, ytcfg\\.set\\(".to_string(),
        };

        assert_eq!(result, Err(expected_error));
    }
}
