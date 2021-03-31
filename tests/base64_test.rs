use cryptopals::encode;
use cryptopals::hex_str_to_u8;

#[test]
fn single_test() {
    assert_eq!(encode(&vec![48, 48, 48]), String::from("MDAw"));

    assert_eq!(
        encode(&hex_str_to_u8("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")),
        String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    );
}
