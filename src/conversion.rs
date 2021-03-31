pub fn single_hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => 0,
    }
}

pub fn hex_str_to_u8_vec(hex_str: &str) -> Vec<u8> {
    let hex_str = hex_str.as_bytes();
    let mut res = Vec::with_capacity(hex_str.len() / 2);
    let mut i: usize = 0;
    while i < hex_str.len() {
        let mut v: u8 = 0;
        v += single_hex_to_u8(hex_str[i]) << 4;
        v += single_hex_to_u8(hex_str[i + 1]);
        res.push(v);
        i += 2;
    }
    res
}

pub fn u8_vec_to_hex_str(uvec: Vec<u8>) -> String {
    assert!(uvec.len() % 2 == 0);
    let mut res = String::with_capacity(uvec.len() * 2);
    for val in uvec {
        let first_hex = val >> 4;
        let second_hex = val & 0b1111;
        let first_hex = match first_hex {
            0..=9 => ('0' as u8 + first_hex) as char,
            10..=15 => ('a' as u8 + first_hex - 10) as char,
            _ => 0 as char,
        };
        let second_hex = match second_hex {
            0..=9 => ('0' as u8 + second_hex) as char,
            10..=15 => ('a' as u8 + second_hex - 10) as char,
            _ => 0 as char,
        };
        res.push(first_hex);
        res.push(second_hex);
    }
    res
}
