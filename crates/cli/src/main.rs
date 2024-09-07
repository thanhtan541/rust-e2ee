use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand};
use crypto::{
    decrypt::decrypt,
    encrypt::encrypt,
    key::{self, generate},
};
use std::{fs::File, io::Write};

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
            let priv_key_pem = keypair.to_privkey_pem();
            export_file(filename.as_str(), &priv_key_pem);
            println!("Your private key has been saved in {}", filename);

            let pub_key_pem = keypair.to_pubkey_pem();
            let pub_filename = format!("{}.pub", filename);
            export_file(&pub_filename, &pub_key_pem);
            println!("Your public key has been saved in {pub_filename}");
        }
        Commands::Encrypt {
            public_key,
            message,
        } => {
            let key = key::get_pubkey_from_pem(public_key).expect("Failed to get Public Key");
            let ciphertext = encrypt(message.as_bytes(), &key).expect("Failed to encrypt message");
            let encoded_cipher = general_purpose::STANDARD.encode(&ciphertext);
            println!("Ciphertext: \n {:?}", encoded_cipher);
        }
        Commands::Decrypt {
            private_key,
            ciphertext,
        } => {
            let key = key::get_privkey_from_pem(private_key).expect("Failed to get Private Key");
            let decoded_cipher = general_purpose::STANDARD
                .decode(ciphertext.as_str())
                .expect("Failed to decode base64 text");
            let plaintext =
                decrypt(decoded_cipher.as_ref(), &key).expect("Failed to encrypt message");
            println!("Plaintext: \n {}", String::from_utf8(plaintext).unwrap());
        }
    }
}

fn export_file(filename: &str, content: &str) {
    let mut exported_file = File::create(filename).expect("Failed to create file");
    exported_file
        .write_all(content.as_bytes())
        .expect("Failed to write file content");
}
