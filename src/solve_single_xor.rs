use ascii_utils::Check;

/*
The hex encoded string:
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
has been XOR'd against a single character. Find the key, decrypt the message.
You can do this by hand. But don't: write code to do it for you.
How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
*/

pub fn all_single_byte_xors(ciphertext: &Vec<u8>) -> Vec<String> {
    let mut res = Vec::new();
    for c in 0..=std::u8::MAX {
        let ciphered = xor_cipher(ciphertext, c);
        if ciphered.iter().all(|byte| byte.is_printable()) {
            res.push(std::str::from_utf8(&ciphered).unwrap().to_string());
        }
    }
    res
}

pub fn xor_cipher(text: &Vec<u8>, cipher: u8) -> Vec<u8> {
    text.iter().map(|x| x ^ cipher).collect()
}
