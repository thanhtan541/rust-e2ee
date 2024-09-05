use std::fs;

use clap::{Parser, Subcommand};

use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};
use rust_e2ee::crypto::{decrypt, encrypt, generate};

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
    #[command(about = "Generate an RSA key pair")]
    Generate {
        #[arg(short, long, default_value = "2048")]
        bits: usize,
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
        Commands::Generate { bits } => {
            let keypair = generate().expect("Failed to generate keypair");

            println!(
                "Private Key (PEM):\n{:?}",
                keypair.secret.to_pkcs8_pem(LineEnding::LF).unwrap()
            );
            println!(
                "Public Key (PEM):\n{:?}",
                keypair.public.to_public_key_pem(LineEnding::LF).unwrap()
            );
        }
        Commands::Encrypt {
            public_key,
            message,
        } => {
            let key_pem = fs::read_to_string(public_key).expect("Failed to read file");
            let key = RsaPublicKey::from_public_key_pem(key_pem.as_str()).unwrap();
            let ciphertext = encrypt(message.as_bytes(), &key).expect("Failed to encrypt message");
            println!("Ciphertext: {:?}", ciphertext);
        }
        Commands::Decrypt {
            private_key,
            ciphertext,
        } => {
            let key_pem = fs::read_to_string(private_key).expect("Failed to read file");
            let key = RsaPrivateKey::from_pkcs8_pem(key_pem.as_str()).unwrap();
            let plaintext =
                decrypt(ciphertext.as_bytes(), &key).expect("Failed to encrypt message");
            println!("Plaintext: {}", String::from_utf8(plaintext).unwrap());
        }
    }
}
