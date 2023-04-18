#![allow(unused_variables, dead_code)]

pub(crate) trait EditDistance {
    fn hamming_weight(&self, other: &[u8]) -> Result<u32, String>;
}

impl EditDistance for [u8] {
    fn hamming_weight(&self, other: &[u8]) -> Result<u32, String> {
        if self.len() != other.len() {
            return Err("inputs must have the same length".into());
        }
        Ok(self.xor(other).iter().fold(0, |a, &u| a + u.count_ones()))
    }
}

pub fn encode_base64(u: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(u)
}

pub fn decode_base64(u: &[u8]) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD.decode(u).unwrap()
}

pub fn single_byte_xor(u: &[u8]) -> u8 {
    (0_u8..=u8::MAX)
        // .min_by_key(|&byte| statistics::score(&u.xor(&[byte])))
        .max_by_key(|&byte| (100f32 * statistics::cosine_similarity(&u.xor(&[byte]))) as u32)
        .unwrap()
}

pub(crate) fn slice_to_string(u: &[u8]) -> String {
    std::str::from_utf8(u).unwrap().to_string()
}

pub(crate) use crate::set1::challenge_3::decipher_single_byte_xor;

pub(crate) mod statistics;
pub(crate) mod xor;

mod set1;

use crate::xor::Xor;
use base64::Engine;
