use common::prometheus::{Counter, HistogramVec, labels, opts, register_counter, register_histogram_vec};
use libsodium_sys::crypto_sign_ed25519_verify_detached;

lazy_static::lazy_static! {
    static ref SIGNATURE_TIME_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "nova_webhook_signature_time",
        "The time taken by the signature verification",
        &["signature"]
    ).unwrap();

    static ref SIGNATURE_COUNTER: Counter = register_counter!(opts!(
        "nova_webhook_signatures_verify",
        "number of signatures verification issued by the service",
        labels! {"handler" => "webhook_main"}
    )).unwrap();
}

/// Checks the signature of a given data using the hex signature and the public key.
pub fn validate_signature(hex_public_key: &str, data: &Vec<u8>, hex_signature: &str) -> bool {
    SIGNATURE_COUNTER.inc();
    let timer = SIGNATURE_TIME_HISTOGRAM.with_label_values(&["webhook_main"]).start_timer();

    // First, we need to check if the signature & private key is valid base64.
    let signature_result = hex::decode(hex_signature);
    let public_key_result = hex::decode(hex_public_key);

    let mut result = false;
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
            result = crypto_sign_ed25519_verify_detached(
                signature_pointer.as_ptr(),
                data_pointer,
                data_len,
                private_key_pointer.as_ptr(),
            ) == 0;
        }
    }

    timer.observe_duration();
    result
}
