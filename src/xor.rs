pub(crate) trait Xor {
    fn xor(&self, other: &Self) -> Vec<u8>;
}

impl Xor for [u8] {
    fn xor(&self, other: &Self) -> Vec<u8> {
        other
            .iter()
            .cycle()
            .take(self.len())
            .zip(self)
            .map(|(u, c)| u ^ c)
            .collect()
    }
}
