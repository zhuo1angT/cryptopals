pub fn repeating_key_xor(text: &[u8], cipher: &[u8]) -> Vec<u8> {
    assert!(cipher.len() != 0);
    let mut res = Vec::with_capacity(text.len());
    let mut counter = 0;
    for &byte in text {
        res.push(byte ^ cipher[counter]);
        counter += 1;
        if counter >= cipher.len() {
            counter = 0;
        }
    }
    res
}
