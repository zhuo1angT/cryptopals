use cryptopals::all_single_byte_xors;
use cryptopals::best_english_text;
use cryptopals::bitwise_hamming_dis;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let text: String = fs::read_to_string(&args[1]).unwrap().lines().collect();
    let text = base64::decode(text).unwrap();

    let mut keysize_and_dis: Vec<(usize, f64)> = Vec::with_capacity(40);
    for keysize in 2..=40 {
        keysize_and_dis.push((
            keysize,
            (bitwise_hamming_dis(&text[..keysize], &text[keysize..keysize * 2])
                + bitwise_hamming_dis(
                    &text[keysize * 2..keysize * 3],
                    &text[keysize * 3..keysize * 4],
                )) as f64
                / keysize as f64,
        ));
    }
    keysize_and_dis.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Compared with f64 NaN"));

    for (keysize, _dis) in keysize_and_dis {
        let mut blocks: Vec<Vec<u8>> = vec![Vec::new(); keysize];
        for (i, &byte) in text.iter().enumerate() {
            blocks[i % keysize].push(byte);
        }
        let snippits: Vec<String> = blocks
            .iter()
            .map(|block| best_english_text(&all_single_byte_xors(block)))
            .collect();

        let mut current_res = String::new();
        for i in 0..snippits[0].len() {
            for j in 0..snippits.len() {
                if i < snippits[j].len() {
                    current_res.push(snippits[j].as_bytes()[i] as char);
                } else {
                    break;
                }
            }
        }

        if current_res.len() != 0 {
            println!("{}", current_res);
            return;
        }
    }
}
