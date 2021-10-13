use libsodium_sys::crypto_sign_ed25519_verify_detached;

/// Checks the signature of a given data using the hex signature and the public key.
pub fn validate_signature(hex_public_key: &str, data: &Vec<u8>, hex_signature: &str) -> bool {
    // First, we need to check if the signature & private key is valid base64.
    let signature_result = hex::decode(hex_signature);
    let public_key_result = hex::decode(hex_public_key);

    if signature_result.is_ok() && public_key_result.is_ok() {
        // Since we now have the signatures in u8 vectors. We will initialize all the
        // parameters for the ffi call to sodium.
        let signature_pointer = signature_result.unwrap();
        let private_key_pointer = public_key_result.unwrap();

        let data_pointer = data.as_ptr();
        let data_len = data.len() as u64;

        // A ffi call is considered unsafe by the Rust compiler
        // we assume all the parameters are correct for the call
        unsafe {
            // If the signature is valid, sodium will return 0
            return crypto_sign_ed25519_verify_detached(
                signature_pointer.as_ptr(),
                data_pointer,
                data_len,
                private_key_pointer.as_ptr(),
            ) == 0;
        }
    }
    false
}
