use reqwest::header::HeaderMap;
use reqwest::{Client, Response};

pub async fn execute_request(
    url: &str,
    method: reqwest::Method,
    headers: Option<HeaderMap>,
    data: Option<serde_json::Value>,
) -> reqwest::Result<Response> {
    let client = Client::new();
    let mut request = client.request(method.clone(), url);

    if let Some(header_map) = headers {
        for (key, value) in header_map.iter() {
            request = request.header(key, value);
        }
    }

    if let Some(json_data) = data {
        request = request.json(&json_data);
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
        let response = execute_request(url, method, None, None).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
