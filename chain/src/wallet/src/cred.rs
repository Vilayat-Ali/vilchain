use random_word::Lang;
use serde::Serialize;

#[derive(Serialize)]
pub struct WalletCreds {
    pub address: String,
    pub seeds: Vec<String>,
}

pub fn generate_wallet_creds() -> Result<WalletCreds, bip39::Error> {
    let mut seed_word_vec: Vec<String> = (0..12)
        .into_iter()
        .map(|_| random_word::gen(Lang::En).to_owned())
        .collect::<Vec<String>>();

    Ok(WalletCreds {
        address: format!("0x"),
        seeds: seed_word_vec,
    })
}
