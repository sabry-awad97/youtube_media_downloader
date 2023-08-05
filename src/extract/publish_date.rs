use chrono::NaiveDate;
use crate::helpers::regex_search;

pub fn publish_date(watch_html: &str) -> Option<NaiveDate> {
    let pattern = r#"itemprop="datePublished" content="(\d{4}-\d{2}-\d{2})""#;
    let result = regex_search(pattern, watch_html, 1);
    if let Ok(date_str) = result {
        if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d") {
            return Some(parsed_date);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_publish_date_with_valid_date() {
        // Test with valid HTML containing a publish date
        let html = r#"
            <html>
                <head>
                    <meta itemprop="datePublished" content="2023-08-03">
                </head>
                <body>
                    <p>This is a video description.</p>
                </body>
            </html>
        "#;

        let expected_date = NaiveDate::from_ymd_opt(2023, 8, 3);
        assert_eq!(publish_date(html), expected_date);
    }

    #[test]
    fn test_publish_date_with_invalid_date() {
        // Test with HTML containing an invalid date format
        let html = r#"
            <html>
                <head>
                    <meta itemprop="datePublished" content="2023-08">
                </head>
                <body>
                    <p>This is a video description.</p>
                </body>
            </html>
        "#;

        assert_eq!(publish_date(html), None);
    }

    #[test]
    fn test_publish_date_with_no_date() {
        // Test with HTML containing no publish date
        let html = r#"
            <html>
                <head>
                    <title>Test Video</title>
                </head>
                <body>
                    <p>This is a video description.</p>
                </body>
            </html>
        "#;

        assert_eq!(publish_date(html), None);
    }

    #[test]
    fn test_publish_date_with_multiple_dates() {
        // Test with HTML containing multiple publish dates
        let html = r#"
            <html>
                <head>
                    <meta itemprop="datePublished" content="2023-08-03">
                    <meta itemprop="datePublished" content="2022-12-25">
                </head>
                <body>
                    <p>This is a video description.</p>
                </body>
            </html>
        "#;

        // The function should return the first date found
        let expected_date = NaiveDate::from_ymd_opt(2023, 8, 3);
        assert_eq!(publish_date(html), expected_date);
    }
}
