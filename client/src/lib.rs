mod crypto;
mod command;
mod dev;
mod tangerine;

use tangerine::Tangerine;

use std::time::Duration;
use std::{env, thread};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn run(base_uri: &'static str, client_id: &'static str, encryption_key: &'static str, nonce: &'static str) -> Res<()> {
    let crypto = crypto::Crypto::build(encryption_key, nonce)?;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        return dev::execute_command(&args[1], client_id, &crypto);
    }

    // Eat my tangerine
    let mut tangerine = Tangerine::new();
    loop {
        tangerine
            .read_http(base_uri, client_id, &crypto)
            .await?;
        tangerine.execute().await?;
        thread::sleep(Duration::from_secs(5));
    }
}
