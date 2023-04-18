fn guess_vignere_keysize(buffer: &[u8]) -> Vec<usize> {
    const MIN_KEYSIZE: usize = 2;
    const MAX_KEYSIZE: usize = 40;

    let limit = buffer.len() / 4;
    (MIN_KEYSIZE..=MAX_KEYSIZE.min(limit))
        .map(|size| (size, buffer.normalized_edit_distance(size)))
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .take(3)
        .map(|(size, _)| size)
        .collect_vec()
}

// Now transpose the blocks: make a block that is the first byte(u8) of every block, and a block that
// is the second byte of every block, and so on.
// [a,b,c,d,e,f] , 2
// = [
// [a,c,e]
// [b,d,f]]
fn transpose_block(u: &[u8], size: usize) -> Vec<Vec<u8>> {
    let mut blocks = (0..size).map(|_| Vec::<u8>::new()).collect_vec();
    for block in u.chunks(size) {
        for (&u, b) in block.iter().zip(blocks.iter_mut()) {
            b.push(u);
        }
    }

    blocks
}

fn solve_blocks(u: Vec<Vec<u8>>) -> Vec<u8> {
    u.iter().map(|b| crate::single_byte_xor(b)).collect()
}

pub fn break_vigenere_cipher(u: &[u8]) -> String {
    let mut answers = Vec::new();

    let keysizes = guess_vignere_keysize(u);
    for size in keysizes {
        let tb = transpose_block(u, size);
        let key = solve_blocks(tb);
        let bytes = u.xor(&key);
        let english = std::str::from_utf8(&bytes).unwrap().to_string();
        answers.push(english);
    }

    answers
        .iter()
        .max_by_key(|s| (100f32 * statistics::cosine_similarity(s.as_bytes())) as u32)
        // .min_by_key(|s| statistics::score(s.as_bytes()))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn decipher_6() {
        let expected = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/6_answer.txt"));

        let file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/6.txt"));
        let input = file.lines().join("");
        let base = decode_base64(input.as_bytes());

        let actual = break_vigenere_cipher(&base);

        assert_eq!(actual, expected);
    }

    #[test]
    fn hamming_37() {
        assert_eq!(b"this is a test".hamming_weight(b"wokka wokka!!!"), 37);
    }

    use super::*;
    use crate::decode_base64;
}

use crate::{statistics, EditDistance, Xor};
use itertools::Itertools;
