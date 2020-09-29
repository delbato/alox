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
    let mut bytes = vec![];
    let mut i = 0;
    while i < length {
        bytes.push(rng.gen());
        i += 2;
    }
    encode_upper(&bytes)
}

pub fn generate_hash(input: &str) -> String {
    let mut hasher = Blake2b::new();
    hasher.update(input);
    let hash_bytes = hasher.finalize_reset();
    encode_upper(&hash_bytes)
}