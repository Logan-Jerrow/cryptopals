#![allow(unused_variables, dead_code)]

use base64::Engine;

mod set1_chal3;
mod set1_chal6;
pub use set1_chal3::score;

pub(crate) trait Xor {
    fn xor(&self, other: &Self) -> Vec<u8>;
}

impl Xor for [u8] {
    fn xor(&self, other: &Self) -> Vec<u8> {
        other
            .iter()
            .cycle()
            .take(self.len())
            .zip(self)
            .map(|(u, c)| u ^ c)
            .collect()
    }
}

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub fn fixed_xor(this: &str, that: &str) -> String {
    let this = hex::decode(this).unwrap();
    let that = hex::decode(that).unwrap();
    hex::encode(
        this.iter()
            .zip(that)
            .map(|(t1, t2)| t1 ^ t2)
            .collect::<Vec<_>>(),
    )
}

pub fn decipher_single_byte_xor(input: &str) -> Option<String> {
    let hex = hex::decode(input).unwrap();
    let cipher = set1_chal3::single_byte_xor_cipher(&hex);
    let decipher = hex.xor(&[cipher]);
    String::from_utf8(decipher).ok()
}

fn repeate_key_xor(input: &[u8], cipher: &[u8]) -> String {
    hex::encode(String::from_utf8(input.xor(cipher)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64() {
        assert_eq!(
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn xor() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn decipher_single_byte_xor() {
        assert_eq!(
            super::decipher_single_byte_xor(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            )
            .unwrap(),
            "Cooking MC's like a pound of bacon"
        );
    }

    #[test]
    fn repeate() {
        assert_eq!(
            repeate_key_xor(
                b"Burning 'em, if you ain't quick and nimble\n\
                  I go crazy when I hear a cymbal",
                b"ICE"
            ),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }
}
