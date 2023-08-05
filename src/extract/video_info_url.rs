use std::collections::HashMap;

use urlencoding::encode;

fn _video_info_url(params: HashMap<&str, &str>) -> String {
    let encoded_params: String = serde_urlencoded::to_string(params).unwrap();
    format!("https://www.youtube.com/get_video_info?{}", encoded_params)
}

pub fn video_info_url(video_id: &str, watch_url: &str) -> String {
    let encoded = encode(watch_url);

    let mut params = HashMap::new();
    params.insert("video_id", video_id);
    params.insert("ps", "default");
    params.insert("eurl", &encoded);
    params.insert("hl", "en_US");
    params.insert("html5", "1");
    params.insert("c", "TVHTML5");
    params.insert("cver", "7.20201028");

    _video_info_url(params)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_video_info_url_function() {
//         let video_id = "abc123";
//         let watch_url = "https://www.example.com/watch?v=abc123";
//         let expected_url = "https://www.youtube.com/get_video_info?video_id=abc123&ps=default&eurl=https%3A%2F%2Fwww.example.com%2Fwatch%3Fv%3Dabc123&hl=en_US&html5=1&c=TVHTML5&cver=7.20201028";
//         assert_eq!(video_info_url(video_id, watch_url), expected_url);
//     }
// }
