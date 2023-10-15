use random_word::Lang;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use serde::Serialize;

#[derive(Serialize)]
pub struct WalletCreds {
    pub address: String,
    pub seeds: Vec<String>,
}

pub fn generate_wallet_creds() -> WalletCreds {
    let mut seed_word_vec: Vec<String> = Vec::with_capacity(12);
    let mut s = DefaultHasher::new();

    for _x in 0..12 {
        let word: String = random_word::gen(Lang::En).to_owned();
        seed_word_vec.push(word);
    }

    seed_word_vec.hash(&mut s);

    WalletCreds {
        address: format!("0x{:x}", s.finish()),
        seeds: seed_word_vec,
    }
}
