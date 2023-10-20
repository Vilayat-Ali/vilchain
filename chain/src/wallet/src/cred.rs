use random_word::Lang;
use serde::Serialize;
use sha3::{Digest, Keccak256};

#[derive(Serialize)]
pub struct WalletCreds {
    pub address: String,
    pub seeds: Vec<String>,
}

pub fn generate_wallet_creds() -> WalletCreds {
    let mut seed_word_vec: Vec<String> = Vec::with_capacity(12);

    for _x in 0..12 {
        let word: String = random_word::gen(Lang::En).to_owned();
        seed_word_vec.push(word);
    }

    let struct_bytes: Vec<u8> = serde_json::to_vec(&seed_word_vec).unwrap();
    let mut hasher = Keccak256::new();
    hasher.update(struct_bytes);
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    WalletCreds {
        address: format!("0x{}", hash_hex),
        seeds: seed_word_vec,
    }
}
