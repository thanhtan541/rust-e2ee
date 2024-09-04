use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use rust_e2ee::crypto::{generate, KeyPair};

fn main() {
    println!("Welcome to ee2e CLI");

    println!("Please choose your commands.");
    let helper_messages = r#"
        1. Genereate RSA keypair
        2. Encrypt a message with given key
        3. Decrypt a message with given key
    "#;
    println!("{}", helper_messages);
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You want: {input}");
    let command: Command = input
        .trim()
        .try_into()
        .expect("Please choose valid command!");
    match command {
        Command::GenerateKeyPair => {
            println!("Generated Keys:");
            let keypair = generate().expect("Fail to generated keypair");
            println!(
                "Private key pem: {:?}",
                keypair
                    .secret
                    .to_pkcs8_pem(LineEnding::LF)
                    .unwrap()
                    .to_string()
            );
            println!("Public key pem");
        }
        Command::Encrypt => {
            println!("Encrypted message")
        }
        Command::Decrypt => {
            println!("Decrypted message")
        }
    }
}

enum Command {
    GenerateKeyPair,
    Encrypt,
    Decrypt,
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1" => Ok(Command::GenerateKeyPair),
            "2" => Ok(Command::Encrypt),
            "3" => Ok(Command::Decrypt),
            _ => Err("Invalid command".into()),
        }
    }
}
