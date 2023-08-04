use regex::Regex;

pub fn safe_filename(s: &str, max_length: usize) -> String {
    // Characters in range 0-31 (0x00-0x1F) are not allowed in ntfs filenames.
    let ntfs_characters: Vec<char> = (0..=31).map(char::from).collect();
    let invalid_chars = vec![
        '"', '#', '$', '%', '\'', '*', ',', '.', '/', ':', ';', '<', '>', '?', '\\', '^', '|', '~',
    ];

    let pattern = format!(
        "[{}]",
        ntfs_characters
            .iter()
            .chain(invalid_chars.iter())
            .map(|&c| regex::escape(&c.to_string()))
            .collect::<Vec<String>>()
            .join("")
    );

    let regex = Regex::new(&pattern).unwrap();
    let filename = regex.replace_all(s, "").to_string();

    filename.chars().take(max_length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Hello, World! How are you?", 30, "Hello World! How are you")]
    #[case("//////invalid\\\\\\\\\\", 15, "invalid")]
    #[case("#####special#####", 10, "special")]
    #[case("abc<>def:ghi?jkl/mno\\pqr", 20, "abcdefghijklmnopqr")]
    #[case("123|456~789", 9, "123456789")]
    fn test_safe_filename(#[case] input: &str, #[case] max_length: usize, #[case] expected: &str) {
        assert_eq!(safe_filename(input, max_length), expected);
    }
}
