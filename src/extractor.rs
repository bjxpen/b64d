use crate::text_codec::TextCodec;

/// Helper to extract clean Base64 content from various payload formats
/// (such as raw Base64, PEM containers, and Data URLs).
pub struct Base64Extractor;

impl Base64Extractor {
    /// Extracts Base64 content from raw input bytes.
    /// Handles PEM blocks, Data URL schemes, and filters out non-Base64 characters.
    /// Validates that the payload is actually Base64 by checking character density on a sample.
    pub fn extract(raw_bytes: &[u8]) -> Result<String, String> {
        // 1. Detect the codec of the input file
        let codec = TextCodec::detect_codec(raw_bytes);

        // 2. Perform density validation on a tiny sample (at most 8192 bytes)
        // directly, to avoid O(N) full-file UTF-8/UTF-16 scans and allocations on large files.
        let sample_bytes_len = std::cmp::min(raw_bytes.len(), 8192);
        let raw_sample = &raw_bytes[..sample_bytes_len];

        // Decode only the tiny 8 KB sample into a string for the density check
        let sample_str = TextCodec::decode_with_codec(raw_sample, &codec);
        let mut sample_target = sample_str.as_str();

        // If the sample starts with a PEM header or a Data URL, adjust the sample target
        if let Some(begin_idx) = sample_str.find("-----BEGIN") {
            if let Some(nl_offset) = sample_str[begin_idx..].find('\n') {
                sample_target = &sample_str[begin_idx + nl_offset + 1..];
            }
        } else if sample_target.starts_with("data:") {
            if let Some(pos) = sample_target.find(";base64,") {
                sample_target = &sample_target[pos + 8..];
            } else if let Some(pos) = sample_target.find(',') {
                sample_target = &sample_target[pos + 1..];
            }
        }

        let mut sample_total = 0;
        let mut sample_valid = 0;

        for c in sample_target.chars() {
            if c == '\n' || c == '\r' || c.is_whitespace() {
                continue;
            }
            sample_total += 1;
            if Self::is_valid_b64_char(c) {
                sample_valid += 1;
            }
        }

        if sample_total == 0 {
            return Err("File contains no readable text content.".to_string());
        }

        let density = sample_valid as f64 / sample_total as f64;
        if density < 0.95 {
            return Err(format!(
                "File does not appear to be Base64 encoded. Only {:.1}% of characters in the sample are valid Base64.",
                density * 100.0
            ));
        }

        // 3. ONLY if the density check passes on the sample do we convert and process the FULL file!
        let content_str = TextCodec::decode_with_codec(raw_bytes, &codec);
        let mut target = content_str.as_str();

        // Isolate content within PEM block if present
        if let Some(begin_idx) = content_str.find("-----BEGIN") {
            if let Some(nl_offset) = content_str[begin_idx..].find('\n') {
                let start_idx = begin_idx + nl_offset + 1;
                if let Some(end_offset) = content_str[start_idx..].find("-----END") {
                    target = &content_str[start_idx..start_idx + end_offset];
                }
            }
        }

        // Isolate content after Data URL prefix if present
        if target.starts_with("data:") {
            if let Some(pos) = target.find(";base64,") {
                target = &target[pos + 8..];
            } else if let Some(pos) = target.find(',') {
                target = &target[pos + 1..];
            }
        }

        // Filter and collect into a single pre-allocated String
        let mut filtered = String::with_capacity(target.len());
        for c in target.chars() {
            if Self::is_valid_b64_char(c) {
                filtered.push(c);
            }
        }

        Ok(filtered)
    }

    /// Centralized checker to determine if a character belongs to standard or URL-safe Base64 alphabets (with padding).
    #[inline]
    fn is_valid_b64_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_' || c == '='
    }
}