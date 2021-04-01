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
use base64::{
    encode
};

pub fn generate_salt(length: usize) -> String {
    let mut bytes_len = length * 6;
    bytes_len /= 8;
    let mut rng = thread_rng();
    let mut bytes = vec![];
    for _ in 0..bytes_len {
        bytes.push(rng.gen());
    }
    encode(&bytes)
}

pub fn generate_hash(input: &str) -> String {
    let mut hasher = Blake2b::new();
    hasher.update(input);
    let hash_bytes = hasher.finalize_reset();
    encode(&hash_bytes)
}