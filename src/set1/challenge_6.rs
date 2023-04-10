//
const GUESS_RANGE: RangeInclusive<usize> = 1..=40;

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

fn key_size(input: &[u8]) -> Vec<Vec<f32>> {
    GUESS_RANGE
        .map(|size| {
            let mut chunk = input.chunks(size);
            let mut norms = vec![];
            loop {
                let Some(a_chunk) = chunk.next() else{break;};
                let Some(b_chunk) = chunk.next() else{break;};
                //
                if let Some(h) = hamming_distance(a_chunk, b_chunk) {
                    let norm = h as f32 / size as f32;
                    norms.push(norm);
                    println!(" {:.3}, ", norm);
                };
            }
            println!();
            println!();
            norms
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::*;

    #[test]
    fn scratchpad() {
        //
        let file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/6.txt"));
        let s = file.lines().collect_vec().join("");
        let base64 = crate::decode_base64(s.as_bytes());

        let keys = key_size(&base64);
        for hamming in keys {
            println!("{hamming:?}");
        }
        let key = crate::single_byte_xor(&base64);
    }

    #[test]
    fn hamming_37() {
        assert_eq!(
            hamming_distance(b"this is a test", b"wokka wokka!!!").unwrap(),
            37
        );
    }
}

use crate::Xor;
use std::ops::RangeInclusive;
