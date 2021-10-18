use crate::handler::signature::validate_signature;
use ed25519_dalek::PublicKey;

#[test]
fn validate_signature_test() {
    let signature = "543ec3547d57f9ddb1ec4c5c36503ebf288ffda3da3d510764c9a49c2abb57690ef974c63d174771bdd2481de1066966f57abbec12a3ec171b9f6e2373837002";
    let content = "message de test incroyable".as_bytes().to_vec();
    let public_key = PublicKey::from_bytes(&hex::decode("eefe0c24473737cb2035232e3b4eb91c206f0a14684168f3503f7d8316058d6f").unwrap()).unwrap();

    assert!(validate_signature(&public_key, &content, signature))
}

#[test]
fn validate_signature_reverse_test() {
    let signature = "543ec3547d57f9ddb1ec4c5c36503ebf288ffda3da3d510764c9a49c2abb57690ef974c63d174771bdd2481de1066966f57abbec12a3ec171b9f6e2373837002";
    let public_key = PublicKey::from_bytes(&hex::decode("c029eea18437292c87c62aec34e7d1bd4e38fe6126f3f7c446de6375dc666044").unwrap()).unwrap();

    let content = "ceci est un test qui ne fonctionnera pas!"
        .as_bytes()
        .to_vec();
    assert!(!validate_signature(&public_key, &content, signature))
}

#[test]
fn invalid_hex() {
    let signature = "zzz";
    let public_key = PublicKey::from_bytes(&hex::decode("c029eea18437292c87c62aec34e7d1bd4e38fe6126f3f7c446de6375dc666044").unwrap()).unwrap();

    let content = "ceci est un test qui ne fonctionnera pas!"
        .as_bytes()
        .to_vec();
    assert!(!validate_signature(&public_key, &content, signature))
}