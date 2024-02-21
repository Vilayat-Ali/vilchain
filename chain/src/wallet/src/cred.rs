extern crate bip39;
extern crate rand;

use std::fmt::Debug;

use bip39::{Mnemonic, Language, Error};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rsa::{pkcs8::EncodePublicKey, traits::PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct WalletCred {
    pub wallet_address: String,
    mnemonic: Mnemonic,
    pub seed_words: Vec<String>,
    private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey
}

const BITS: usize = 2048;

impl WalletCred {
    pub fn new() -> Result<WalletCred, Error> {
        let mut rng = StdRng::from_entropy();

        let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), BITS).unwrap();
        let public_key = RsaPublicKey::from(&private_key);

        let entropy = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;

        Ok(WalletCred {
            wallet_address: format!("0xasd9034n3248324n32498234h9rfnwef98fy"),
            mnemonic: mnemonic.clone(),
            seed_words: mnemonic.to_string().split(' ').map(|x| x.trim().to_string()).collect::<Vec<String>>(),
            private_key,
            public_key
        })
    }

}
