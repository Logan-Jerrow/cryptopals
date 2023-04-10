use crate::xor::Xor;

pub fn xor_string(this: &str, that: &str) -> String {
    assert_eq!(this.len(), that.len());

    let this = hex::decode(this).unwrap();
    let that = hex::decode(that).unwrap();
    hex::encode(this.xor(&that))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor() {
        assert_eq!(
            xor_string(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }
}
