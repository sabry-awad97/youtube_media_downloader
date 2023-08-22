use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Response};
use std::collections::HashMap;

pub async fn execute_request(
    url: &str,
    method: reqwest::Method,
    headers: Option<HashMap<&str, &str>>,
    data: Option<&serde_json::Value>,
) -> reqwest::Result<Response> {
    let client = Client::new();
    let mut request = client.request(method.clone(), url);

    if let Some(header_map) = headers {
        let mut converted_headers = HeaderMap::new();
        for (key, value) in header_map.iter() {
            let header_name = HeaderName::from_bytes(key.as_bytes()).expect("Invalid header name");
            let header_value = HeaderValue::from_str(value).expect("Invalid header value");
            converted_headers.insert(header_name, header_value);
        }
        request = request.headers(converted_headers);
    }

    if let Some(json_data) = data {
        request = request.json(json_data);
    }

    request.send().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Method, StatusCode};

    #[tokio::test]
    async fn test_execute_request() {
        let url = "https://www.example.com";
        let method = Method::GET;
        let headers = Some([("User-Agent", "Test-Agent")].iter().cloned().collect());
        let response = execute_request(url, method, headers, None).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
