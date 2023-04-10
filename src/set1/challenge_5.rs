pub fn repeate_key_xor(input: &[u8], cipher: &[u8]) -> String {
    use crate::xor::Xor;

    hex::encode(String::from_utf8(input.xor(cipher)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeate() {
        assert_eq!(
            repeate_key_xor(
                b"Burning 'em, if you ain't quick and nimble\n\
                  I go crazy when I hear a cymbal",
                b"ICE"
            ),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }
}
