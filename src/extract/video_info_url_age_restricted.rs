use crate::regex_search;
use std::collections::HashMap;
use url::form_urlencoded;

fn _video_info_url(params: HashMap<&str, &str>) -> String {
    let encoded_params: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish();
    format!("https://www.youtube.com/get_video_info?{}", encoded_params)
}

pub fn video_info_url_age_restricted(video_id: &str, embed_html: &str) -> String {
    let mut sts = String::new();

    let sts_regex = r#""sts"\s*:\s*(\d+)"#;
    if let Ok(result) = regex_search(sts_regex, embed_html, 1) {
        sts = result
    };

    let eurl = format!("https://youtube.googleapis.com/v/{}", video_id);
    let mut params = HashMap::new();
    params.insert("video_id", video_id);
    params.insert("eurl", &eurl);
    params.insert("sts", &sts);
    params.insert("html5", "1");
    params.insert("c", "TVHTML5");
    params.insert("cver", "7.20201028");
    _video_info_url(params)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_video_info_url_age_restricted() {
//         let video_id = "your_video_id_here";
//         let embed_html = r#"your_embed_html_with_sts_123_here"#;

//         let result = video_info_url_age_restricted(video_id, embed_html);

//         let expected_sts = "123";
//         let expected_url = format!(
//         "https://www.youtube.com/get_video_info?video_id={}&eurl=https://youtube.googleapis.com/v/{}&sts={}&html5=1&c=TVHTML5&cver=7.20201028",
//         video_id, video_id, expected_sts
//     );
//         assert_eq!(result, expected_url);
//     }
// }
