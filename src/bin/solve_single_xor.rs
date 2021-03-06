use cryptopals::all_single_byte_xors;
use cryptopals::best_english_text;
use cryptopals::hex_str_to_u8_vec;
use std::env;

fn main() {
    assert!(env::args().len() == 2);
    let args: Vec<String> = env::args().collect();
    let all_results = all_single_byte_xors(&(hex_str_to_u8_vec(&args[1]).to_vec()));
    println!("{}", best_english_text(&all_results));
}
