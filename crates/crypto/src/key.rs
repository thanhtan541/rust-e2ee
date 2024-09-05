use rand::thread_rng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct KeyPair {
    pub secret: RsaPrivateKey,
    pub public: RsaPublicKey,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_keypair_successfully() {
        let keypair = generate().expect("Fail to generate key");

        assert!(keypair.secret.validate().is_ok());
    }
}
