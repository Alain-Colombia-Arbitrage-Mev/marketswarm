//! Minimal, dependency-free base64 (RFC 4648) encode/decode used for
//! image payloads in multimodal inference and the HTTP `/vision` endpoint.

const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            TABLE[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

/// Decodes standard base64, tolerating surrounding whitespace and padding.
/// Stops at the first `=` padding character and ignores ASCII whitespace.
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    for c in input.bytes() {
        let value = match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a' + 26) as u32,
            b'0'..=b'9' => (c - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            b' ' | b'\t' | b'\n' | b'\r' => continue,
            _ => return Err(format!("Invalid base64 character: {c:#x}")),
        };
        acc = (acc << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors_round_trip() {
        for (raw, expected) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(encode(raw.as_bytes()), expected);
            assert_eq!(decode(expected).unwrap(), raw.as_bytes());
        }
    }

    #[test]
    fn binary_round_trip_and_padding_tolerance() {
        let bytes: Vec<u8> = (0..=255).collect();
        let encoded = encode(&bytes);
        assert_eq!(decode(&encoded).unwrap(), bytes);
        // Trailing padding is optional for the decoder.
        let trimmed = encoded.trim_end_matches('=');
        assert_eq!(decode(trimmed).unwrap(), bytes);
    }

    #[test]
    fn whitespace_is_ignored() {
        assert_eq!(decode("Zm 9v\nYg==\n").unwrap(), b"foob");
    }

    #[test]
    fn invalid_character_is_rejected() {
        assert!(decode("Zm9v$").is_err());
    }
}
