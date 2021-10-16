pub fn generate_keypair() -> (
    String,
    [u8; libsodium_sys::crypto_sign_ed25519_SECRETKEYBYTES as usize],
) {
    use libsodium_sys::crypto_sign_ed25519_keypair;
    let pk_s: String;

    let mut pk = [0; libsodium_sys::crypto_sign_ed25519_PUBLICKEYBYTES as usize];
    let mut sk = [0; libsodium_sys::crypto_sign_ed25519_SECRETKEYBYTES as usize];

    let pk_p = pk.as_mut_ptr();
    let sk_p = sk.as_mut_ptr();

    // generate keypair
    unsafe {
        if crypto_sign_ed25519_keypair(pk_p, sk_p) < 0 {
            panic!("keypair generation failed!");
        }
    };

    pk_s = hex::encode(pk);
    return (pk_s, sk);
}

pub fn sign_message(
    msg: Vec<u8>,
    sk: [u8; libsodium_sys::crypto_sign_ed25519_SECRETKEYBYTES as usize],
) -> String {
    use libc::c_ulonglong;
    use libsodium_sys::crypto_sign_ed25519_detached;

    let len = msg.len();
    let mut signature_len: c_ulonglong = 0;
    let mut str = [0; 64];
    unsafe {
        crypto_sign_ed25519_detached(
            str.as_mut_ptr(),
            &mut signature_len,
            msg.as_ptr(),
            len as u64,
            sk.as_ptr(),
        );
    };

    return hex::encode(str);
}