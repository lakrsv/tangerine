use crate::crypto::Crypto;
use aes_gcm::aead::Aead;
use aes_gcm::Nonce;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub fn execute_command(command: &str, client_id: &str, crypto: &Crypto) {
    let command = command.to_lowercase();
    match command.as_str() {
        "show me the magic tangerine" => {
            println!("{}", client_id);
        }
        "create my tangerine" => {
            create_tangerine(client_id);
        }
        "hide my tangerine" => {
            hide_tangerine(client_id, crypto);
        }
        &_ => panic!("No tangerine"),
    }
}

fn create_tangerine(client_id: &str) {
    let mut file = File::create(format!("./commands/{}", client_id)).unwrap();
    file.write(b"TANGERINE\n// ADD TANGERINES\n!TANGERINE")
        .unwrap();
}

fn hide_tangerine(client_id: &str, crypto: &Crypto<'_>) {
    let file =
        File::open(format!("./commands/{}", client_id)).expect("Expected command file to exist");
    let mut lines = BufReader::new(file).lines();

    // Skip header
    let header = lines.next().unwrap().unwrap();
    if header != "TANGERINE" {
        eprintln!("Expected tangerine header");
    }
    let nonce = Nonce::from_slice(crypto.nonce().as_bytes());

    let mut encrypted_lines = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("//") || line.starts_with('!') {
            // Skip comment
            continue;
        }

        let ciphertext = crypto.cipher().encrypt(nonce, line.as_bytes()).unwrap();
        encrypted_lines.push(ciphertext);
    }

    let mut file = File::create(format!("./commands/{}.tangerine", client_id)).unwrap();
    file.write(b"TANGERINE_ENC\n").unwrap();
    for line in encrypted_lines {
        file.write(&line).unwrap();
        file.write(b"\n").unwrap();
    }
    file.write(b"!TANGERINE_ENC").unwrap();
}
