use std::collections::HashMap;

use crate::Xor;

// In English, the space character occurs almost twice as frequently than the top letter (⟨e⟩)[16]
// and the non-alphabetic characters (digits, punctuation, etc.) collectively occupy the fourth
// position (having already included the space) between ⟨t⟩ and ⟨a⟩.
const FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 22.0),
    (b'e', 12.70),
    (b't', 9.056),
    (b'.', 8.500),
    (b'a', 8.167),
    (b'o', 7.507),
    (b'i', 6.966),
    (b'n', 6.749),
    (b's', 6.327),
    (b'h', 6.094),
    (b'r', 5.987),
    (b'd', 4.253),
    (b'l', 4.025),
    (b'c', 2.782),
    (b'u', 2.758),
    (b'm', 2.406),
    (b'w', 2.360),
    (b'f', 2.228),
    (b'g', 2.015),
    (b'y', 1.974),
    (b'p', 1.929),
    (b'b', 1.492),
    (b'v', 0.978),
    (b'k', 0.772),
    (b'j', 0.153),
    (b'x', 0.150),
    (b'q', 0.095),
    (b'z', 0.074),
];

pub fn score(v: &[u8]) -> u32 {
    if !v.is_ascii() {
        return u32::MAX;
    }

    if v.iter().any(|&c| c.is_ascii_control() && c != b'\n') {
        return u32::MAX;
    }

    let counts = char_counts(v);
    let length = v.len() as f32;

    FREQUENCIES.iter().fold(0_f32, |acc, &(c, frq)| {
        let expected_count = frq / 100_f32 * length;
        let actual_count = *counts.get(&c).unwrap_or(&0f32);

        acc + (expected_count - actual_count).powi(2)
    }) as u32
}

fn char_counts(input: &[u8]) -> HashMap<u8, f32> {
    let mut table = HashMap::new();
    for c in input {
        let key = if c.is_ascii_alphabetic() {
            c.to_ascii_lowercase()
        } else if c.is_ascii_whitespace() {
            b' '
        } else {
            b'.'
        };
        *table.entry(key).or_insert(0_f32) += 1_f32;
    }

    table
}

pub(crate) fn single_byte_xor_cipher(input: &[u8]) -> u8 {
    (0_u8..=u8::MAX)
        .min_by_key(|&u| score(&input.xor(&[u])))
        .unwrap()
}
