use cryptopals::repeating_key_xor;
use cryptopals::u8_vec_to_hex_str;

#[test]
fn repeating_key_xor_test() {
    assert_eq!(
        u8_vec_to_hex_str(repeating_key_xor(
            &("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec()),
            &("ICE".as_bytes().to_vec())
        )),
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
}
