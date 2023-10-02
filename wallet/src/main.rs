use wallet::cred::generate_wallet_creds;

fn main() {
    let add = generate_wallet_creds();
    println!("{:#?}", add);
}
