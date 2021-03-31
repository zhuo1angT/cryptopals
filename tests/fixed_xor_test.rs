use cryptopals::fixed_xor::fixed_xor;
use cryptopals::hex_str_to_u8_vec;
use cryptopals::u8_vec_to_hex_str;

#[test]
fn fixed_xor_test() {
    assert_eq!(
        u8_vec_to_hex_str(fixed_xor(
            hex_str_to_u8_vec("1c0111001f010100061a024b53535009181c"),
            hex_str_to_u8_vec("686974207468652062756c6c277320657965"),
        )),
        "746865206b696420646f6e277420706c6179",
    );
}
