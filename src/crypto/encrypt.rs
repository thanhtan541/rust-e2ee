use rand::thread_rng;
use rsa::Pkcs1v15Encrypt;

use super::KeyPair;

pub fn encrypt(data: &[u8], keypair: &KeyPair) -> Result<Vec<u8>, String> {
    let mut rng = thread_rng();
    let enc_data = keypair
        .public
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");

    Ok(enc_data)
}

#[cfg(test)]
mod test {
    use crate::crypto::generate;

    use super::*;

    #[test]
    fn should_encrypt_message_successfully() {
        let keypair = generate().expect("Fail to generate key");

        assert!(keypair.secret.validate().is_ok());

        let data = b"hello world";
        let enc_data = encrypt(data, &keypair).expect("Fail to encrypt data");

        assert_ne!(&data[..], &enc_data[..]);
    }
}
