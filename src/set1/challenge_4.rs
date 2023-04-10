use crate::statistics;

pub fn decipher(content: &str) -> String {
    content
        .lines()
        .filter_map(crate::decipher_single_byte_xor)
        .min_by_key(|english| statistics::score(english.as_bytes()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumping() {
        let file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/4.txt"));
        assert_eq!(decipher(file), "Now that the party is jumping\n");
    }
}
