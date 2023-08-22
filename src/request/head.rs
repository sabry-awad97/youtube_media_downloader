use reqwest::{header::HeaderMap, Method};

use super::execute_request;

pub async fn head(url: &str) -> Result<HeaderMap, reqwest::Error> {
    let response = execute_request(url, Method::HEAD, None, None).await?;
    let response_headers = response.headers().clone();
    Ok(response_headers)
}
