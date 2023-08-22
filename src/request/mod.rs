mod execute_request;

use execute_request::execute_request;
use std::collections::HashMap;

pub async fn get(url: &str, extra_headers: Option<HashMap<&str, &str>>) -> reqwest::Result<String> {
    let response = execute_request(url, reqwest::Method::GET, extra_headers, None).await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let url = "https://www.example.com";
        let extra_headers = Some([("User-Agent", "Test-Agent")].iter().cloned().collect());
        let response_text = get(url, extra_headers).await.unwrap();

        assert!(!response_text.is_empty());
    }
}
