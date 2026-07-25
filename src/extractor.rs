use crate::text_codec::TextCodec;

/// Helper to extract clean Base64 content from various payload formats
/// (such as raw Base64, PEM containers, and Data URLs).
pub struct Base64Extractor;

impl Base64Extractor {
    /// Extracts Base64 content from raw input bytes.
    /// Handles PEM blocks, Data URL schemes, and filters out non-Base64 characters.
    pub fn extract(raw_bytes: &[u8]) -> String {
        let content_str = TextCodec::to_utf8_string(raw_bytes);
        let mut target = content_str.as_str();

        // 1. Isolate content within PEM block if present
        if let Some(begin_idx) = content_str.find("-----BEGIN") {
            if let Some(nl_offset) = content_str[begin_idx..].find('\n') {
                let start_idx = begin_idx + nl_offset + 1;
                if let Some(end_offset) = content_str[start_idx..].find("-----END") {
                    target = &content_str[start_idx..start_idx + end_offset];
                }
            }
        }

        // 2. Isolate content after Data URL prefix if present
        if target.starts_with("data:") {
            if let Some(pos) = target.find(";base64,") {
                target = &target[pos + 8..];
            } else if let Some(pos) = target.find(',') {
                target = &target[pos + 1..];
            }
        }

        // 3. Filter valid Base64 alphabet characters directly into a single pre-allocated String
        let mut filtered = String::with_capacity(target.len());
        for c in target.chars() {
            if c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_' || c == '=' {
                filtered.push(c);
            }
        }
        filtered
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

    #[test]
    fn test_extract_utf16_le() {
        // "SGVsbG8=" in UTF-16LE with BOM
        let mut raw = vec![0xFF, 0xFE];
        for &c in b"SGVsbG8=" {
            raw.push(c);
            raw.push(0);
        }
        assert_eq!(Base64Extractor::extract(&raw), "SGVsbG8=");
    }

    #[test]
    fn test_extract_utf16_be() {
        // "SGVsbG8=" in UTF-16BE with BOM
        let mut raw = vec![0xFE, 0xFF];
        for &c in b"SGVsbG8=" {
            raw.push(0);
            raw.push(c);
        }
        assert_eq!(Base64Extractor::extract(&raw), "SGVsbG8=");
    }

    #[test]
    fn test_extract_utf16_le_no_bom() {
        // "SGVsbG8=" in UTF-16LE without BOM (longer stream to trigger heuristic)
        let mut raw = Vec::new();
        for _ in 0..10 {
            for &c in b"SGVsbG8=" {
                raw.push(c);
                raw.push(0);
            }
        }
        let extracted = Base64Extractor::extract(&raw);
        assert!(extracted.starts_with("SGVsbG8"));
    }
}