use crate::Xor;

fn hamming_distance(this: &[u8], other: &[u8]) -> u32 {
    assert_eq!(this.len(), other.len());
    this.xor(other).iter().fold(0, |a, &u| a + u.count_ones())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_37() {
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
}
