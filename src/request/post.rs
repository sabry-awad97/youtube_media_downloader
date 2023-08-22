use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};

use super::execute_request;

pub async fn post(
    url: &str,
    extra_headers: Option<HeaderMap>,
    data: Option<serde_json::Value>,
) -> reqwest::Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    if let Some(header_map) = extra_headers {
        headers.extend(header_map);
    }

    let response = execute_request(url, Method::POST, Some(headers), data).await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post() {
        let url = "https://example.com";
        let extra_headers = HeaderMap::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "key".to_string(),
            serde_json::Value::String("value".to_string()),
        );
        let result = post(
            url,
            Some(extra_headers),
            Some(serde_json::Value::Object(data)),
        )
        .await;

        assert!(result.is_ok());
    }
}
