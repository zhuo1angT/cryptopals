// Convert hex to base64

static ENCODE_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn encode(hex: &Vec<u8>) -> String {
    let mut res = String::new();

    let mut cur: usize = 0;

    while cur + 3 <= hex.len() {
        let all: u32 =
            ((hex[cur] as u32) << 16) | ((hex[cur + 1] as u32) << 8) | (hex[cur + 2] as u32);
        let a = ((all >> 18) & 0b111111) as usize;
        let b = ((all >> 12) & 0b111111) as usize;
        let c = ((all >> 6) & 0b111111) as usize;
        let d = ((all >> 0) & 0b111111) as usize;
        res.push(ENCODE_TABLE[a]);
        res.push(ENCODE_TABLE[b]);
        res.push(ENCODE_TABLE[c]);
        res.push(ENCODE_TABLE[d]);
        cur += 3;
    }

    match hex.len() % 3 {
        0 => (),
        1 => res.push_str(&encode_one_byte(hex[hex.len() - 1])),
        2 => res.push_str(&encode_two_bytes(hex[hex.len() - 2], hex[hex.len() - 1])),
        _ => (),
    }
    res
}

pub fn encode_one_byte(first: u8) -> String {
    let all = first;
    let a = all >> 2;
    let b = (all & 0b11) << 4;
    format!("{}{}", ENCODE_TABLE[a as usize], ENCODE_TABLE[b as usize])
}

pub fn encode_two_bytes(first: u8, second: u8) -> String {
    let all = ((first as u32) << 8) | (second as u32);
    let a = all >> 10;
    let b = (all >> 4) & (0b111111);
    let c = (all & 0b1111) << 2;
    format!(
        "{}{}{}",
        ENCODE_TABLE[a as usize], ENCODE_TABLE[b as usize], ENCODE_TABLE[c as usize],
    )
}

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
