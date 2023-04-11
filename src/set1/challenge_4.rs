use crate::statistics;

pub fn find_single_english_line(content: &str) -> String {
    content
        .lines()
        .filter_map(|line| crate::decipher_single_byte_xor(&hex::decode(line).unwrap()))
        .min_by_key(|english| statistics::score(english.as_bytes()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumping() {
        let file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/4.txt"));
        assert_eq!(
            find_single_english_line(file),
            "Now that the party is jumping\n"
        );
    }
}
