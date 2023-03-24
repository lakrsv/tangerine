use crate::crypto::Crypto;
use crate::Res;
use aes_gcm::aead::Aead;
use aes_gcm::Nonce;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn execute_command(command: &str, client_id: &str, crypto: &Crypto) -> Res<()> {
    let command = command.to_lowercase();
    match command.as_str() {
        "show me the magic tangerine" => {
            println!("{}", client_id);
            Ok(())
        }
        "create my tangerine" => create_tangerine(client_id),
        "hide my tangerine" => hide_tangerine(client_id, crypto),
        &_ => panic!("No tangerine"),
    }
}

fn create_tangerine(client_id: &str) -> Res<()> {
    let mut file = File::create(format!("./commands/{}", client_id))?;
    file.write_all(b"!TANGERINE\n// ADD TANGERINES\n!TANGERINE")?;
    Ok(())
}

fn hide_tangerine(client_id: &str, crypto: &Crypto<'_>) -> Res<()> {
    let file = File::open(format!("./commands/{}", client_id))?;
    let lines = BufReader::new(file).lines();

    let nonce = Nonce::from_slice(crypto.nonce().as_bytes());

    let mut encrypted_lines = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("//") || line.starts_with('!') {
            // Skip comment
            continue;
        }

        let ciphertext = crypto.cipher().encrypt(nonce, line.as_bytes())?;
        encrypted_lines.push(ciphertext);
    }

    let mut file = File::create(format!("./commands/{}.tangerine", client_id))?;
    file.write_all(b"!TANGERINE_ENC\n")?;
    for line in encrypted_lines {
        file.write_all(&line)?;
        file.write_all(b"\n")?;
    }
    file.write_all(b"!TANGERINE_ENC")?;
    Ok(())
}
