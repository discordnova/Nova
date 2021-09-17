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

#[cfg(test)]
mod test {
    use crate::handler::signature::validate_signature;


    #[test]
    fn validate_signature_test() {
        let signature = "543ec3547d57f9ddb1ec4c5c36503ebf288ffda3da3d510764c9a49c2abb57690ef974c63d174771bdd2481de1066966f57abbec12a3ec171b9f6e2373837002";
        let public_key = "eefe0c24473737cb2035232e3b4eb91c206f0a14684168f3503f7d8316058d6f";
        let content = "message de test incroyable".as_bytes().to_vec();
        assert!(validate_signature(public_key, &content, signature))
    }

    #[test]
    fn validate_signature_reverse_test() {
        let signature = "543ec3547d57f9ddb1ec4c5c36503ebf288ffda3da3d510764c9a49c2abb57690ef974c63d174771bdd2481de1066966f57abbec12a3ec171b9f6e2373837002";
        let public_key = "c029eea18437292c87c62aec34e7d1bd4e38fe6126f3f7c446de6375dc666044";
        let content = "ceci est un test qui ne fonctionnera pas!"
            .as_bytes()
            .to_vec();
        assert!(!validate_signature(public_key, &content, signature))
    }

    #[test]
    fn invalid_hex() {
        let signature = "zzz";
        let public_key = "zzz";
        let content = "ceci est un test qui ne fonctionnera pas!"
            .as_bytes()
            .to_vec();
        assert!(!validate_signature(public_key, &content, signature))
    }
}
