use cryptopals::bit_wise_hamming_dis;

#[test]
fn break_test() {
    assert_eq!(bit_wise_hamming_dis("this is a test", "wokka wokka!!!"), 37);
}
