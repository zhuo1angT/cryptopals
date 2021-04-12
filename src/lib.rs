pub use conversion::hex_str_to_u8_vec;
pub use conversion::single_hex_to_u8;
pub use conversion::u8_vec_to_hex_str;

pub use crate::base64::encode;
pub use crate::base64::encode_one_byte;
pub use crate::base64::encode_two_bytes;

pub use fixed_xor::fixed_xor;

pub use solve_single_xor::all_single_byte_xors;
pub use solve_single_xor::best_english_text;
pub use solve_single_xor::xor_cipher;

pub use repeating_key_xor::repeating_key_xor;

pub use break_repeating_key_xor::bitwise_hamming_dis;

pub mod base64;
pub mod break_repeating_key_xor;
pub mod conversion;
pub mod fixed_xor;
pub mod repeating_key_xor;
pub mod solve_single_xor;
