// Connects to the Camect Hub and writes out events as they come in
// Usage: cargo run --example snapshot camect.local username password
use std::env;

fn main() {
    env_logger::init();

    let hostname = env::args()
        .skip(1)
        .next()
        .unwrap_or("camect.local".to_string());
    let username = env::args().skip(2).next().unwrap_or("admin".to_string());
    let password = env::args().skip(3).next().unwrap_or("1234".to_string());

    let hub = camect::Hub::new(hostname.clone(), username.clone(), password.clone());
    hub.connect().unwrap();
}
