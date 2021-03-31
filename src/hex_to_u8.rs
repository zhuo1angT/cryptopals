pub fn single_hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => 0,
    }
}

pub fn hex_str_to_u8(hex_str: &str) -> Vec<u8> {
    let hex_str = hex_str.as_bytes();
    let mut res = Vec::with_capacity(hex_str.len() / 2);
    let mut i: usize = 0;
    while i < hex_str.len() {
        println!("{}", i);
        let mut v: u8 = 0;
        v += single_hex_to_u8(hex_str[i]) << 4;
        v += single_hex_to_u8(hex_str[i + 1]);
        res.push(v);
        i += 2;
    }
    res
}
