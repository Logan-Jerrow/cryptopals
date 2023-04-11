//
const GUESS_RANGE: RangeInclusive<usize> = 2..=40;

fn hamming_distance(a: &[u8], b: &[u8]) -> Option<u32> {
    if a.len() != b.len() {
        return None;
    }
    // print!(
    //     "{} <> {}:",
    //     std::str::from_utf8(this).unwrap(),
    //     std::str::from_utf8(other).unwrap(),
    // );
    Some(a.xor(b).iter().fold(0, |a, &u| a + u.count_ones()))
}

fn normaized(hamming: u32, keysize: usize) -> f32 {
    hamming as f32 / keysize as f32
}

fn key_size(input: &[u8]) -> usize {
    let keysize = GUESS_RANGE
        .filter_map(|size| {
            let mut chunk = input.chunks(size);
            let mut norms = vec![];
            loop {
                let Some(a_chunk) = chunk.next() else{break;};
                let Some(b_chunk) = chunk.next() else{break;};
                //
                if let Some(h) = hamming_distance(a_chunk, b_chunk) {
                    let norm = h as f32 / size as f32;
                    norms.push(norm);
                };
            }
            norms.sort_by(|a, b| a.partial_cmp(b).unwrap());
            match !norms.is_empty() {
                true => Some(norms),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    let (size, min) = keysize
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                i,
                v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            )
        })
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("size: {}\n{min:.3?}\n", size + 2);

    size + 2
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
    let keysize = key_size(u);
    let tb = transpose_block(u, keysize);
    let key = solve_blocks(tb);
    crate::slice_to_string(&u.xor(&key))
}

#[cfg(test)]
mod tests {
    #[test]
    fn decipher_6() {
        //
        let file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/6.txt"));
        let input = file.lines().join("");
        let base = decode_base64(input.as_bytes());
        let actual = break_vigenere_cipher(&base);

        println!("{}", actual);
    }
    #[test]
    fn break_ice_2() {
        let expected = "Burning 'em, if you ain't quick and nimble\n\
            I go crazy when I hear a cymbal";
        let cipher = b"ICE";
        let encrypted = hex::decode(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        )
        .unwrap();

        let actual = break_vigenere_cipher(&encrypted);
        assert_eq!(expected, actual);
    }

    #[test]
    fn break_ice() {
        let answer = "Burning 'em, if you ain't quick and nimble\n\
            I go crazy when I hear a cymbal";
        let cipher = b"ICE";
        let encrypted = hex::decode(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        )
        .unwrap();

        let keysize = key_size(&encrypted);
        assert_eq!(keysize, 3);

        let tb = transpose_block(&encrypted, keysize);
        let key = solve_blocks(tb);
        assert_eq!(crate::slice_to_string(&key), "ICE");
        let decrypted = crate::slice_to_string(&encrypted.xor(&key));
        assert_eq!(decrypted, answer);
    }

    #[test]
    fn hamming_37() {
        assert_eq!(
            hamming_distance(b"this is a test", b"wokka wokka!!!").unwrap(),
            37
        );
    }

    use super::*;
    use crate::{decode_base64, xor::Xor};
    use itertools::Itertools;
}

use crate::Xor;
use itertools::Itertools;
use std::ops::RangeInclusive;
