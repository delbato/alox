pub mod jwt;

use rand::{
    Rng,
    RngCore,
    thread_rng
};
use blake2::{
    Blake2b,
    Digest
};
use hex::encode_upper;

pub fn generate_salt(length: usize) -> String {
    let mut rng = thread_rng();
    let mut ret = String::new();
    for i in 0..length {
        let hex_val = rng.gen_range(0, 15) as u16;
        ret += &format!("{:X}", hex_val);
    }
    ret
}

pub fn generate_hash(input: &str) -> String {
    let mut hasher = Blake2b::new();
    hasher.update(input);
    let hash_bytes = hasher.finalize_reset();
    encode_upper(&hash_bytes)
}