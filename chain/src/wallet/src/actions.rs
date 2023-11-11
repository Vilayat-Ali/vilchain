use secp256k1::{
    ecdsa::{self},
    All, Error, Message, PublicKey, Secp256k1, SecretKey,
};
use sha3::{Digest, Sha3_256};

pub fn sign_message(data: &[u8], private_key: &[u8]) -> Result<(Message, [u8; 64]), Error> {
    let secp: Secp256k1<All> = Secp256k1::new();
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let message: Message = Message::from_digest(result.into());
    let sec_key: SecretKey = SecretKey::from_slice(private_key)?;
    let signature: secp256k1::ecdsa::Signature = secp.sign_ecdsa(&message, &sec_key);

    Ok((message, signature.serialize_compact()))
}

pub fn verify_message(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, Error> {
    let secp: Secp256k1<All> = Secp256k1::new();
    let mut hasher = Sha3_256::new();
    hasher.update(message);
    let result = hasher.finalize();

    let msg: Message = Message::from_digest_slice(result.as_ref())?;
    let sig = ecdsa::Signature::from_compact(signature)?;
    let pubkey = PublicKey::from_slice(public_key)?;

    Ok(secp.verify_ecdsa(&msg, &sig, &pubkey).is_ok())
}
