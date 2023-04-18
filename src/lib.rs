#![allow(unused_variables, dead_code)]

pub(crate) trait EditDistance {
    fn hamming_weight(&self, other: &[u8]) -> u32;

    fn normalized_edit_distance(&self, size: usize) -> u32;
}

impl EditDistance for [u8] {
    fn hamming_weight(&self, other: &[u8]) -> u32 {
        if self.len() != other.len() {
            panic!("inputs must have the same length");
        }
        self.xor(other).iter().fold(0, |a, &u| a + u.count_ones())
    }

    fn normalized_edit_distance(&self, size: usize) -> u32 {
        const N_BLOCKS: usize = 8;

        let chunck_size = size * N_BLOCKS;
        let mut it = self.chunks_exact(chunck_size);
        let block1 = it.next().unwrap();
        let block2 = it.next().unwrap();

        let chunk_size = u32::try_from(chunck_size).unwrap();
        block1.hamming_weight(block2) / chunk_size
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
