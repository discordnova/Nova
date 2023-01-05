use ed25519_dalek::{PublicKey, Signature, Verifier};

pub fn validate_signature(public_key: &PublicKey, data: &[u8], hex_signature: &str) -> bool {
    let mut slice: [u8; Signature::BYTE_SIZE] = [0; Signature::BYTE_SIZE];
    let signature_result = hex::decode_to_slice(hex_signature, &mut slice);

    let mut result = false;
    if signature_result.is_ok() {
        result = public_key.verify(data, &Signature::from(slice)).is_ok();
    }

    result
}
