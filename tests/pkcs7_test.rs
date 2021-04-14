use cryptopals::pkcs7;

#[test]
fn pkcs7_test() {
    let mut result: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
    for _ in 0..4 {
        result.push(4);
    }
    assert_eq!(pkcs7("YELLOW SUBMARINE".as_bytes(), 20), result);
}
