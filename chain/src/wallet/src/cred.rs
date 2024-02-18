extern crate bip39;
extern crate rand;

use bip39::{Mnemonic, Language, Error};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct WalletCred {
    pub wallet_address: String,
    mnemonic: Mnemonic,
    pub seed_words: Vec<String>,
    private_key: Vec<u8>,
    pub public_key: Vec<u8>
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
        wallet_address: String::default(),
        mnemonic,
        seed_words: Vec::default(),
        private_key: Vec::default(),
        public_key: Vec::default()
    })
}

    pub fn get_seed_words(&self) -> Vec<String> {
        (&self.mnemonic).to_string().split(' ').map(|x| x.trim().to_string()).collect::<Vec<String>>()
    }
}
