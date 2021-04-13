use openssl::symm::{decrypt, Cipher};
use std::env;
use std::fs;

extern crate base64;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);

    let text: String = fs::read_to_string(&args[1]).unwrap().lines().collect();
    let text = base64::decode(text).unwrap();
    let key = &args[2];

    let cipher = Cipher::aes_128_ecb();
    let ciphertext = decrypt(cipher, key.as_bytes(), None, &text).unwrap();

    println!("{}", std::str::from_utf8(&ciphertext).unwrap());
}
