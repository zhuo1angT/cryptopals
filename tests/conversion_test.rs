use cryptopals::u8_vec_to_hex_str;

#[test]
fn conversion_test() {
    assert_eq!(u8_vec_to_hex_str(vec![0, 1, 16, 17]), "00011011");
}
