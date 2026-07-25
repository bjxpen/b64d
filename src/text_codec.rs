/// Helper to detect and decode text bytes from multiple encodings (UTF-8, UTF-16, ANSI) into standard Strings.
pub struct TextCodec;

impl TextCodec {
    /// Detects the text encoding of raw bytes and decodes them into a standard UTF-8 String.
    pub fn to_utf8_string(bytes: &[u8]) -> String {
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            // UTF-8 with BOM
            String::from_utf8_lossy(&bytes[3..]).into_owned()
        } else if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
            // UTF-16LE with BOM
            Self::decode_utf16_le(&bytes[2..])
        } else if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
            // UTF-16BE with BOM
            Self::decode_utf16_be(&bytes[2..])
        } else {
            // No BOM. Detect UTF-16LE/BE heuristically by analyzing null-byte patterns.
            if bytes.len() >= 4 {
                let mut nulls_odd = 0;
                let mut nulls_even = 0;
                let limit = bytes.len() - (bytes.len() % 2);
                for i in (0..limit).step_by(2) {
                    if bytes[i] == 0 {
                        nulls_even += 1;
                    }
                    if bytes[i + 1] == 0 {
                        nulls_odd += 1;
                    }
                }
                let pairs = limit / 2;
                if nulls_odd > pairs * 7 / 10 && nulls_even < pairs * 1 / 10 {
                    // Very high likelihood of UTF-16LE (even ASCII bytes, odd Null bytes)
                    return Self::decode_utf16_le(bytes);
                } else if nulls_even > pairs * 7 / 10 && nulls_odd < pairs * 1 / 10 {
                    // Very high likelihood of UTF-16BE (even Null bytes, odd ASCII bytes)
                    return Self::decode_utf16_be(bytes);
                }
            }
            // Default fallback: handles standard UTF-8 (no BOM) and 8-bit ANSI codepages (like GBK/Shift-JIS)
            // seamlessly for Base64 alphanumeric characters.
            String::from_utf8_lossy(bytes).into_owned()
        }
    }

    fn decode_utf16_le(bytes: &[u8]) -> String {
        let iter = bytes.chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]));
        std::char::decode_utf16(iter)
            .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER))
            .collect()
    }

    fn decode_utf16_be(bytes: &[u8]) -> String {
        let iter = bytes.chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]));
        std::char::decode_utf16(iter)
            .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_utf8_with_bom() {
        let bytes = b"\xEF\xBB\xBFHello";
        assert_eq!(TextCodec::to_utf8_string(bytes), "Hello");
    }

    #[test]
    fn test_codec_utf16_le_bom() {
        let bytes = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
        assert_eq!(TextCodec::to_utf8_string(bytes), "Hello");
    }

    #[test]
    fn test_codec_utf16_be_bom() {
        let bytes = b"\xFE\xFF\x00H\x00e\x00l\x00l\x00o";
        assert_eq!(TextCodec::to_utf8_string(bytes), "Hello");
    }

    #[test]
    fn test_codec_utf16_le_heuristic() {
        // Longer sequence to trigger the 70%+ null ratio heuristic
        let mut bytes = Vec::new();
        for _ in 0..10 {
            bytes.extend_from_slice(b"H\x00e\x00l\x00l\x00o\x00 \x00w\x00o\x00r\x00l\x00d\x00!\x00");
        }
        assert!(TextCodec::to_utf8_string(&bytes).contains("Hello world!"));
    }
}
