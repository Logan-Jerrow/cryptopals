pub fn decipher_single_byte_xor(s: &str) -> Option<String> {
    use crate::xor::Xor;

    let bytes = hex::decode(s).unwrap();
    let cipher = crate::single_byte_xor(&bytes);
    let decipher = bytes.xor(&[cipher]);
    String::from_utf8(decipher).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decipher_xor() {
        assert_eq!(
            decipher_single_byte_xor(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            )
            .unwrap(),
            "Cooking MC's like a pound of bacon"
        );
    }
}
