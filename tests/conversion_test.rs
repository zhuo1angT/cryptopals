use cryptopals::u8_vec_to_hex_str;

#[test]
fn conversion_test() {
    assert_eq!(u8_vec_to_hex_str(&[0, 1, 16, 17]), "00011011");
    assert_eq!(u8_vec_to_hex_str(&[162, 130, 178]), "a282b2");
}
