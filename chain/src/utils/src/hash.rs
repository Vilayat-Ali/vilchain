use serde::Serialize;
use sha3::{Digest, Keccak256};

pub fn compute_hash<T>(val: T) -> String
where
    T: Serialize,
{
    let struct_bytes: Vec<u8> = serde_json::to_vec(&val).unwrap();
    let mut hasher = Keccak256::new();
    hasher.update(struct_bytes);
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);
    format!("0x{}", hash_hex)
}
