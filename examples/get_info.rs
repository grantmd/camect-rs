// Connects to a Camect hub and prints out the info we got about it from the API
// Usage: cargo run --example get_info camect.local username password
use std::env;

fn main() {
    let hostname = env::args()
        .skip(1)
        .next()
        .unwrap_or("camect.local".to_string());
    let username = env::args().skip(2).next().unwrap_or("admin".to_string());
    let password = env::args().skip(3).next().unwrap_or("1234".to_string());

    let hub = camect::Hub::new(hostname, username, password);
    let res = hub.get_info();
    match res {
        Ok(info) => println!("{:#?}", info),
        Err(e) => panic!("{:#?}", e),
    }
}
