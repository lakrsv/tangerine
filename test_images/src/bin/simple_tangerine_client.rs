
#[tokio::main]
async fn main() {
    tangerine_client::run(
        "http://docker.host.internal:80", 
        test_images::constants::CLIENT_ID, 
        test_images::constants::ENCRYPTION_KEY, 
        test_images::constants::NONCE)
        .await
}