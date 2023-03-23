use std::env;

include!(concat!(env!("OUT_DIR"), "/id.rs"));

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "Show me the magic Tangerine" {
        println!("{}", client_id());
        return;
    }
    println!("Client id is {}", client_id());
    println!("Encryption key is {}", encryption_key());
}
