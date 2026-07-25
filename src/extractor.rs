/// Helper to extract clean Base64 content from various payload formats
/// (such as raw Base64, PEM containers, and Data URLs).
pub struct Base64Extractor;

impl Base64Extractor {
    /// Extracts Base64 content from raw input bytes.
    /// Handles PEM blocks, Data URL schemes, and filters out non-Base64 characters.
    pub fn extract(raw_bytes: &[u8]) -> String {
        let mut target = raw_bytes;

        // Extract raw bytes inside PEM block if present
        if let Some(begin) = raw_bytes.windows(10).position(|w| w == b"-----BEGIN") {
            if let Some(nl) = raw_bytes[begin..].iter().position(|&b| b == b'\n') {
                let actual_start = begin + nl + 1;
                if let Some(end) = raw_bytes[actual_start..].windows(8).position(|w| w == b"-----END") {
                    target = &raw_bytes[actual_start..actual_start + end];
                }
            }
        }

        // Handle Data URL prefix
        if target.starts_with(b"data:") {
            if let Some(pos) = target.windows(8).position(|w| w == b";base64,") {
                target = &target[pos + 8..];
            } else if let Some(pos) = target.iter().position(|&b| b == b',') {
                target = &target[pos + 1..];
            }
        }

        // Filter valid Base64 alphabet characters
        target.iter()
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
