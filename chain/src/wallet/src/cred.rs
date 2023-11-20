use bip39::Mnemonic;
use secp256k1::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::filer::{FileType, Filer};

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletKeys {
    pub public_key: Vec<u8>,
    pub private_key: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct WalletCreds {
    pub address: String,
    pub seeds: Vec<String>,
    pub keys: WalletKeys,
}

pub fn generate_wallet_creds() -> Result<(), Box<dyn std::error::Error>> {
    let secp: Secp256k1<All> = Secp256k1::new();

    let (secret_key, public_key): (SecretKey, PublicKey) =
        secp.generate_keypair(&mut rand::thread_rng());

    // generating mneumonics
    // Mnemonic
    // A Secret Recovery Phrase, mnemonic phrase, or Seed Phrase is a set of typically either 12 or 24 words, which can be used to derive an infinite number of accounts. Often times these phrases are used by cryptocurrency hardware wallets, to be written down on a piece of paper by the user to safely back up the users' funds.
    let seed_word_vec: Vec<String> = Mnemonic::from_entropy(secret_key.secret_bytes().as_slice())?
        .to_string()
        .split(' ')
        .map(|x| x.to_owned())
        .collect();

    let contents: String = serde_json::to_string_pretty(&WalletCreds {
        address: format!("0x{}", public_key),
        seeds: seed_word_vec,
        keys: WalletKeys {
            public_key: public_key.serialize().to_vec(),
            private_key: secret_key.secret_bytes(),
        },
    })?;

    let filer = Filer;
    filer.gen_file(
        FileType::Wallet(format!("0x{public_key}")),
        "wallet.json",
        contents,
    )?;

    Ok(())
}

pub fn generate_wallet_spend_history(_address: String) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
