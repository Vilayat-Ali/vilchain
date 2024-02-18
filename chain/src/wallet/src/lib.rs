pub mod cred;

pub trait Wallet {
    fn sign_message(&self, message: &[u8]) -> String {
            "".to_string()
    }
}