

use std::io::Write;
use wallet::cred::generate_wallet_creds;
use wallet::filer::{FileType, Filer};

fn main() {
    let add: wallet::cred::WalletCreds = generate_wallet_creds();

    let mut f = Filer::gen_file(FileType::Log, "wallet-seed.json").unwrap();
    let b = serde_json::to_string_pretty(&add).unwrap();
    let _ = f.write(b.as_bytes());
}
