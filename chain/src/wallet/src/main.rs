use std::io::Write;

use secp256k1::PublicKey;
use wallet::{
    actions::{sign_message, verify_message},
    cred::{generate_wallet_creds, WalletCreds},
    filer::*,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let creds = generate_wallet_creds().unwrap();
    let mut f = Filer::gen_file(FileType::Cred, "wallet.json")?;
    f.write_all(serde_json::to_string_pretty(&creds)?.as_bytes())?;

    let seckey: [u8; 32] = creds.keys.private_key;
    let pubkey = creds.keys.public_key.as_slice();

    let msg = b"This is some message";

    let (message, signature) = sign_message(msg, &seckey).unwrap();

    println!(
        "{}",
        verify_message(msg, signature.as_slice(), &pubkey).unwrap()
    );

    Ok(())
}
