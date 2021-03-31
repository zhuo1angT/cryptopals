use cryptopals::encode;
use cryptopals::encode_one_byte;
use cryptopals::encode_two_bytes;

#[test]
fn single_test() {
    assert_eq!(encode(&[48, 48, 48]), String::from("MDAw"));
}
