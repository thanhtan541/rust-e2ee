use std::fs;

use rand::thread_rng;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};

pub struct KeyPair {
    pub secret: RsaPrivateKey,
    pub public: RsaPublicKey,
}

impl KeyPair {
    pub fn to_pubkey_pem(&self) -> String {
        self.public
            .to_public_key_pem(LineEnding::LF)
            .expect("Failed to get key pem")
    }

    pub fn to_privkey_pem(&self) -> String {
        self.secret
            .to_pkcs8_pem(LineEnding::LF)
            .expect("Failed to get key pem")
            .to_string()
    }
}

pub fn generate() -> Result<KeyPair, String> {
    let mut rng = thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let keypair = KeyPair {
        public: pub_key,
        secret: priv_key,
    };

    Ok(keypair)
}

pub fn get_pubkey_from_pem(key_path: String) -> Result<RsaPublicKey, String> {
    let key_pem = fs::read_to_string(key_path).expect("Failed to read file");
    let key =
        RsaPublicKey::from_public_key_pem(key_pem.as_str()).expect("Failed to generate Public Key");

    Ok(key)
}

pub fn get_privkey_from_pem(key_path: String) -> Result<RsaPrivateKey, String> {
    let key_pem = fs::read_to_string(key_path).expect("Failed to read file");
    let key =
        RsaPrivateKey::from_pkcs8_pem(key_pem.as_str()).expect("Failed to generate Private Key");

    Ok(key)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_keypair_successfully() {
        let keypair = generate().expect("Fail to generate key");

        assert!(keypair.secret.validate().is_ok());
    }
}
