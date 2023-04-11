use crate::xor::Xor;

pub fn decipher_single_byte_xor(u: &[u8]) -> Option<String> {
    let cipher = crate::single_byte_xor(u);
    let decipher = u.xor(&[cipher]);
    String::from_utf8(decipher).ok()
}

pub fn three(hex: &str) -> Option<String> {
    decipher_single_byte_xor(&hex::decode(hex).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decipher_xor() {
        assert_eq!(
            three("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap(),
            "Cooking MC's like a pound of bacon"
        );
    }
}
