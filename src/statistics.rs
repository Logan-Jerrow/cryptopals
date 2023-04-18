#[rustfmt::skip]
const LETTER_FREQS: [(u8, f32); 27] = [
    (b'a', 0.08167), (b'b', 0.01492), (b'c', 0.02782), (b'd', 0.04253),
    (b'e', 0.012702), (b'f', 0.02228), (b'g', 0.02015), (b'h', 0.06094),
    (b'i', 0.06966), (b'j', 0.00153), (b'k', 0.00772), (b'l', 0.04025),
    (b'm', 0.02406), (b'n', 0.06749), (b'o', 0.07507), (b'p', 0.01929),
    (b'q', 0.00095), (b'r', 0.05987), (b's', 0.06327), (b't', 0.09056),
    (b'u', 0.02758), (b'v', 0.00978), (b'w', 0.02360), (b'x', 0.00150),
    (b'y', 0.01974), (b'z', 0.00074), (b' ', 0.13),
];

const fn english_byte_freqs() -> [f32; 0xFF] {
    let mut byte_freqs = [0_f32; 0xFF];

    let mut i = 0_usize;
    while i < LETTER_FREQS.len() {
        let (key, value) = LETTER_FREQS[i];
        byte_freqs[key as usize] = value;
        i += 1;
    }

    byte_freqs
}

pub fn byte_frequencies(buf: &[u8]) -> [f32; 0xFF] {
    //
    let mut freq = [0_f32; 0xFF];

    for &b in buf {
        let i: usize = b.into();
        freq[i] += 1.0;
    }

    let count = buf.len() as f32;
    for v in freq.iter_mut() {
        *v /= count; // normalize
    }

    freq
}
// consider better like "earth-mover"
pub fn cosine_similarity(u: &[u8]) -> f32 {
    let Ok(valid) = std::str::from_utf8(u) else {
        return 0.0;
    };

    const ENGLISH_FREQS: &[f32; 255] = &english_byte_freqs();
    const RANGE: std::ops::Range<usize> = 0..0xFF;

    let lower = valid.to_lowercase();
    let msg_frequencies = byte_frequencies(lower.as_bytes());
    let mut cos_sim = 0.0;
    let mut norm_msg_frq = 0.0;
    let mut norm_engl_frq = 0.0;

    for i in RANGE {
        //
        cos_sim += msg_frequencies[i] * ENGLISH_FREQS[i];
        norm_msg_frq += msg_frequencies[i].powi(2);
        norm_engl_frq += ENGLISH_FREQS[i].powi(2);
    }
    norm_msg_frq = norm_msg_frq.sqrt();
    norm_engl_frq = norm_engl_frq.sqrt();
    cos_sim /= norm_msg_frq * norm_engl_frq;

    cos_sim
}

pub fn score(v: &[u8]) -> u32 {
    if !v.is_ascii() {
        return u32::MAX;
    }

    if v.iter().any(|&c| c.is_ascii_control() && c != b'\n') {
        return u32::MAX;
    }

    let counts = char_counts(v);
    let length = v.len() as f32;

    LETTER_FREQS.iter().fold(0_f32, |acc, &(c, frq)| {
        let expected_count = frq * length;
        let actual_count = *counts.get(&c).unwrap_or(&0f32);

        acc + (expected_count - actual_count).powi(2)
    }) as u32
}

fn char_counts(input: &[u8]) -> HashMap<u8, f32> {
    let mut table = HashMap::new();
    for c in input {
        if c.is_ascii_control() {
            continue;
        }

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

#[cfg(test)]
mod tests {
    #[test]
    fn freq() {
        let fr = byte_frequencies(b"baaaaaaaaa");
        assert_eq!(fr[b'a' as usize], 0.9f32);
        assert_eq!(fr[b'b' as usize], 0.1f32);
    }

    #[test]
    fn cosine() {
        let red_rover = cosine_similarity(b"Red rover red rover. Why won't you come over");
        let a = cosine_similarity(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        dbg!(red_rover);
        dbg!(a);
    }

    use super::*;
}

use std::collections::HashMap;
