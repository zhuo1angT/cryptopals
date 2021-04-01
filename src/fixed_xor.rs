// Takes two equal-length buffers and produces their XOR combination.
pub fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    assert!(buf1.len() == buf2.len());
    let len = buf1.len();
    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        res.push(buf1[i] ^ buf2[i]);
    }
    res
}
