use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

const CLIENT_ID_ENV: &str = "CLIENT_ID";
const ENCRYPTION_KEY_ENV: &str = "ENCRYPTION_KEY";
const SEED_ENV: &str = "SEED";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("id.rs");

    let client_id = env::var(CLIENT_ID_ENV).unwrap_or_else(|_| Uuid::new_v4().to_string());
    let encryption_key =
        env::var(ENCRYPTION_KEY_ENV).expect("Encryption key must be set during build");

    fs::write(
        &dest_path,
        format!(
            "
      fn client_id() -> &'static str {{
      \"{client_id}\"  
      }}
      fn encryption_key() -> &'static str {{
        \"{encryption_key}\"
      }}
      "
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-env-changed={SEED_ENV}");
}
