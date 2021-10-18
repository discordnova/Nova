use common::prometheus::{Counter, HistogramVec, labels, opts, register_counter, register_histogram_vec};
use ed25519_dalek::PublicKey;
use ed25519_dalek::Verifier;
use ed25519_dalek::Signature;
use std::convert::TryInto;

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

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn validate_signature(public_key: &PublicKey, data: &Vec<u8>, hex_signature: &str) -> bool {
    SIGNATURE_COUNTER.inc();
    let timer = SIGNATURE_TIME_HISTOGRAM.with_label_values(&["webhook_main"]).start_timer();

    let signature_result = hex::decode(hex_signature);

    let mut result = false;
    if let Ok(signature) = signature_result {
        let sig = Signature::from(demo(signature));

        result = public_key.verify(data, &sig).is_ok();
    }

    timer.observe_duration();
    result
}
