use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand};
use crypto::{decrypt::decrypt, encrypt::encrypt, key::generate};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Parser)]
#[command(
    author = "thanhtan541",
    version = "0.1.0",
    about = "RSA encryption/decryption CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate an RSA key pair in pem format")]
    Generate {
        #[arg(short, long, help = "Enter file in which to save the key")]
        filename: String,
    },
    #[command(about = "Encrypt a message")]
    Encrypt {
        #[arg(short, long, help = "Path to public key")]
        public_key: String,
        #[arg(short, long)]
        message: String,
    },
    #[command(about = "Decrypt a message")]
    Decrypt {
        #[arg(short, long, help = "Path to private key")]
        private_key: String,
        #[arg(short, long)]
        ciphertext: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { filename } => {
            let keypair = generate().expect("Failed to generate keypair");

            //Todo: use thread to speed up process
            println!("Generating your private key");
            let priv_key_pem = keypair.secret.to_pkcs8_pem(LineEnding::LF).unwrap();
            let mut private_key_file =
                File::create(filename.clone()).expect("Failed to create public key file");
            private_key_file
                .write_all(priv_key_pem.as_bytes())
                .expect("Failed to write file content");
            println!("Your private key has been saved in {}", filename);

            println!("Generating your public key");
            let pub_key_pem = keypair.public.to_public_key_pem(LineEnding::LF).unwrap();
            let mut public_key_file = File::create(format!("{}.pub", filename))
                .expect("Failed to create public key file");
            public_key_file
                .write_all(pub_key_pem.as_bytes())
                .expect("Failed to write file content");

            println!("Your public key has been saved in {}.pub", filename);
        }
        Commands::Encrypt {
            public_key,
            message,
        } => {
            let key_pem = fs::read_to_string(public_key).expect("Failed to read file");
            let key = RsaPublicKey::from_public_key_pem(key_pem.as_str()).unwrap();
            let ciphertext = encrypt(message.as_bytes(), &key).expect("Failed to encrypt message");
            let encoded_cipher = general_purpose::STANDARD.encode(&ciphertext);
            println!("Ciphertext: \n {:?}", encoded_cipher);
        }
        Commands::Decrypt {
            private_key,
            ciphertext,
        } => {
            let key_pem = fs::read_to_string(private_key).expect("Failed to read file");
            let key = RsaPrivateKey::from_pkcs8_pem(key_pem.as_str()).unwrap();
            let decoded_cipher = general_purpose::STANDARD
                .decode(ciphertext.as_str())
                .expect("Failed to decode base64 text");
            let plaintext =
                decrypt(decoded_cipher.as_ref(), &key).expect("Failed to encrypt message");
            println!("Plaintext: \n {}", String::from_utf8(plaintext).unwrap());
        }
    }
}
