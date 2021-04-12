use cryptopals::bitwise_hamming_dis;

#[test]
fn break_test() {
    assert_eq!(
        bitwise_hamming_dis("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
        37
    );
}
