use std::fs;

use base64::{engine::general_purpose, Engine};
use crypto::{decrypt::decrypt, encrypt::encrypt};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn encrypt_message(plaintext: String, key_path: String) -> String {
    let key_pem = fs::read_to_string(key_path).expect("Failed to read file");
    let key = RsaPublicKey::from_public_key_pem(key_pem.as_str()).unwrap();
    let ciphertext = encrypt(plaintext.as_bytes(), &key).expect("Failed to generate keypair");

    let encoded_cipher = general_purpose::STANDARD.encode(&ciphertext);
    encoded_cipher
}

#[uniffi::export]
pub fn decrypt_cipher(ciphertext: String, key_path: String) -> String {
    let key_pem = fs::read_to_string(key_path).expect("Failed to read file");
    let key = RsaPrivateKey::from_pkcs8_pem(key_pem.as_str()).unwrap();
    let decoded_cipher = general_purpose::STANDARD
        .decode(ciphertext.as_str())
        .expect("Failed to decode base64 text");
    let plaintext = decrypt(decoded_cipher.as_ref(), &key).expect("Failed to encrypt message");
    String::from_utf8(plaintext).unwrap()
}
