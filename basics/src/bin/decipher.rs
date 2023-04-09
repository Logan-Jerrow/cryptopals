mod xorcipher {
    #[derive(Debug, Eq)]
    pub(crate) struct XorCipher<'a> {
        original: &'a str,
        english: String,
        score: u32,
    }

    impl<'a> XorCipher<'a> {
        pub(crate) fn new(original: &'a str, english: String, score: u32) -> Self {
            Self {
                original,
                english,
                score,
            }
        }

        #[allow(dead_code)]
        pub fn english(&self) -> &str {
            &self.english
        }
    }

    impl<'a> PartialEq for XorCipher<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.original == other.original
        }
    }

    impl Ord for XorCipher<'_> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.score.cmp(&other.score)
        }
    }

    impl PartialOrd for XorCipher<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.score.partial_cmp(&other.score)
        }
    }

    impl std::fmt::Display for XorCipher<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.english.trim())
        }
    }
}

fn main() {
    let name = std::env::args().nth(1).expect("no file name given");
    let content = std::fs::read_to_string(name).expect("read failed");

    let best_fit = decipher(&content);
    println!("{best_fit}")
}

pub(crate) fn decipher(content: &str) -> xorcipher::XorCipher {
    let mut valid = Vec::new();
    for line in content.lines() {
        if let Some(english) = basics::decipher_single_byte_xor(line) {
            let score = basics::score(english.as_bytes());
            valid.push(xorcipher::XorCipher::new(line.trim(), english, score));
        }
    }

    valid.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumping() {
        let file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/4.txt"));
        assert_eq!(decipher(file).english(), "Now that the party is jumping\n");
    }
}
