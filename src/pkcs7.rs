pub fn pkcs7(text: &[u8], block_size: usize) -> Vec<u8> {
    let len = text.len();
    let padding_size = match len % block_size {
        0 => block_size,
        remainder => block_size - remainder,
    };
    let mut res = text.to_vec();
    let mut padding = vec![padding_size as u8; padding_size];
    res.append(&mut padding);
    res
}
