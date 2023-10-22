use std::io::Write;

use wallet::{
    cred::{generate_wallet_creds, WalletCreds},
    filer::*,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if let Ok(creds) = generate_wallet_creds() {
        let mut f = Filer::gen_file(FileType::Cred, "wallet.json")?;
        f.write_all(serde_json::to_string_pretty(&creds)?.as_bytes())?;
    } else {
        println!("fucl");
    }

    Ok(())
}
