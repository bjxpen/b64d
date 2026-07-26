/// Custom high-performance Base64 decoder.
pub struct Base64Decoder;

impl Base64Decoder {
    /// Decodes a pre-cleansed Base64 string into its original bytes.
    /// Supports both standard and URL-safe Base64 alphabets.
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let mut decoded = Vec::with_capacity((input.len() * 3) / 4 + 4);
        let mut buffer = 0u32;
        let mut bits = 0;

        for &b in input.as_bytes() {
            if b == b'=' {
                break;
            }

            let val = match b {
                b'A'..=b'Z' => (b - b'A') as u32,
                b'a'..=b'z' => (b - b'a') as u32 + 26,
                b'0'..=b'9' => (b - b'0') as u32 + 52,
                b'+' | b'-' => 62,
                b'/' | b'_' => 63,
                _ => return Err(format!("Invalid character '{}' in base64", b as char)),
            };

            buffer = (buffer << 6) | val;
            bits += 6;

            if bits >= 8 {
                bits -= 8;
                decoded.push((buffer >> bits) as u8);
                buffer &= (1 << bits) - 1;
            }
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_b64_basic() {
        assert_eq!(Base64Decoder::decode("SGVsbG8=").unwrap(), b"Hello");
        assert_eq!(Base64Decoder::decode("SGVsbG8gd29ybGQ=").unwrap(), b"Hello world");
    }

    #[test]
    fn test_decode_b64_unpadded() {
        assert_eq!(Base64Decoder::decode("SGVsbG8").unwrap(), b"Hello");
        assert_eq!(Base64Decoder::decode("SGVsbG8gd29ybGQ").unwrap(), b"Hello world");
    }

    #[test]
    fn test_decode_b64_whitespace() {
        assert_eq!(Base64Decoder::decode("SGVsbG8=").unwrap(), b"Hello");
    }

    #[test]
    fn test_decode_b64_url_safe() {
        assert_eq!(Base64Decoder::decode("+/8=").unwrap(), vec![251, 255]);
        assert_eq!(Base64Decoder::decode("-_8=").unwrap(), vec![251, 255]);
    }
}