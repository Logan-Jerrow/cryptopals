#![allow(unused_variables, dead_code)]

use crate::xor::Xor;
use base64::Engine;

pub(crate) use crate::set1::challenge_3::decipher_single_byte_xor;

mod set1;

pub(crate) mod statistics;
pub(crate) mod xor;

pub fn encode_base64(u: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(u)
}

pub fn decode_base64(u: &[u8]) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD.decode(u).unwrap()
}

pub fn single_byte_xor(u: &[u8]) -> u8 {
    (0_u8..=u8::MAX)
        .min_by_key(|&byte| statistics::score(&u.xor(&[byte])))
        .unwrap()
}
