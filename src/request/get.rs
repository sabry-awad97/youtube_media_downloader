use super::execute_request;
use reqwest::header::HeaderMap;

pub async fn get(url: &str, extra_headers: Option<HeaderMap>) -> reqwest::Result<String> {
    let response = execute_request(url, reqwest::Method::GET, extra_headers, None).await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderValue;

    use super::*;

    #[tokio::test]
    async fn test_get() {
        let url = "https://www.example.com";
        let mut extra_headers = HeaderMap::new();
        extra_headers.insert("User-Agent", HeaderValue::from_static("Test-Agent"));

        let response_text = get(url, Some(extra_headers)).await.unwrap();

        assert!(!response_text.is_empty());
    }
}
