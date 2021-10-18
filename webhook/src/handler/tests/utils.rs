use rand::rngs::OsRng;
use ed25519_dalek::{Signer, Keypair, Signature};

pub fn generate_keypair() -> Keypair {
    let mut csprng = OsRng{};
     Keypair::generate(&mut csprng)
}

pub fn sign_message(
    message: Vec<u8>,
    keypair: &Keypair,
) -> String {
    let signature: Signature = keypair.sign(&message);
    return hex::encode(signature.to_bytes());
}