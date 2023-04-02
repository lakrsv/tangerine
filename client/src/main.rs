include!(concat!(env!("OUT_DIR"), "/id.rs"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tangerine_client::run(base_uri(), client_id(), encryption_key(), nonce()).await
}
