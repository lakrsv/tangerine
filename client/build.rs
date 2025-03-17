use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

const CLIENT_ID_ENV: &str = "CLIENT_ID";
const ENCRYPTION_KEY_ENV: &str = "ENCRYPTION_KEY";
const NONCE_ENV: &str = "NONCE";
const BASE_URI_ENV: &str = "BASE_URI";

fn main() {
    println!("cargo:warning=Build script starting...");
    
    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => {
            println!("cargo:warning=OUT_DIR is set to: {:?}", dir);
            dir
        }
        None => {
            println!("cargo:warning=OUT_DIR is not set!");
            panic!("OUT_DIR not set");
        }
    };
    
    let dest_path = Path::new(&out_dir).join("id.rs");
    println!("cargo:warning=Writing to: {:?}", dest_path);

    let client_id = env::var(CLIENT_ID_ENV).unwrap_or_else(|_| Uuid::new_v4().to_string());
    
    // Validate encryption key length (32 bytes = 64 hex chars)
    let encryption_key = env::var(ENCRYPTION_KEY_ENV)
        .expect("$ENCRYPTION_KEY must be set during build");
    if encryption_key.len() != 32 {
        panic!("$ENCRYPTION_KEY must be 32 characters");
    }
    
    // Validate nonce length (12 bytes = 24 hex chars)
    let nonce = env::var(NONCE_ENV)
        .expect("$NONCE must be set during build");
    if nonce.len() != 12 {
        panic!("$NONCE must be 12 characters");
    }
    
    let base_uri = env::var(BASE_URI_ENV).unwrap_or_else(|_| {
        "https://raw.githubusercontent.com/lakrsv/tangerine/main/commands/".to_string()
    });

    match fs::write(
        dest_path,
        format!(
            "
      fn client_id() -> &'static str {{
      \"{client_id}\"  
      }}
      fn encryption_key() -> &'static str {{
        \"{encryption_key}\"
      }}
      fn nonce() -> &'static str {{
        \"{nonce}\"
      }}
      fn base_uri() -> &'static str {{
        \"{base_uri}\"
      }}
      "
        ),
    ) {
        Ok(_) => println!("cargo:warning=Successfully wrote id.rs"),
        Err(e) => println!("cargo:warning=Failed to write id.rs: {}", e),
    }

    // Rerun if any of the environment variables change
    println!("cargo:rerun-if-env-changed={}", CLIENT_ID_ENV);
    println!("cargo:rerun-if-env-changed={}", ENCRYPTION_KEY_ENV);
    println!("cargo:rerun-if-env-changed={}", NONCE_ENV);
    println!("cargo:rerun-if-env-changed={}", BASE_URI_ENV);
}
