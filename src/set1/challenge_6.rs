fn hamming_weight(a: &[u8], b: &[u8]) -> Result<u32, String> {
    if a.len() != b.len() {
        return Err("inputs must have the same length".into());
    }
    Ok(a.xor(b).iter().fold(0, |a, &u| a + u.count_ones()))
}

fn normaized_hamming_distance(u: &[u8], size: usize) -> f32 {
    const LENGTH: usize = 4;
    let chunks = u.chunks_exact(size).take(LENGTH).collect_vec();
    if chunks.len() != 4 {
        // f32::INFINITY as u32 == 0
        return f32::INFINITY;
    }

    chunks
        .iter()
        .combinations_with_replacement(2)
        .map(|v| hamming_weight(v[0], v[1]).unwrap())
        .map(|f| f as f32)
        .sum::<f32>()
        / size as f32
}

fn key_size(input: &[u8]) -> Vec<usize> {
    const START: usize = 2;
    const END: usize = 40;
    let limit = input.len() / 4;

    (START..=END.min(limit))
        .map(|keysize| {
            (
                keysize,
                (100_f32 * normaized_hamming_distance(input, keysize)) as u32,
            )
        })
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

    let keysizes = key_size(u);
    for size in keysizes {
        //
        let tb = transpose_block(u, size);
        let key = solve_blocks(tb);
        let bytes = u.xor(&key);
        let english = std::str::from_utf8(&bytes).unwrap().to_string();
        answers.push(english);
    }
    answers
        .iter()
        .min_by_key(|s| statistics::score(s.as_bytes()))
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
        assert_eq!(
            hamming_weight(b"this is a test", b"wokka wokka!!!").unwrap(),
            37
        );
    }

    use super::*;
    use crate::decode_base64;
}

use crate::{statistics, Xor};
use itertools::Itertools;
