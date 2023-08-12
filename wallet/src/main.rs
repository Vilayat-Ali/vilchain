use wallet::cred::generate_wallet_creds;

fn main() {
    let (wallet_address, seed_words) = generate_wallet_creds();

    println!("Wallet Address: {}", wallet_address);
    for seed_word in seed_words.iter() {
        println!("Seed word: {}", seed_word);
    }
}
