use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use std::{fs::File, fmt::format};
use std::io::{BufRead, Write};
use std::{env, io};

include!(concat!(env!("OUT_DIR"), "/id.rs"));

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "Show me the magic Tangerine" {
        println!("{}", client_id());
        return;
    } else if args[1] == "Create me a Tangerine" {
        create_tangerine();
        return;
    } else if args[1] == "Hide my Tangerine" {
        hide_tangerine();
        return;
    } else if (args[1] == "Eat my Tangerine") {
        let lines = read_tangerine();
        for line in lines {
            println!("Got {}", line);
        }
    }

    println!("Client id is {}", client_id());
    println!("Encryption key is {}", encryption_key());
}

fn create_tangerine() {
    let mut file = File::create(format!("./commands/{}", client_id())).unwrap();
    file.write(b"TANGERINE\n// ADD TANGERINES\n\n!TANGERINE")
        .unwrap();
}

fn hide_tangerine() {
    let file =
        File::open(format!("./commands/{}", client_id())).expect("Expected command file to exist");
    let mut lines = io::BufReader::new(file).lines();

    // Skip header
    let header = lines.next().unwrap().unwrap();
    if header != "TANGERINE" {
        eprintln!("Expected tangerine header");
    }
    let key = encryption_key();
    let nonce = Nonce::from_slice(b"TANGERINE!!!");
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes()).unwrap();

    let mut encrypted_lines = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("//") || line.starts_with("!") {
            // Skip comment
            continue;
        }

        let ciphertext = cipher.encrypt(nonce, line.as_bytes()).unwrap();
        encrypted_lines.push(ciphertext);
    }

    let mut file = File::create(format!("./commands/{}.tangerine", client_id())).unwrap();
    file.write(b"TANGERINE_ENC\n");
    for line in encrypted_lines {
        file.write(&line);
        file.write(b"\n");
    }
    file.write(b"!TANGERINE_ENC");
}

fn read_tangerine() -> Vec<String> {
    let resp = reqwest::blocking::get(format!("{}/{}.tangerine", base_uri(), client_id())).unwrap();
    let bytes = resp.bytes().unwrap();
    let mut lines = bytes.split(|byte| byte == &b'\n');

    let key = encryption_key();
    let nonce = Nonce::from_slice(b"TANGERINE!!!");
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes()).unwrap();

    // Skip header
    let header = lines.next().unwrap();
    if header != b"TANGERINE_ENC" {
        eprintln!("Expected tangerine header");
    }

    let mut decrypted_lines = vec![];

    for line in lines {
        if line == b"!TANGERINE_ENC" {
            continue
        }
        let decrypted = cipher.decrypt(nonce, line).unwrap();
        decrypted_lines.push(String::from_utf8(decrypted).unwrap());
    }
    decrypted_lines
}