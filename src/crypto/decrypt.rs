use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

pub fn decrypt(enc_data: &[u8], key: &RsaPrivateKey) -> Result<Vec<u8>, String> {
    let dec_data = key
        .decrypt(Pkcs1v15Encrypt, enc_data)
        .expect("failed to decrypt");

    Ok(dec_data)
}

#[cfg(test)]
mod test {
    use crate::crypto::{encrypt, generate};

    use super::*;

    #[test]
    fn should_decrypt_message_successfully() {
        let keypair = generate().expect("Fail to generate key");

        assert!(keypair.secret.validate().is_ok());

        let data = b"hello world";
        let enc_data = encrypt(data, &keypair.public).expect("Fail to encrypt data");
        assert_ne!(&data[..], &enc_data[..]);

        let dec_data = decrypt(&enc_data, &keypair.secret).expect("Fail to decrypt cipher");
        assert_eq!(&data[..], &dec_data[..]);
    }
}
