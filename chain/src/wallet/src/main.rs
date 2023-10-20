use std::io::Write;

use wallet::{
    cred::{generate_wallet_creds, WalletCreds},
    filer::*,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut f = Filer::gen_file(FileType::Log, "log.toml")?;
    let creds: WalletCreds = generate_wallet_creds();
    f.write_all(serde_json::to_string_pretty(&creds)?.as_bytes())?;
    Ok(())
}
