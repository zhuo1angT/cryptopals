use ascii_utils::Check;
use std::collections::HashMap;

/*
The hex encoded string:
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
has been XOR'd against a single character. Find the key, decrypt the message.
You can do this by hand. But don't: write code to do it for you.
How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
*/

pub fn all_single_byte_xors(ciphertext: &[u8]) -> Vec<String> {
    let mut res = Vec::new();
    for c in 0..=std::u8::MAX {
        let ciphered = xor_cipher(ciphertext, c);
        if ciphered
            .iter()
            .all(|&byte| byte.is_printable() || byte == '\n' as u8)
        {
            res.push(std::str::from_utf8(&ciphered).unwrap().to_string());
        }
    }
    res
}

pub fn best_english_text(texts: &[String]) -> String {
    // http://www.data-compression.com/english.html
    let character_freq: HashMap<char, f64> = [
        ('a', 0.0651738),
        ('b', 0.0124248),
        ('c', 0.0217339),
        ('d', 0.0349835),
        ('e', 0.1041442),
        ('f', 0.0197881),
        ('g', 0.0158610),
        ('h', 0.0492888),
        ('i', 0.0558094),
        ('j', 0.0009033),
        ('k', 0.0050529),
        ('l', 0.0331490),
        ('m', 0.0202124),
        ('n', 0.0564513),
        ('o', 0.0596302),
        ('p', 0.0137645),
        ('q', 0.0008606),
        ('r', 0.0497563),
        ('s', 0.0515760),
        ('t', 0.0729357),
        ('u', 0.0225134),
        ('v', 0.0082903),
        ('w', 0.0171272),
        ('x', 0.0013692),
        ('y', 0.0145984),
        ('z', 0.0007836),
        (' ', 0.1918182),
    ]
    .iter()
    .cloned()
    .collect();

    let mut max_score: f64 = 0.0;
    let mut res = String::new();
    for s in texts {
        let mut cur_str_score = 0.0;
        for ch in s.chars() {
            if let Some(score) = character_freq.get(&ch.to_ascii_lowercase()) {
                cur_str_score += score;
            }
        }
        if cur_str_score > max_score {
            max_score = cur_str_score;
            res = (*s).clone();
        }
    }
    return res;
}

pub fn xor_cipher(text: &[u8], cipher: u8) -> Vec<u8> {
    text.iter().map(|x| x ^ cipher).collect()
}
