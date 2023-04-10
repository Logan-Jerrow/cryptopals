pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    crate::encode_base64(&bytes)
}

pub fn base64_to_hex(base64: &str) -> String {
    let bytes = crate::decode_base64(base64.as_bytes());
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex), base);
        assert_eq!(base64_to_hex(base), hex);
    }
}
