/// Supported text file encodings.
pub enum Codec {
    Utf8,
    Utf16Le,
    Utf16Be,
}

/// Helper to detect and decode text bytes from multiple encodings (UTF-8, UTF-16, ANSI) into standard Strings.
pub struct TextCodec;

impl TextCodec {
    /// Detects the text encoding of raw bytes without doing full-file scans.
    pub fn detect_codec(bytes: &[u8]) -> Codec {
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            // UTF-8 with BOM
            Codec::Utf8
        } else if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
            // UTF-16LE with BOM
            Codec::Utf16Le
        } else if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
            // UTF-16BE with BOM
            Codec::Utf16Be
        } else {
            // No BOM. Detect UTF-16LE/BE heuristically by analyzing null-byte patterns of the first 4096 bytes.
            let scan_len = std::cmp::min(bytes.len(), 4096);
            if scan_len >= 4 {
                let mut nulls_odd = 0;
                let mut nulls_even = 0;
                let limit = scan_len - (scan_len % 2);
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
                    return Codec::Utf16Le;
                } else if nulls_even > pairs * 7 / 10 && nulls_odd < pairs * 1 / 10 {
                    // Very high likelihood of UTF-16BE (even Null bytes, odd ASCII bytes)
                    return Codec::Utf16Be;
                }
            }
            Codec::Utf8
        }
    }

    /// Decodes raw bytes using the specified text codec.
    pub fn decode_with_codec(bytes: &[u8], codec: &Codec) -> String {
        match codec {
            Codec::Utf8 => {
                if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
                    String::from_utf8_lossy(&bytes[3..]).into_owned()
                } else {
                    String::from_utf8_lossy(bytes).into_owned()
                }
            }
            Codec::Utf16Le => {
                let start = if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE { 2 } else { 0 };
                Self::decode_utf16_le(&bytes[start..])
            }
            Codec::Utf16Be => {
                let start = if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF { 2 } else { 0 };
                Self::decode_utf16_be(&bytes[start..])
            }
        }
    }

    /// Detects the text encoding of raw bytes and decodes them into a standard UTF-8 String (legacy helper).
    pub fn to_utf8_string(bytes: &[u8]) -> String {
        let codec = Self::detect_codec(bytes);
        Self::decode_with_codec(bytes, &codec)
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