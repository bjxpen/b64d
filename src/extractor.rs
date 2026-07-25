/// Helper to extract clean Base64 content from various payload formats
/// (such as raw Base64, PEM containers, and Data URLs).
pub struct Base64Extractor;

impl Base64Extractor {
    /// Extracts Base64 content from raw input bytes.
    /// Handles PEM blocks, Data URL schemes, and filters out non-Base64 characters.
    pub fn extract(raw_bytes: &[u8]) -> String {
        let content_str = String::from_utf8_lossy(raw_bytes);
        let mut captured_lines = Vec::new();
        let mut in_block = false;
        let mut has_block = false;

        for line in content_str.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("-----BEGIN") {
                in_block = true;
                has_block = true;
                continue;
            }
            if trimmed.starts_with("-----END") {
                in_block = false;
                continue;
            }
            if has_block {
                if in_block {
                    captured_lines.push(trimmed);
                }
            } else {
                captured_lines.push(trimmed);
            }
        }

        let joined = captured_lines.join("");

        // Handle data URL prefix if any (e.g. "data:text/plain;base64,SGVsbG8=")
        let mut start_idx = 0;
        let joined_bytes = joined.as_bytes();
        if joined_bytes.starts_with(b"data:") {
            if let Some(pos) = joined_bytes.windows(8).position(|w| w == b";base64,") {
                start_idx = pos + 8;
            } else if let Some(pos) = joined_bytes.iter().position(|&b| b == b',') {
                start_idx = pos + 1;
            }
        }

        // Filter and collect only valid base64 alphabet characters
        joined_bytes[start_idx..]
            .iter()
            .map(|&b| b as char)
            .filter(|&c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_' || c == '=')
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_b64_content_data_url() {
        let raw = b"data:text/plain;base64,SGVsbG8gd29ybGQ=";
        assert_eq!(Base64Extractor::extract(raw), "SGVsbG8gd29ybGQ=");

        let raw_simple = b"data:,SGVsbG8=";
        assert_eq!(Base64Extractor::extract(raw_simple), "SGVsbG8=");
    }

    #[test]
    fn test_extract_b64_content_pem() {
        let raw = b"
-----BEGIN CERTIFICATE-----
SGVsbG8=
-----END CERTIFICATE-----
";
        assert_eq!(Base64Extractor::extract(raw), "SGVsbG8=");
    }
}
